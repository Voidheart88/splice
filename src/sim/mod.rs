pub(crate) mod commands;
pub(crate) mod options;
pub(crate) mod simulation_result;

mod ac;
mod dc;
mod op;
mod tran;

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

            match element {
                Element::VSource(vsource) => {
                    let value = vsource.value();
                    if let Some(node0_idx) = vsource.node0_idx() {
                        local_guess[node0_idx] = -value;
                    }
                    if let Some(node1_idx) = vsource.node1_idx() {
                        local_guess[node1_idx] = value;
                    }
                }
                Element::Diode(diode) => {
                    let a_idx = diode.a_idx();
                    let c_idx = diode.c_idx();

                    match (a_idx, c_idx) {
                        (None, Some(c_idx)) => {
                            local_guess[c_idx] = -DIO_GUESS / (Numeric::one() + Numeric::one())
                        }
                        (Some(a_idx), None) => local_guess[a_idx] = DIO_GUESS,
                        (Some(a_idx), Some(c_idx)) => {
                            local_guess[a_idx] = DIO_GUESS / (Numeric::one() + Numeric::one());
                            local_guess[c_idx] = -DIO_GUESS / (Numeric::one() + Numeric::one());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            for (i, &val) in local_guess.iter().enumerate() {
                acc[i] += val;
            }
        }

        acc
    }

    fn has_converged(&self, x_old: &[Numeric], x_new: &[Numeric], tolerance: Numeric) -> bool {
        x_old
            .iter()
            .zip(x_new.iter())
            .all(|(&old, &new)| (old - new).abs() < tolerance)
    }

    /// Updates capacitor voltages after each time step for proper transient simulation
    /// This is crucial for correct integration of capacitor behavior
    fn update_capacitor_voltages(&mut self, x_vec: &[Numeric]) {
        for element in &mut self.elements {
            if let Element::Capacitor(cap) = element {
                // Calculate voltage across capacitor: V = V(node0) - V(node1)
                let v_node0 = cap.node0.as_ref().map(|n| x_vec[n.idx()]).unwrap_or(Numeric::zero());
                let v_node1 = cap.node1.as_ref().map(|n| x_vec[n.idx()]).unwrap_or(Numeric::zero());
                let voltage_across = v_node0 - v_node1;
                
                // Always update with the actual voltage across the capacitor
                // This works for both OP initialization and transient steps
                cap.update_previous_voltage(voltage_across);
            }
        }
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
