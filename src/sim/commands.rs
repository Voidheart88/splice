use std::sync::Arc;

use crate::{frontends::FrontendError, spot::*};

/// Represents different simulation commands in a circuit simulator.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimulationCommand {
    /// Represents an operating point analysis command.
    Op,
    /// Represents a transient analysis command.
    Tran(Numeric, Numeric), //Transient (Timestep, Stop Time)
    /// Represents an AC analysis command.
    Ac(Numeric, Numeric, usize, ACMode),
    /// Represents a DC analysis command.
    Dc(
        Arc<str>,                                      // DC Source Name
        Numeric,                                       // Start Value
        Numeric,                                       // Stop Value
        Numeric,                                       // Step Value
        Option<(Arc<str>, Numeric, Numeric, Numeric)>, // Optional second Source
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

impl TryFrom<&str> for ACMode {
    type Error = FrontendError;

    fn try_from(value: &str) -> Result<Self, FrontendError> {
        match value.to_lowercase().as_str() {
            "dec" => Ok(ACMode::Dec),
            "lin" => Ok(ACMode::Lin),
            "oct" => Ok(ACMode::Oct),
            _ => Err(FrontendError::ParseError(value.into())),
        }
    }
}
