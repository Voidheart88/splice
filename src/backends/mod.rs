mod csv;
mod network;
mod plot;
mod raw;

use clap::ValueEnum;
use miette::Diagnostic;
use plotters::drawing::DrawingAreaErrorKind;
use thiserror::Error;

pub(crate) use csv::CsvBackend;
pub(crate) use network::NetworkBackend;
pub(crate) use plot::PlotBackend;
pub(crate) use raw::RawBackend;

use crate::sim::simulation_result::SimulationResults;

/// Error that can occur during output.
#[derive(Debug, Error, Diagnostic)]
pub enum BackendError {
    /// Error that occurs when the output option is not implemented.
    #[error("This output option is not implemented")]
    #[diagnostic(help("Try helping by implementing this output option!"))]
    Unimplemented,

    /// Error that occurs during plotting.
    #[error("An error occurred during plotting: {0}")]
    PlotError(String),

    #[error("Cant find Max or Min value für plotting")]
    #[diagnostic(help("This is an Error and should be reportet on Github"))]
    CantFindMaxMin,
}

impl From<DrawingAreaErrorKind<std::io::Error>> for BackendError {
    fn from(err: DrawingAreaErrorKind<std::io::Error>) -> Self {
        BackendError::PlotError(format!("{:?}", err))
    }
}

// the trait `From<fn(std::string::String) -> outputs::BackendError {outputs::BackendError::PlotError}>` is not implemented for `outputs::BackendError`

/// Enum for selecting different output options.
#[derive(Copy, Clone, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Backends {
    /// CSV output.
    Csv,
    /// Raw output.
    Raw,
    /// Plot output.
    Plot,
    /// Network output.
    Network,
}

/// Trait for various output types.
pub trait Backend {
    /// Performs the output for the given simulation results.
    ///
    /// # Parameters
    ///
    /// - `res`: The simulation results to be output.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the output was successful, or an `BackendError` if an error occurred.
    fn output(&self, res: SimulationResults) -> Result<(), BackendError>;
}
