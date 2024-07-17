pub(crate) mod json;
pub(crate) mod kicad;
pub(crate) mod network;
pub(crate) mod spice;
//pub(crate) mod spice_pest;
pub(crate) mod yml;

use std::io;

use clap::ValueEnum;
use miette::Diagnostic;
use thiserror::Error;

use crate::models::*;
use crate::sim::commands::{ACMode, SimulationCommand};
pub(crate) use json::JsonFrontend;
pub(crate) use kicad::KicadFrontend;
pub(crate) use network::NetworkFrontend;
pub(crate) use spice::SpiceFrontend;
//pub(crate) use spice_pest::SpicePestFrontend;
pub(crate) use yml::YmlFrontend;

#[derive(Copy, Clone, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Frontends {
    Spice,
    //SpicePest,
    Yml,
    Json,
    Network,
    Kicad,
    Select,
}

#[derive(Debug, Error, Diagnostic)]
pub enum FrontendError {
    #[error("The Frontend is not implemented")]
    #[diagnostic(help("try helping by implementing this frontend!"))]
    Unimplemented,

    #[error("A suitable frontend could not be found")]
    #[diagnostic(help("No frontend found that can handle the given input.\n Please check if the file format is supported."))]
    FrontendNotFound,

    #[error("Element {0} already in circuit")]
    #[diagnostic(help("Rename one of the elements: {0}"))]
    ElementDouble(String),

    #[error("IO Error: {0}")]
    #[diagnostic(help("Check the path"))]
    IoError(String),

    //#[error("Parse Error \n{0}")]
    //#[diagnostic(help("Check Element"))]
    //PestError(String),

    #[error("Parse Error")]
    #[diagnostic(help("Check Element"))]
    ParseError(String),

    #[error("Parse Command Error")]
    #[diagnostic(help("{0}"))]
    ParseCommandError(String),
}

impl From<io::Error> for FrontendError {
    fn from(error: io::Error) -> Self {
        FrontendError::IoError(format!("{}", error))
    }
}

impl From<std::num::ParseFloatError> for FrontendError {
    fn from(error: std::num::ParseFloatError) -> Self {
        FrontendError::ParseCommandError(format!("{}", error))
    }
}

impl From<std::num::ParseIntError> for FrontendError {
    fn from(error: std::num::ParseIntError) -> Self {
        FrontendError::ParseCommandError(format!("{}", error))
    }
}
pub struct SelectFrontend {}

impl SelectFrontend {
    /// Automatically select a frontend from a file extension
    pub fn from_path(pth: String) -> Result<Box<dyn Frontend>, FrontendError> {
        let end = pth.split(".").last().unwrap();
        match end {
            "yml" => Err(FrontendError::Unimplemented),
            "yaml" => Err(FrontendError::Unimplemented),
            "json" => Err(FrontendError::Unimplemented),
            "kicad_sch" => Err(FrontendError::Unimplemented),
            "cir" => Ok(Box::new(SpiceFrontend::new(pth))),
            "lib" => Ok(Box::new(SpiceFrontend::new(pth))),
            _ => Err(FrontendError::FrontendNotFound),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Simulation {
    pub variables: Vec<Variable>,
    pub elements: Vec<Element>,
    pub commands: Vec<SimulationCommand>,
}

/// The Frontend trait defines the interface between the choosen frontend
/// and the simulator.
pub(crate) trait Frontend {
    /// The provided circuit method must be implemented by the frontend and returns
    /// a circuit.
    /// This Simulation consists of a vector with CircuitElements and a vector of commands
    fn simulation(&self) -> Result<Simulation, FrontendError>;
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

#[cfg(test)]
mod tests;
