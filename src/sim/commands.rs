use std::sync::Arc;

use crate::spot::*;

/// Represents different simulation commands in a circuit simulator.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimulationCommand {
    /// Represents an operating point analysis command.
    Op,
    /// Represents a transient analysis command.
    Tran,
    /// Represents an AC analysis command.
    Ac(Numeric, Numeric, usize, ACMode),
    /// Represents a DC analysis command.
    Dc(
        Arc<str>,
        Numeric,
        Numeric,
        Numeric,
        Option<(Arc<str>, Numeric, Numeric, Numeric)>,
    ),
}

/// Represents the ac simulation options
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum ACMode {
    #[default]
    Lin,
    Dec,
    Oct,
}
