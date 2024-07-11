pub(crate) mod commands;
pub(crate) mod simulation_result;

use std::fmt::{self, Debug};
use std::sync::Arc;

use itertools::Itertools;
use log::{info, trace};
use miette::Diagnostic;
use rayon::prelude::*;
use thiserror::Error;

use crate::backends::{Backend, BackendError};
use crate::consts::{DIO_GUESS, MAXITER, VECTOL};
use crate::models::{Pairs, Element, Triples, Variable};
use crate::Simulation;
use commands::SimulationCommand;
use simulation_result::Sim;
use simulation_result::SimulationResults;

#[derive(Debug, Error, Diagnostic)]
pub(crate) enum SimulatorError {
    #[error("This Operation is not implemented")]
    #[diagnostic(help("Try helping by implementing this operation!"))]
    Unimplemented,

    #[error("{0}")]
    BackendError(BackendError),

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

impl From<BackendError> for SimulatorError {
    fn from(error: BackendError) -> Self {
        SimulatorError::BackendError(error)
    }
}

/// A simulator for circuit analysis.
///
/// This struct represents a simulator used for analyzing electrical circuits.
/// It contains the circuit elements, simulation commands, variables, backend,
/// and results of the simulation.
pub(super) struct Simulator<BE: Backend> {
    /// The elements in the circuit.
    elements: Vec<Element>,
    /// The simulation commands to be executed.
    commands: Vec<SimulationCommand>,
    /// The variables used in the simulation.
    vars: Vec<Variable>,
    /// The backend used for solving the circuit equations.
    backend: BE,
}

impl<BE: Backend> Simulator<BE> {
    /// Executes the simulation commands and returns the simulation results.
    ///
    /// This method iterates through the simulation commands, executes each one, and collects
    /// the results. If an error occurs during execution, it returns an error.
    pub fn run(&mut self) -> Result<SimulationResults, SimulatorError> {
        let commands = self.commands.clone();
        let mut results = SimulationResults(Vec::new());
        for com in commands {
            let error = self.execute_command(&com);
            match error {
                Ok(res) => results.0.push(res),
                Err(err) => return Err(err),
            }
        }

        Ok(results)
    }

    /// Executes a single simulation command.
    ///
    /// This method executes the given simulation command by calling the corresponding
    /// method for that command. If an error occurs during execution, it returns an error.
    fn execute_command(&mut self, comm: &SimulationCommand) -> Result<Sim, SimulatorError> {
        let res = match comm {
            SimulationCommand::Op => self.run_op()?,
            SimulationCommand::Tran => self.run_tran()?,
            SimulationCommand::Ac => self.run_ac()?,
            SimulationCommand::Dc(vs, vstart, vstop, vstep, _) => {
                self.run_dc(vs, vstart, vstop, vstep)?
            }
        };
        Ok(res)
    }

    /// Executes an operating point analysis.
    ///
    /// This method performs an operating point analysis by building the constant matrices,
    /// transferring them to the backend, solving the equations, and collecting the results.
    fn run_op(&mut self) -> Result<Sim, SimulatorError> {
        info!("Run operating point analysis");
        // Check for nonlinear elements
        let const_a_mat = self.build_constant_a_mat()?;
        let const_b_vec = self.build_constant_b_vec()?;
        if !self.has_nonlinear_elements() {
            // Build the constant matrix
            self.backend.set_a(&const_a_mat);
            self.backend.set_b(&const_b_vec);
            let x_vec = self.backend.solve()?.clone();
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
                let a_mat = self.build_nonlinear_a_mat(&x) + const_a_mat.clone();
                let b_vec = self.build_nonlinear_b_vec(&x) + const_b_vec.clone();

                trace!("Set matrix");
                // Populate matrices
                self.backend.set_a(&a_mat);
                self.backend.set_b(&b_vec);

                trace!("Solve matrix");
                // Solve for the new x
                let x_new = match self.backend.solve().cloned() {
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

    /// Checks if the circuit contains any nonlinear elements.
    fn has_nonlinear_elements(&self) -> bool {
        self.elements.iter().any(|element| element.is_nonlinear())
    }

    /// Adds variable names to the solution vector.
    ///
    /// This method takes a solution vector and adds variable names to each value,
    /// based on the order of variables stored in the `vars` field of the `Simulator`.
    /// It returns a vector of tuples, where each tuple contains a variable name and its corresponding value.
    fn add_var_name(&self, solution: Vec<f64>) -> Vec<(Variable, f64)> {
        let sol = solution
            .into_iter()
            .enumerate()
            .map(|(idx, var)| (self.vars[idx].clone(), var))
            .collect_vec();
        sol
    }

    /// Executes a transient analysis.
    ///
    /// This method performs a transient analysis by building the constant, time variant,
    /// and nonlinear matrices and vectors. It does not currently perform any calculations
    /// but returns `Err(SimulatorError::Unimplemented)` to indicate NYI.
    fn run_tran(&mut self) -> Result<Sim, SimulatorError> {
        let _ = self.build_time_variant_a_mat();
        let _ = self.build_time_variant_b_vec();

        Err(SimulatorError::Unimplemented)
    }

    /// Executes an AC analysis.
    ///
    /// This method performs an AC analysis. It does not currently perform any calculations
    /// but returns `Err(SimulatorError::Unimplemented)` to indicate NYI.
    fn run_ac(&mut self) -> Result<Sim, SimulatorError> {
        Err(SimulatorError::Unimplemented)
    }

    /// Executes a DC analysis.
    ///
    /// This method performs a DC analysis.
    fn run_dc(
        &mut self,
        srcnam: &Arc<String>,
        vstart: &f64,
        vstop: &f64,
        vstep: &f64,
    ) -> Result<Sim, SimulatorError> {
        let mut voltage_source_index = None;
        for (index, element) in self.elements.iter().enumerate() {
            if let Element::VSource(ref vs) = element {
                if vs.name() == *srcnam {
                    voltage_source_index = Some(index);
                    break;
                }
            }
        }

        let voltage_source_index = match voltage_source_index {
            Some(index) => index,
            None => return Err(SimulatorError::VoltageSourceNotFound(srcnam.to_string())),
        };

        let mut dc_results = Vec::new();

        let voltage_0;
        {
            let source = match &mut self.elements[voltage_source_index] {
                Element::VSource(ref mut vs) => vs,
                _ => unreachable!(),
            };
            voltage_0 = source.value();
        }

        // Iterate over the voltage range
        let mut voltage = *vstart;
        while voltage <= *vstop {
            {
                // Set the voltage source to the current value
                let source = match &mut self.elements[voltage_source_index] {
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
            let source = match &mut self.elements[voltage_source_index] {
                Element::VSource(ref mut vs) => vs,
                _ => unreachable!(),
            };
            source.set_voltage(voltage_0);
        }

        Ok(Sim::Dc(dc_results))
    }

    /// Executes a simgle operating point analysis for dc analysis
    ///
    /// This method performs an operating point analysis by building the constant matrices,
    /// transferring them to the backend, solving the equations, and collecting the results.
    fn find_op(&mut self) -> Result<Vec<(Variable, f64)>, SimulatorError> {
        // Check for nonlinear elements
        let const_a_mat = self.build_constant_a_mat()?;
        let const_b_vec = self.build_constant_b_vec()?;

        if !self.has_nonlinear_elements() {
            // Build the constant matrix
            self.backend.set_a(&const_a_mat);
            self.backend.set_b(&const_b_vec);
            let x_vec = self.backend.solve()?.clone();
            let res = self.add_var_name(x_vec);
            return Ok(res);
        }

        // Build the initial guess
        let mut x = self.generate_initial_guess();
        for _ in 0..MAXITER {
            let a_mat = self.build_nonlinear_a_mat(&x) + const_a_mat.clone();
            let b_vec = self.build_nonlinear_b_vec(&x) + const_b_vec.clone();

            // Populate matrices
            self.backend.set_a(&a_mat);
            self.backend.set_b(&b_vec);

            // Solve for the new x
            let x_new = self.backend.solve()?.clone();

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

    /// Builds a constant matrix 'A' from the elements.
    ///
    /// This method iterates over the elements and collects their constant triples.
    /// If any elements provide constant triples, they are summed together.
    ///
    /// # Returns
    ///
    /// * `Ok(Triples)` - The combined constant triples.
    /// * `Err(SimulatorError::ConstantMatrixEmpty)` - If no constant triples are found.
    fn build_constant_a_mat(&self) -> Result<Triples, SimulatorError> {
        let const_a_mat = self
            .elements
            .par_iter()
            .filter_map(|ele| ele.get_constant_triples())
            .reduce(|| Triples::Empty, |acc, ele| acc + ele);
        if const_a_mat == Triples::Empty {
            Err(SimulatorError::ConstantMatrixEmpty)
        } else {
            Ok(const_a_mat)
        }
    }

    /// Builds a constant vector 'b' from the elements.
    ///
    /// This method iterates over the elements and collects their constant pairs.
    /// If any elements provide constant pairs, they are summed together.
    ///
    /// # Returns
    ///
    /// * `Ok(pairs)` - The combined constant pairs.
    /// * `Err(SimulatorError::ConstantVectorEmpty)` - If no constant pairs are found.
    fn build_constant_b_vec(&self) -> Result<Pairs, SimulatorError> {
        let const_b_vec = self
            .elements
            .par_iter()
            .filter_map(|ele| ele.get_constant_pairs())
            .reduce(|| Pairs::Empty, |acc, ele| acc + ele);
        if const_b_vec == Pairs::Empty {
            Err(SimulatorError::ConstantVectorEmpty)
        } else {
            Ok(const_b_vec)
        }
    }

    /// Builds a time-variant matrix 'A' from the elements.
    ///
    /// This method iterates over the elements and collects their time-variant triples.
    /// If any elements provide time-variant triples, they are summed together.
    ///
    /// # Returns
    ///
    /// * `Triples` - The combined time-variant triples, or an empty `Triples` if none are found.
    fn build_time_variant_a_mat(&self) -> Triples {
        self.elements
            .par_iter()
            .filter_map(|ele| ele.get_time_variant_triples())
            .reduce(|| Triples::Empty, |acc, ele| acc + ele)
    }

    /// Builds a time-variant vector 'B' from the elements.
    ///
    /// This method iterates over the elements and collects their time-variant pairs.
    /// If any elements provide time-variant pairs, they are summed together.
    ///
    /// # Returns
    ///
    /// * `pairs` - The combined time-variant pairs, or an empty `pairs` if none are found.
    fn build_time_variant_b_vec(&self) -> Pairs {
        self.elements
            .par_iter()
            .filter_map(|ele| ele.get_time_variant_pairs())
            .reduce(|| Pairs::Empty, |acc, ele| acc + ele)
    }

    /// Builds a nonlinear matrix 'A' from the elements, based on a given vector `x_vec`.
    ///
    /// This method iterates over the elements and collects their nonlinear triples
    /// using the provided `x_vec`. If any elements provide nonlinear triples,
    /// they are summed together.
    ///
    /// # Parameters
    ///
    /// * `x_vec` - A reference to a vector of floating-point numbers used in the nonlinear calculation.
    ///
    /// # Returns
    ///
    /// * `Triples` - The combined nonlinear triples, or an empty `Triples` if none are found.
    fn build_nonlinear_a_mat(&self, x_vec: &Vec<f64>) -> Triples {
        self.elements
            .par_iter()
            .filter_map(|ele| ele.get_nonlinear_triples(x_vec))
            .reduce(|| Triples::Empty, |acc, ele| acc + ele)
    }

    /// Builds a nonlinear vector 'B' from the elements.
    ///
    /// This method iterates over the elements and collects their nonlinear pairs.
    /// If any elements provide nonlinear pairs, they are summed together.
    ///
    /// # Returns
    ///
    /// * `pairs` - The combined nonlinear pairs, or an empty `pairs` if none are found.
    fn build_nonlinear_b_vec(&self, x_vec: &Vec<f64>) -> Pairs {
        self.elements
            .par_iter()
            .filter_map(|ele| ele.get_nonlinear_pairs(x_vec))
            .reduce(|| Pairs::Empty, |acc, ele| acc + ele)
    }

    /// Generates an initial guess for the node voltages in the circuit.
    ///
    /// This method iterates over all elements in the circuit and sets the initial
    /// guess for node voltages based on the connected voltage sources and diodes.
    ///
    /// # Returns
    ///
    /// A vector containing the initial guess for the node voltages.
    fn generate_initial_guess(&self) -> Vec<f64> {
        let len = self.vars.len();
        self.elements
            .par_iter()
            .map(|element| {
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
                            (None, Some(c_idx)) => local_guess[c_idx] = -DIO_GUESS / 2.0,
                            (Some(a_idx), None) => local_guess[a_idx] = DIO_GUESS,
                            (Some(a_idx), Some(c_idx)) => {
                                local_guess[a_idx] = DIO_GUESS / 2.0;
                                local_guess[c_idx] = -DIO_GUESS / 2.0;
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        // Handle other elements if needed
                    }
                }

                local_guess
            })
            .reduce(
                || vec![0.0; len],
                |mut acc, local_guess| {
                    for (i, &val) in local_guess.iter().enumerate() {
                        acc[i] += val;
                    }
                    acc
                },
            )
    }

    /// Checks if two vectors have converged within a given tolerance.
    ///
    /// This method compares the old and new vectors element-wise to determine if
    /// the difference is within the specified tolerance.
    ///
    /// # Arguments
    ///
    /// * `x_old` - The old vector of voltages.
    /// * `x_new` - The new vector of voltages.
    /// * `tolerance` - The convergence tolerance.
    ///
    /// # Returns
    ///
    /// `true` if the vectors have converged, otherwise `false`.
    fn has_converged(&self, x_old: &Vec<f64>, x_new: &Vec<f64>, tolerance: f64) -> bool {
        x_old
            .par_iter()
            .zip(x_new.par_iter())
            .all(|(&old, &new)| (old - new).abs() < tolerance)
    }
}

/// Implements the `Debug` trait for the `Simulator` struct, which uses the specified backend.
/// This trait allows the `Simulator` to be formatted using the `{:?}` and `{:#?}` format specifiers.
///
/// # Example
///
/// ```
/// let sim = Simulator::<NalgebraBackend>::from(simulation);
/// println!("{:?}", sim);
/// ```
impl<BE: Backend> Debug for Simulator<BE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Simulator").finish()
    }
}

/// Implements the `From` trait to create a `Simulator` instance from a `Simulation`.
/// This trait allows for easy conversion between `Simulation` and `Simulator`.
///
/// # Parameters
///
/// - `sim`: The `Simulation` instance to be converted into a `Simulator`.
///
/// # Returns
///
/// A new `Simulator` instance initialized with the given `Simulation` parameters.
///
/// # Example
///
/// ```
/// let simulation = Simulation {
///     variables: vec!["1".into(), "v1#branch".into()],
///     elements: vec![Element::Resistor(res), Element::VSource(vol)],
///     commands: vec![SimulationCommand::Op],
/// };
/// let simulator: Simulator<NalgebraBackend> = Simulator::from(simulation);
/// ```
impl<BE: Backend> From<Simulation> for Simulator<BE> {
    fn from(sim: Simulation) -> Self {
        let Simulation {
            variables,
            elements,
            commands,
        } = sim;

        let backend = BE::new(variables.len()).unwrap();

        Simulator {
            elements,
            commands,
            backend,
            vars: variables,
        }
    }
}

#[cfg(test)]
mod tests;
