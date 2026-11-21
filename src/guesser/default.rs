// StandardGuesser forwards to the existing Simulator::generate_initial_guess.

use crate::sim::{Simulator, SimulatorError};
use crate::solver::Solver;
use crate::spot::Numeric;
use crate::guesser::traits::Guesser;

/// The default guesser implementation.
pub struct StandardGuesser {
    // Currently no fields needed.
}

impl StandardGuesser {
    /// Creates a new `StandardGuesser`.
    pub fn new() -> Self {
        StandardGuesser {}
    }
}

// Provide a `Default` implementation required by the codebase.
impl Default for StandardGuesser {
    fn default() -> Self {
        Self::new()
    }
}

// Implement the `Guesser` trait for any solver.
impl<SO: Solver> Guesser<SO> for StandardGuesser {
    fn guess(
        &self,
        sim: &mut Simulator<SO>,
    ) -> Result<Vec<Numeric>, SimulatorError> {
        // Directly call the (now public) generate_initial_guess method.
        Ok(sim.generate_initial_guess())
    }
}
