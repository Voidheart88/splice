// State‑Space Guesser implementation (DC linear guess)

use crate::sim::{Simulator, SimulatorError};
use crate::solver::Solver;
use crate::spot::Numeric;
use crate::guesser::traits::Guesser;

/// StateSpaceGuesser provides an initial guess based on a linearized DC state‑space model.
pub struct StateSpaceGuesser;

impl StateSpaceGuesser {
    pub fn new() -> Self {
        StateSpaceGuesser
    }
}

impl<SO: Solver> Guesser<SO> for StateSpaceGuesser {
    fn guess(
        &self,
        sim: &mut Simulator<SO>,
    ) -> Result<Vec<Numeric>, SimulatorError> {
        // Build only the constant (linear) part of the system.
        sim.build_constant_a_mat();
        sim.build_constant_b_vec();
        // Solve the linear system – any singularity will be reported by the solver.
        let x_vec = sim.solver.solve()?;
        Ok(x_vec.clone())
    }
}
