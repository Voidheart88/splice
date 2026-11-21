// Placeholder for future State‑Space Guesser implementation

use crate::sim::{Simulator, SimulatorError};
use crate::solver::Solver;
use crate::spot::Numeric;
use crate::guesser::traits::{Guesser, GuesserOptions};

pub struct StateSpaceGuesser;

impl StateSpaceGuesser {
    pub fn new() -> Self {
        StateSpaceGuesser
    }
}

impl<SO: Solver> Guesser<SO> for StateSpaceGuesser {
    fn guess(
        &self,
        _sim: &mut Simulator<SO>,
    ) -> Result<Vec<Numeric>, SimulatorError> {
        // Not implemented yet – panic with a clear message.
        unimplemented!("StateSpaceGuesser is not implemented yet");
    }
}
