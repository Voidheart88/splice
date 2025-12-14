use log::info;

use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;
use num::Zero;

pub(super) trait TranSimulation<SO: Solver> {
    fn run_tran(&mut self, tstep: &Numeric, tstop: &Numeric) -> Result<Sim, SimulatorError>;
}

impl<SO: Solver> TranSimulation<SO> for Simulator<SO> {
    fn run_tran(&mut self, tstep: &Numeric, tstop: &Numeric) -> Result<Sim, SimulatorError> {
        info!("Run transient analysis");

        let mut t = Numeric::zero();
        let mut tran_results = Vec::new();

        let mut x_prev: Vec<Numeric> = self.find_op()?.iter().map(|op| op.1).collect();

        while t <= *tstop {
            self.solver.reset();
            self.build_constant_a_mat();
            self.build_constant_b_vec();
            self.build_time_variant_a_mat(tstep);
            self.build_time_variant_b_vec(tstep);
            self.build_nonlinear_a_mat(&x_prev);
            self.build_nonlinear_b_vec(&x_prev);

            let x_new = self.solver.solve()?.clone();

            if self.has_converged(&x_prev, &x_new, VECTOL) {
                tran_results.push((t, self.add_var_name(x_new.clone())));
            } else {
                return Err(SimulatorError::NonConvergentMaxIter {
                    max_iter: MAXITER,
                    tol: VECTOL,
                });
            }

            x_prev = x_new;
            t += tstep;
        }

        Ok(Sim::Tran(tran_results))
    }
}
