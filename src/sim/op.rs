use log::info;
use log::trace;

use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;

pub(super) trait OpSimulation<SO: Solver> {
    fn run_op(&mut self) -> Result<Sim, SimulatorError>;
}

impl<SO: Solver> OpSimulation<SO> for Simulator<SO> {
    fn run_op(&mut self) -> Result<Sim, SimulatorError> {
        info!("Run operating point analysis");

        if !self.has_nonlinear_elements() {
            self.build_constant_a_mat();
            self.build_constant_b_vec();
            let x_vec = self.solver.solve()?.clone();
            let res = self.add_var_name(x_vec);
            return Ok(Sim::Op(res));
        }

        // Build the initial guess
        let mut x = self.generate_initial_guess();

        // Use an iterator for the iterations
        let result = (0..MAXITER)
            .map(|run| {
                trace!("Iteration: {run}");
                trace!("Set matrix");
                self.build_constant_a_mat();
                self.build_constant_b_vec();
                self.build_nonlinear_a_mat(&x);
                self.build_nonlinear_b_vec(&x);
                trace!("Solve matrix");
                // Solve for the new x
                let x_new = match self.solver.solve().cloned() {
                    // FIXME — This should only be cloned if converged.
                    Ok(solution) => solution,
                    Err(err) => return Some(Err(err.into())),
                };

                trace!("Check convergence matrix");
                // Check for convergence
                if self.has_converged(&x, &x_new, VECTOL) {
                    // If converged, store the result
                    let res = self.add_var_name(x_new);
                    return Some(Ok(Sim::Op(res)));
                }

                trace!("Update x");
                // Update x for the next iteration
                x = x_new;

                None
            })
            .find_map(|result| result);

        // If not converged after maximum iterations, return an error
        match result {
            Some(Ok(res)) => Ok(res),
            Some(Err(err)) => Err(err),
            None => Err(SimulatorError::NonConvergentMaxIter {
                max_iter: MAXITER,
                tol: VECTOL,
            }),
        }
    }
}
