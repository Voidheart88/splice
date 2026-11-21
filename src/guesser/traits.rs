// Traits and options for the Guesser abstraction

/// Options controlling which Guesser implementation to use.
/// Options controlling which Guesser implementation to use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuesserOptions;

impl Default for GuesserOptions {
    fn default() -> Self {
        GuesserOptions
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
pub fn new_guesser<SO: crate::solver::Solver>(_options: GuesserOptions) -> Box<dyn Guesser<SO>> {
    Box::new(crate::guesser::StandardGuesser::new())
}
