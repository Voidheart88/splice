// StandardGuesser forwards to the existing Simulator::generate_initial_guess.

use crate::sim::{Simulator, SimulatorError};
use crate::solver::Solver;
use crate::spot::Numeric;
use crate::guesser::traits::Guesser;

pub struct StandardGuesser {
    // Currently no fields needed.
}

impl StandardGuesser {
    pub fn new() -> Self {
        StandardGuesser {}
    }
}

impl<SO: Solver> Guesser<SO> for StandardGuesser {
    fn guess(
        &self,
        sim: &mut Simulator<SO>,
    ) -> Result<Vec<Numeric>, SimulatorError> {
        // Directly call the (now public) generate_initial_guess method.
        Ok(sim.generate_initial_guess())
    }
}
