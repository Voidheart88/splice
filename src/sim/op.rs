use log::info;

use crate::Simulator;
use crate::sim::SimulatorError;
use crate::sim::simulation_result::Sim;
use crate::solver::Solver;
use crate::spot::*;

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
        let guesser = crate::guesser::new_guesser(self.guesser_options.clone());
        let x = guesser.guess(self)?;

        // Use Newton-Raphson with Armijo step size control for global convergence
        // For now, use the standard Newton iteration
        let mut x_current = x;
        let mut converged = false;

        for _ in 0..MAXITER {
            self.build_constant_a_mat();
            self.build_constant_b_vec();
            self.build_nonlinear_a_mat(&x_current);
            self.build_nonlinear_b_vec(&x_current);

            let x_new = match self.solver.solve() {
                Ok(solution) => solution.clone(),
                Err(_) => break,
            };

            if self.has_converged(&x_current, &x_new, VECTOL) {
                x_current = x_new;
                converged = true;
                break;
            }

            x_current = x_new;
        }

        if !converged {
            return Err(SimulatorError::NonConvergentMaxIter {
                max_iter: MAXITER,
                tol: VECTOL,
            });
        }

        let res = self.add_var_name(x_current);
        Ok(Sim::Op(res))
    }
}
