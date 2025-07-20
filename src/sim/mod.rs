pub(crate) mod commands;
pub(crate) mod options;
pub(crate) mod simulation_result;

use std::fmt::{self, Debug};
use std::sync::Arc;

use itertools::Itertools;
use log::{info, trace};
use miette::Diagnostic;
use num::{Complex, One};
use options::SimulationOption;
use thiserror::Error;

use crate::models::{Element, Variable};
use crate::solver::{Solver, SolverError};
use crate::spot::*;
use crate::Simulation;
use commands::{ACMode, SimulationCommand};
use simulation_result::Sim;
use simulation_result::SimulationResults;

#[derive(Debug, Error, Diagnostic)]
pub(crate) enum SimulatorError {
    #[error("This Operation is not implemented")]
    #[diagnostic(help("Try helping by implementing this operation!"))]
    Unimplemented,

    #[error("{0}")]
    BackendError(SolverError),

    #[error("The constant part of the conductance matrix is empty")]
    #[diagnostic(help("This is a severe error! Send your circuit to Github"))]
    ConstantMatrixEmpty,

    #[error("The constant part of the vector is empty")]
    #[diagnostic(help("This is a severe error! Send your circuit to Github"))]
    ConstantVectorEmpty,

    #[error("The Simulation did not converge after MAXITER steps")]
    #[diagnostic(help("Try reducing the convergence settings by increasing VECTOL"))]
    NonConvergentMaxIter,

    #[error("Source {0} not found")]
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
    /// The simulation commands to be executed.
    options: Vec<SimulationOption>,
    /// The variables used in the simulation.
    vars: Vec<Variable>,
    /// The backend used for solving the circuit equations.
    solver: SO,
}

impl<SO: Solver> Simulator<SO> {
    pub fn run(&mut self) -> Result<SimulationResults, SimulatorError> {
        let commands = self.commands.clone();
        let mut results = SimulationResults::default();
        results.options = self.options.clone();
        for com in commands {
            let error = self.execute_command(&com);
            match error {
                Ok(res) => results.results.push(res),
                Err(err) => return Err(err),
            }
        }

        Ok(results)
    }

    fn execute_command(&mut self, comm: &SimulationCommand) -> Result<Sim, SimulatorError> {
        let res = match comm {
            SimulationCommand::Op => self.run_op()?,
            SimulationCommand::Tran => self.run_tran()?,
            SimulationCommand::Ac(fstart, fend, steps, options) => {
                self.run_ac(fstart, fend, steps, options)?
            }
            SimulationCommand::Dc(vs, vstart, vstop, vstep, optional) => {
                self.run_dc(vs, vstart, vstop, vstep, optional)?
            }
        };
        Ok(res)
    }

    fn run_op(&mut self) -> Result<Sim, SimulatorError> {
        info!("Run operating point analysis");

        self.build_constant_a_mat();
        self.build_constant_b_vec();

        if !self.has_nonlinear_elements() {
            self.solver.set_a(&const_a_mat);
            self.solver.set_b(&const_b_vec);
            let x_vec = self.solver.solve()?.clone();
            let res = self.add_var_name(x_vec);
            return Ok(Sim::Op(res));
        }

        // Build the initial guess
        let mut x = self.generate_initial_guess();

        // Use an iterator for the iterations
        let result = (0..MAXITER)
            .into_iter()
            .map(|run| {
                trace!("Iteration: {run}");
                let a_mat = self.build_nonlinear_a_mat()// + const_a_mat.clone();
                let b_vec = self.build_nonlinear_b_vec()// + const_b_vec.clone();

                trace!("Set matrix");
                // Populate matrices
                self.solver.set_a(&a_mat);
                self.solver.set_b(&b_vec);

                trace!("Solve matrix");
                // Solve for the new x
                let x_new = match self.solver.solve().cloned() {
                    Ok(solution) => solution,
                    Err(err) => return Some(Err(err.into())),
                };

                trace!("Check convergence matrix");
                // Check for convergence
                if self.has_converged(&x, &x_new, VECTOL) {
                    // If converged, store the result
                    let res = self.add_var_name(x_new.clone());
                    return Some(Ok(Sim::Op(res)));
                }

                trace!("Update x");
                // Update x for the next iteration
                x = x_new.clone();

                None
            })
            .find_map(|result| result);

        // If not converged after maximum iterations, return an error
        match result {
            Some(Ok(res)) => Ok(res),
            Some(Err(err)) => Err(err),
            None => Err(SimulatorError::NonConvergentMaxIter),
        }
    }

    fn has_nonlinear_elements(&self) -> bool {
        self.elements.iter().any(|element| element.is_nonlinear())
    }

    fn add_var_name(&self, solution: Vec<Numeric>) -> Vec<(Variable, Numeric)> {
        solution
            .into_iter()
            .enumerate()
            .map(|(idx, var)| (self.vars[idx].clone(), var))
            .collect_vec()
    }

    fn add_complex_var_name(&self, solution: Vec<Complex<Numeric>>) -> Vec<(Variable, Complex<Numeric>)> {
        solution
            .into_iter()
            .enumerate()
            .map(|(idx, var)| (self.vars[idx].clone(), var))
            .collect_vec()
    }

    fn run_tran(&mut self) -> Result<Sim, SimulatorError> {
        let _ = self.build_time_variant_a_mat();
        let _ = self.build_time_variant_b_vec();

        Err(SimulatorError::Unimplemented)
    }

    fn run_ac(
        &mut self,
        fstart: &Numeric,
        fend: &Numeric,
        steps: &usize,
        ac_option: &ACMode,
    ) -> Result<Sim, SimulatorError> {
        info!("Run ac analysis");
        info!("Find operating point");
        self.find_op()?;

        //Calculate frequencies in the range from [fstart;fend]
        let freqs: Vec<Numeric> = match ac_option {
            ACMode::Lin => {
                let step_size = (fend - fstart) / (*steps as Numeric);
                (0..=*steps)
                    .map(|i| fstart + i as Numeric * step_size)
                    .collect()
            }
            ACMode::Dec => {
                let log_fstart = fstart.log10();
                let log_fend = fend.log10();
                let step_size = (log_fend - log_fstart) / (*steps as Numeric);
                (0..=*steps)
                    .map(|i| 10f64.powf(log_fstart + i as Numeric * step_size))
                    .collect()
            }
            ACMode::Oct => {
                let oct_fstart = fstart.log2();
                let oct_fend = fend.log2();
                let step_size = (oct_fend - oct_fstart) / (*steps as Numeric);
                (0..=*steps)
                    .map(|i| 2f64.powf(oct_fstart + i as Numeric * step_size))
                    .collect()
            }
        };

        info!("Run analysis");
        
        let mut ac_results = Vec::new();
        for freq in freqs {
            let cplx_a_mat = self.build_ac_a_mat(freq);
            let cplx_b_vec = self.build_ac_b_vec(freq);

            let x_new = match self.solver.solve_cplx().cloned() {
                Ok(solution) => solution,
                Err(err) => return Err(err.into()),
            };

            let x_new = self.add_complex_var_name(x_new);

            ac_results.push((freq, x_new))
        }

        Ok(Sim::Ac(ac_results))
    }

    /// Executes a DC analysis.
    ///
    /// This method performs a DC analysis.
    fn run_dc(
        &mut self,
        srcnam: &Arc<str>,
        vstart: &Numeric,
        vstop: &Numeric,
        vstep: &Numeric,
        _optional: &Option<(Arc<str>, Numeric, Numeric, Numeric)>,
    ) -> Result<Sim, SimulatorError> {
        let vsource1_idx = self
            .elements
            .iter()
            .enumerate()
            .find(|&(_, element)| is_vsource_with_name(element, srcnam))
            .map(|(index, _)| index);

        let vsource1_idx = match vsource1_idx {
            Some(index) => index,
            None => return Err(SimulatorError::VoltageSourceNotFound(srcnam.to_string())),
        };

        // Safe the original voltage for later use
        let voltage_0 = self
            .elements
            .get_mut(vsource1_idx)
            .and_then(get_vsource_value)
            .expect("Element should be a VSource");

        let mut dc_results = Vec::new();
        // Iterate over the voltage range
        let mut voltage = *vstart;

        while voltage <= *vstop {
            {
                // Set the voltage source to the current value
                let source = match &mut self.elements[vsource1_idx] {
                    Element::VSource(ref mut vs) => vs,
                    _ => unreachable!(),
                };
                source.set_voltage(voltage);
            }
            // Perform the operating point analysis
            dc_results.push(self.find_op()?);
            // Increment the voltage
            voltage += vstep;
        }

        {
            // Restore the original voltage
            let source = match &mut self.elements[vsource1_idx] {
                Element::VSource(ref mut vs) => vs,
                _ => unreachable!(),
            };
            source.set_voltage(voltage_0);
        }

        Ok(Sim::Dc(dc_results))
    }

    fn find_op(&mut self) -> Result<Vec<(Variable, Numeric)>, SimulatorError> {
        // Check for nonlinear elements
        self.build_constant_a_mat();
        self.build_constant_b_vec();

        if !self.has_nonlinear_elements() {
            let x_vec = self.solver.solve()?.clone();
            let res = self.add_var_name(x_vec);
            return Ok(res);
        }

        // Build the initial guess
        let mut x = self.generate_initial_guess();
        
        for _ in 0..MAXITER {
            self.build_constant_a_mat();
            self.build_constant_b_vec();
            self.build_nonlinear_a_mat(&x);
            self.build_nonlinear_b_vec(&x);

            // Solve for the new x
            let x_new = self.solver.solve()?.clone();

            // Check for convergence
            if self.has_converged(&x, &x_new, VECTOL) {
                // If converged, store the result
                let res = self.add_var_name(x_new.clone());
                return Ok(res);
            }

            // Update x for the next iteration
            x = x_new;
        }

        // If not converged after maximum iterations, return an error
        Err(SimulatorError::NonConvergentMaxIter)
    }

    fn build_constant_a_mat(&mut self) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_constant_triples())
            .flat_map(|triples| triples.iter().map(|&(row, col, val)| (row, col, val)))
            .for_each(|ele| self.solver.set_a(ele));      
    }

    fn build_constant_b_vec(&mut self) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_constant_pairs())
            .flat_map(|pairs| pairs.iter().map(|&(row, val)| (row, val)))
            .for_each(|ele| self.solver.set_b(ele));  
    }

    fn build_time_variant_a_mat(&mut self) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_time_variant_triples())
            .flat_map(|triples| triples.iter().map(|&(row, col, val)| (row, col, val)))
            .for_each(|ele| self.solver.set_a(ele));  
    }

    fn build_time_variant_b_vec(&mut self){
        self.elements
            .iter()
            .filter_map(|ele| ele.get_time_variant_pairs())
            .flat_map(|pairs| pairs.iter().map(|&(row, val)| (row, val)))
            .for_each(|ele| self.solver.set_b(ele)); 
    }

    fn build_nonlinear_a_mat(&mut self, x_vec: &Vec<Numeric>) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_nonlinear_triples(x_vec))
            .flat_map(|triples| triples.iter().map(|&(row, col, val)| (row, col, val)))
            .for_each(|ele| self.solver.set_a(ele)); 
    }

    fn build_nonlinear_b_vec(&mut self, x_vec: &Vec<Numeric>) {
        self.elements
            .iter()
            .filter_map(|ele| ele.get_nonlinear_pairs(x_vec))
            .flat_map(|pairs| pairs.iter().map(|&(row, val)| (row, val)))
            .for_each(|ele| self.solver.set_b(ele));
    }

    fn generate_initial_guess(&self) -> Vec<Numeric> {
        let len = self.vars.len();
        let mut acc = vec![0.0; len];

        for element in &self.elements {
            let mut local_guess = vec![0.0; len];

            match element {
                Element::VSource(vsource) => {
                    let value = vsource.value();
                    let node0_idx: Option<usize> = vsource.node0_idx();
                    let node1_idx = vsource.node1_idx();

                    if let Some(node0_idx) = node0_idx {
                        local_guess[node0_idx] = -value;
                    }
                    if let Some(node1_idx) = node1_idx {
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
                _ => {
                }
            }

            for (i, &val) in local_guess.iter().enumerate() {
                acc[i] += val;
            }
        }

        acc
    }

    fn has_converged(
        &self,
        x_old: &Vec<Numeric>,
        x_new: &Vec<Numeric>,
        tolerance: Numeric,
    ) -> bool {
        x_old
            .iter()
            .zip(x_new.iter())
            .all(|(&old, &new)| (old - new).abs() < tolerance)
    }

    fn build_ac_a_mat(&mut self, freq: Numeric){
        self.elements
            .iter()
            .filter_map(|ele| ele.get_ac_triples(freq))
            .flat_map(|triples| triples.iter().map(|&(row, col, val)| (row, col, val)))
            .for_each(|ele| self.solver.set_cplx_a(ele)); 
        
    }

    fn build_ac_b_vec(&mut self, freq: Numeric){
        self.elements
            .iter()
            .filter_map(|ele| ele.get_ac_pairs(freq))
            .flat_map(|pairs| pairs.iter().map(|&(row, val)| (row, val)))
            .for_each(|ele| self.solver.set_cplx_b(ele));
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

        let backend = SO::new(variables.len()).unwrap();

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
