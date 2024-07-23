use std::sync::Arc;



/// Represents different simulation commands in a circuit simulator.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum SimulationOption {
    /// Represents an operating point analysis command.
    Out(Vec<Arc<str>>)
}