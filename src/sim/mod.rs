pub(crate) mod commands;
pub(crate) mod options;
pub(crate) mod simulation_result;

mod ac;
pub mod autotune;
mod dc;
mod op;
mod tran;

use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::sync::Arc;

use itertools::{izip, Itertools};
use miette::Diagnostic;
use num::{Complex, One, Zero};
use options::SimulationOption;
use thiserror::Error;

use crate::models::{Element, Variable};
use crate::sim::ac::AcSimulation;
use crate::sim::dc::DcSimulation;
use crate::sim::op::OpSimulation;
use crate::sim::options::IntegrationMethod;
use crate::sim::tran::TranSimulation;
use crate::solver::{Solver, SolverError};
use crate::spot::*;
use crate::Simulation;
use commands::SimulationCommand;
use simulation_result::Sim;
use simulation_result::SimulationResults;

#[derive(Debug, Error, Diagnostic)]
pub(crate) enum SimulatorError {
    #[error("{0}")]
    BackendError(SolverError),

    #[error("The Simulation did not converge after {max_iter} steps (VECTOL={tol})")]
    #[diagnostic(help("Try increasing VECTOL (current: {tol}) or check for unstable elements"))]
    NonConvergentMaxIter { max_iter: usize, tol: Numeric },

    #[error("Voltage source {0} not found")]
    #[diagnostic(help("Check the source in your .dc command"))]
    VoltageSourceNotFound(String),

    #[error("{0}")]
    #[diagnostic(help("Check your circuit for coupling errors"))]
    CircuitError(String),
}

impl From<SolverError> for SimulatorError {
    fn from(error: SolverError) -> Self {
        SimulatorError::BackendError(error)
    }
}

pub(super) struct Simulator<SO: Solver> {
    /// The elements in the circuit.
    elements: Vec<Element>,
    /// The simulation commands to be executed.
    commands: Vec<SimulationCommand>,
    /// The simulation options for the backend.
    options: Vec<SimulationOption>,
    /// The variables used in the simulation.
    vars: Vec<Variable>,
    /// The backend used for solving the circuit equations.
    solver: SO,
}

impl<SO: Solver> Simulator<SO> {
    /// Returns the integration method to use for transient simulation
    /// Defaults to BackwardEuler for stability
    fn get_integration_method(&self) -> IntegrationMethod {
        self.options
            .iter()
            .find_map(|opt| opt.get_integration_method())
            .unwrap_or(IntegrationMethod::BackwardEuler)
    }

    pub fn run(&mut self) -> Result<SimulationResults, SimulatorError> {
        //Inits matrices and sparsity patterns
        self.init_solver();

        let commands = self.commands.clone();
        let mut results = SimulationResults {
            options: self.options.clone(),
            ..Default::default()
        };
        for com in commands {
            let error = self.execute_command(&com);
            match error {
                Ok(res) => results.results.push(res),
                Err(err) => return Err(err),
            }
        }

        Ok(results)
    }

    fn init_solver(&mut self) {
        let a_mat: Vec<(usize, usize)> = self
            .elements
            .iter()
            .filter_map(|ele| ele.get_triple_indices())
            .flat_map(|ele| ele.data())
            .collect();

        let cplx_a_mat: Vec<(usize, usize)> = self
            .elements
            .iter()
            .filter_map(|ele| ele.get_cplx_triple_indices())
            .flat_map(|ele| ele.data())
            .collect();

        self.solver.init(a_mat, cplx_a_mat);
    }

    fn execute_command(&mut self, comm: &SimulationCommand) -> Result<Sim, SimulatorError> {
        self.solver.reset();
        let res = match comm {
            SimulationCommand::Op => self.run_op()?,
            SimulationCommand::Tran(tstep, tstop) => self.run_tran(tstep, tstop)?,
            SimulationCommand::Ac(fstart, fend, steps, options) => {
                self.run_ac(fstart, fend, steps, options)?
            }
            SimulationCommand::Dc(vs, vstart, vstop, vstep, optional) => {
                self.run_dc(vs, vstart, vstop, vstep, optional)?
            }
        };
        Ok(res)
    }

    fn has_nonlinear_elements(&self) -> bool {
        self.elements.iter().any(|element| element.is_nonlinear())
    }

    fn add_var_name(&self, solution: Vec<Numeric>) -> Vec<(Variable, Numeric)> {
        izip!(&self.vars, solution)
            .map(|(var, val)| (var.clone(), val))
            .collect_vec()
    }

    fn add_complex_var_name(
        &self,
        solution: Vec<Complex<Numeric>>,
    ) -> Vec<(Variable, Complex<Numeric>)> {
        solution
            .into_iter()
            .enumerate()
            .map(|(idx, var)| (self.vars[idx].clone(), var))
            .collect_vec()
    }

    fn find_op(&mut self) -> Result<Vec<(Variable, Numeric)>, SimulatorError> {
        self.solver.reset();
        self.build_constant_a_mat();
        self.build_constant_b_vec();

        if !self.has_nonlinear_elements() {
            let x_vec = self.solver.solve()?.clone();
            let res = self.add_var_name(x_vec);
            return Ok(res);
        }

        let mut x = self.generate_initial_guess();

        for _ in 0..MAXITER {
            self.solver.reset();
            self.build_constant_a_mat();
            self.build_constant_b_vec();
            self.build_nonlinear_a_mat(&x);
            self.build_nonlinear_b_vec(&x);

            let x_new = self.solver.solve()?.clone();

            if self.has_converged(&x, &x_new, VECTOL) {
                let res = self.add_var_name(x_new.clone());
                return Ok(res);
            }

            x = x_new;
        }

        Err(SimulatorError::NonConvergentMaxIter {
            max_iter: MAXITER,
            tol: VECTOL,
        })
    }

    fn build_constant_a_mat(&mut self) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_constant_triples())
            .flat_map(|triples| triples.data())
            .for_each(|triplet| self.solver.insert_a(&triplet));
    }

    fn build_constant_b_vec(&mut self) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_constant_pairs())
            .flat_map(|pairs| pairs.data())
            .for_each(|pair| self.solver.insert_b(&pair));
    }

    fn build_time_variant_a_mat(&mut self, delta_t: &Numeric) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_time_variant_triples(delta_t))
            .flat_map(|triples| triples.data())
            .for_each(|triplet| self.solver.insert_a(&triplet));
    }

    fn build_time_variant_b_vec(&mut self, time: &Numeric, delta_t: &Numeric) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_time_variant_pairs(Some(time), delta_t))
            .flat_map(|pairs| pairs.data())
            .for_each(|pair| self.solver.insert_b(&pair));
    }

    fn build_time_variant_b_vec_trapezoidal(&mut self, time: &Numeric, delta_t: &Numeric) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_time_variant_pairs_trapezoidal(Some(time), delta_t))
            .flat_map(|pairs| pairs.data())
            .for_each(|pair| self.solver.insert_b(&pair));
    }

    fn build_nonlinear_a_mat(&mut self, x_vec: &[Numeric]) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_nonlinear_triples(x_vec))
            .flat_map(|triples| triples.data())
            .for_each(|triplet| self.solver.insert_a(&triplet));
    }

    fn build_nonlinear_b_vec(&mut self, x_vec: &[Numeric]) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_nonlinear_pairs(x_vec))
            .flat_map(|pairs| pairs.data())
            .for_each(|pair| self.solver.insert_b(&pair));
    }

    fn build_ac_a_mat(&mut self, freq: Numeric) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_ac_triples(freq))
            .flat_map(|triples| triples.data())
            .for_each(|triplet| self.solver.insert_cplx_a(&triplet));
    }

    fn build_ac_b_vec(&mut self, freq: Numeric) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_ac_pairs(freq))
            .flat_map(|pairs| pairs.data())
            .for_each(|pair| self.solver.insert_cplx_b(&pair));
    }

    fn generate_initial_guess(&self) -> Vec<Numeric> {
        let len = self.vars.len();
        let mut acc = vec![0.0; len];

        for element in &self.elements {
            let mut local_guess = vec![0.0; len];
            self.add_element_guess(element, &mut local_guess);

            for (i, &val) in local_guess.iter().enumerate() {
                acc[i] += val;
            }
        }

        acc
    }

    fn add_element_guess(&self, element: &Element, guess: &mut [Numeric]) {
        match element {
            Element::VSource(vsource) => {
                let value = vsource.value();
                if let Some(node0_idx) = vsource.node0_idx() {
                    guess[node0_idx] = -value;
                }
                if let Some(node1_idx) = vsource.node1_idx() {
                    guess[node1_idx] = value;
                }
            }
            Element::Diode(diode) => {
                let a_idx = diode.a_idx();
                let c_idx = diode.c_idx();

                match (a_idx, c_idx) {
                    (None, Some(c_idx)) => {
                        guess[c_idx] = -DIO_GUESS / (Numeric::one() + Numeric::one())
                    }
                    (Some(a_idx), None) => guess[a_idx] = DIO_GUESS,
                    (Some(a_idx), Some(c_idx)) => {
                        guess[a_idx] = DIO_GUESS / (Numeric::one() + Numeric::one());
                        guess[c_idx] = -DIO_GUESS / (Numeric::one() + Numeric::one());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn has_converged(&self, x_old: &[Numeric], x_new: &[Numeric], tolerance: Numeric) -> bool {
        x_old
            .iter()
            .zip(x_new.iter())
            .all(|(&old, &new)| (old - new).abs() < tolerance)
    }

    /// Adjusts the timestep based on error estimation for adaptive timestep control
    /// Uses a simple error estimation based on the change in solution between time steps
    fn adjust_timestep(
        &self,
        x_prev: &[Numeric],
        x_current: &[Numeric],
        current_timestep: Numeric,
    ) -> Numeric {
        // Calculate the error estimate based on the relative change in the solution
        let error_estimate: Numeric = x_prev
            .iter()
            .zip(x_current.iter())
            .map(|(&prev, &curr)| {
                // Relative error with protection against division by zero
                let denominator = prev.abs().max(curr.abs()).max(1e-12);
                (curr - prev).abs() / denominator
            })
            .sum();

        // Normalize the error estimate by the number of variables
        let normalized_error = error_estimate / x_prev.len().max(1) as Numeric;

        // Calculate the optimal timestep based on the error
        let error_ratio = (ADAPTIVE_TOLERANCE / normalized_error).sqrt();
        let mut new_timestep = current_timestep * error_ratio * ADAPTIVE_SAFETY_FACTOR;

        // Apply growth factor limits to prevent too rapid changes
        let growth_factor = new_timestep / current_timestep;
        if growth_factor > ADAPTIVE_MAX_GROWTH_FACTOR {
            new_timestep = current_timestep * ADAPTIVE_MAX_GROWTH_FACTOR;
        } else if growth_factor < ADAPTIVE_MIN_GROWTH_FACTOR {
            new_timestep = current_timestep * ADAPTIVE_MIN_GROWTH_FACTOR;
        }

        // Clamp the timestep to the allowed range
        new_timestep = new_timestep.clamp(ADAPTIVE_MIN_TIMESTEP, ADAPTIVE_MAX_TIMESTEP);

        new_timestep
    }

    /// Updates capacitor voltages after each time step for proper transient simulation
    /// This is crucial for correct integration of capacitor behavior
    // CHECK: Check if this can be part of the inductor and capacitor elements
    fn update_capacitor_voltages(&mut self, x_vec: &[Numeric]) {
        for element in &mut self.elements {
            if let Element::Capacitor(cap) = element {
                let v_node0 = cap.node0.as_ref().map(|n| x_vec[n.idx()]).unwrap_or(Numeric::zero());
                let v_node1 = cap.node1.as_ref().map(|n| x_vec[n.idx()]).unwrap_or(Numeric::zero());
                let voltage_across = v_node0 - v_node1;
                cap.update_previous_voltage(voltage_across);
            }
        }
    }



    /// Updates regular inductor currents
    fn update_regular_inductor_currents(&mut self, inductor_currents: &HashMap<Arc<str>, Numeric>) {
        for element in &mut self.elements {
            if let Element::Inductor(ind) = element {
                if let Some(&current) = inductor_currents.get(&ind.name) {
                    ind.update_previous_current(current);
                }
            }
        }
    }

    /// Updates coupled inductor currents
    fn update_coupled_inductor_currents(&mut self, inductor_currents: &HashMap<Arc<str>, Numeric>) {
        for element in &mut self.elements {
            if let Element::CoupledInductors(coupled) = element {
                let inductor1_name = coupled.inductor1();
                let inductor2_name = coupled.inductor2();

                if let (Some(&current1), Some(&current2)) = (
                    inductor_currents.get(&inductor1_name),
                    inductor_currents.get(&inductor2_name),
                ) {
                    coupled.update_previous_currents(current1, current2);
                }
            }
        }
    }

    /// This is crucial for correct integration of inductor behavior
    // CHECK: Check if this can be part of the inductor and capacitor elements themselves
    fn update_inductor_currents(&mut self, x_vec: &[Numeric], delta_t: &Numeric) {
        // First, collect all inductor currents
        let mut inductor_currents: HashMap<Arc<str>, Numeric> = HashMap::new();

        // FIXME: This loop nests too deep and should be refactored
        for element in &self.elements {
            if let Element::Inductor(ind) = element {
                let v_node0 = ind.node0.as_ref().map(|n| x_vec[n.idx()]).unwrap_or(Numeric::zero());
                let v_node1 = ind.node1.as_ref().map(|n| x_vec[n.idx()]).unwrap_or(Numeric::zero());
                let voltage_across = v_node0 - v_node1;
                let equivalent_conductance = delta_t / ind.value;
                let current = voltage_across * equivalent_conductance;
                inductor_currents.insert(ind.name.clone(), current);
            }
        }

        // Update inductor currents
        self.update_regular_inductor_currents(&inductor_currents);
        
        // Update coupled inductor currents
        self.update_coupled_inductor_currents(&inductor_currents);
    }
}

impl<SO: Solver> Debug for Simulator<SO> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Simulator").finish()
    }
}

impl<SO: Solver> From<Simulation> for Simulator<SO> {
    fn from(sim: Simulation) -> Self {
        let Simulation {
            commands,
            options,
            elements,
            variables,
        } = sim;

        let backend = SO::new(variables.len())
            .expect("Failed to create solver backend. This indicates a system resource limitation or invalid configuration.");

        Simulator {
            elements,
            commands,
            options,
            solver: backend,
            vars: variables,
        }
    }
}

fn is_vsource_with_name(element: &Element, srcnam: &Arc<str>) -> bool {
    if let Element::VSource(ref vs) = element {
        vs.name() == *srcnam
    } else {
        false
    }
}

fn get_vsource_value(element: &mut Element) -> Option<Numeric> {
    if let Element::VSource(ref mut vs) = element {
        Some(vs.value())
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
