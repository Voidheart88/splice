// Traits and options for the Guesser abstraction

/// Options controlling which Guesser implementation to use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuesserOptions {
    /// If true, a future State‑Space Guesser would be used (currently unimplemented).
    pub use_state_space: bool,
}

impl Default for GuesserOptions {
    fn default() -> Self {
        Self {
            use_state_space: false,
        }
    }
}

/// Trait that all guesser implementations must satisfy.
/// It produces an initial guess vector for the given simulator.
pub trait Guesser<SO: crate::solver::Solver> {
    fn guess(
        &self,
        sim: &mut crate::sim::Simulator<SO>,
    ) -> Result<Vec<crate::spot::Numeric>, crate::sim::SimulatorError>;
}

/// Factory function returning a concrete Guesser based on the provided options.
/// Currently always returns a `StandardGuesser`.
pub fn new_guesser<SO: crate::solver::Solver>(
    options: GuesserOptions,
) -> Box<dyn Guesser<SO>> {
    if options.use_state_space {
        Box::new(crate::guesser::state_space::StateSpaceGuesser::new())
    } else {
        Box::new(crate::guesser::StandardGuesser::new())
    }
}

