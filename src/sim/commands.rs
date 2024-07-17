use std::sync::Arc;

/// Represents different simulation commands in a circuit simulator.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum SimulationCommand {
    /// Represents an operating point analysis command.
    Op,
    /// Represents a transient analysis command.
    Tran,
    /// Represents an AC analysis command.
    Ac,
    /// Represents a DC analysis command.
    Dc(Arc<str>, f64, f64, f64, Option<(Arc<str>, f64, f64, f64)>),
}
