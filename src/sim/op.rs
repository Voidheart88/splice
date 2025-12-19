use log::info;
use log::trace;

use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;

/// Trait for operating point (OP) simulation.
/// 
/// The operating point analysis calculates the DC working point of a circuit
/// by solving the nonlinear equations iteratively. This is essential for
/// initializing transient and AC analyses.
pub(super) trait OpSimulation<SO: Solver> {
    /// Runs the operating point analysis.
    ///
    /// # Returns
    /// 
    /// * `Ok(Sim::Op)` - The operating point simulation results with node voltages
    /// * `Err(SimulatorError)` - If the simulation fails to converge or other errors occur
    fn run_op(&mut self) -> Result<Sim, SimulatorError>;
}

impl<SO: Solver> OpSimulation<SO> for Simulator<SO> {
    fn run_op(&mut self) -> Result<Sim, SimulatorError> {
        info!("Run operating point analysis");

        // Fast path for linear circuits - solve directly without iteration
        if !self.has_nonlinear_elements() {
            trace!("Linear circuit detected - using direct solve");
            self.build_constant_a_mat();
            self.build_constant_b_vec();
            let x_vec = self.solver.solve()?.clone();
            let res = self.add_var_name(x_vec);
            return Ok(Sim::Op(res));
        }

        // Build the initial guess for nonlinear circuits
        trace!("Nonlinear circuit detected - using iterative Newton-Raphson");
        let mut x = self.generate_initial_guess();

        // Use an iterator for the iterations with maximum iteration limit
        let result = (0..MAXITER)
            .map(|run| {
                trace!("Iteration: {run}");
                trace!("Building conductance matrix (A) and source vector (B)");
                
                // Build the linear and nonlinear parts of the circuit equations
                self.build_constant_a_mat();  // Linear elements (resistors, etc.)
                self.build_constant_b_vec();  // Constant sources
                self.build_nonlinear_a_mat(&x);  // Nonlinear element conductances
                self.build_nonlinear_b_vec(&x);  // Nonlinear element contributions
                
                trace!("Solving linear system");
                // Solve for the new x using current estimates
                let x_new = match self.solver.solve().cloned() {
                    // FIXME — This should only be cloned if converged.
                    Ok(solution) => solution,
                    Err(err) => return Some(Err(err.into())),
                };

                trace!("Checking convergence with tolerance VECTOL={}", VECTOL);
                // Check for convergence using vector tolerance
                if self.has_converged(&x, &x_new, VECTOL) {
                    trace!("Convergence achieved");
                    // If converged, store the result with variable names
                    let res = self.add_var_name(x_new);
                    return Some(Ok(Sim::Op(res)));
                }

                trace!("Update solution vector for next iteration");
                // Update x for the next iteration
                x = x_new;

                None
            })
            .find_map(|result| result);

        // If not converged after maximum iterations, return an error
        match result {
            Some(Ok(res)) => Ok(res),
            Some(Err(err)) => Err(err),
            None => {
                log::error!("OP analysis failed to converge after {} iterations", MAXITER);
                Err(SimulatorError::NonConvergentMaxIter {
                    max_iter: MAXITER,
                    tol: VECTOL,
                })
            },
        }
    }
}
