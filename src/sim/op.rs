use log::info;

use crate::Simulator;
use crate::sim::SimulatorError;
use crate::sim::newton_armijo::NewtonArmijo;
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
        let armijo_solver = NewtonArmijo::new();
        let x_current = armijo_solver.solve(&mut *self, x, VECTOL)?;

        let res = self.add_var_name(x_current);
        Ok(Sim::Op(res))
    }
}
