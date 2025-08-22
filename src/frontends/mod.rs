pub(crate) mod json;
pub(crate) mod kicad;
pub(crate) mod network;
pub(crate) mod spice;
pub(crate) mod yaml;

use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use clap::ValueEnum;
use miette::Diagnostic;
use thiserror::Error;

use crate::models::*;
use crate::sim::commands::{ACMode, SimulationCommand};
use crate::sim::options::SimulationOption;
pub(crate) use json::JsonFrontend;
pub(crate) use kicad::KicadFrontend;
pub(crate) use network::NetworkFrontend;
pub(crate) use spice::SpiceFrontend;
pub(crate) use yaml::YamlFrontend;

#[derive(Copy, Clone, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Frontends {
    Spice,
    Yaml,
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

    #[error("Parse Error \n{0}")]
    #[diagnostic(help("Check Element"))]
    PestError(String),

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

#[derive(Debug, Clone)]
pub struct Simulation {
    pub commands: Vec<SimulationCommand>,
    pub options: Vec<SimulationOption>,
    pub elements: Vec<Element>,
    pub variables: Vec<Variable>,
}

/// The Frontend trait defines the interface between the choosen frontend
/// and the simulator.
pub trait Frontend {
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

pub(crate) fn get_variable(
    inp: &str,
    unit: Unit,
    variables: &mut Vec<Variable>,
    var_map: &mut HashMap<Arc<str>, usize>,
) -> Option<Variable> {
    if inp == "0" {
        return None;
    }

    let inp_arc = Arc::from(inp);

    if let Some(&index) = var_map.get(&inp_arc) {
        return Some(variables[index].clone());
    }

    let new_variable = Variable::new(inp_arc.clone(), unit, variables.len());
    var_map.insert(inp_arc, variables.len());
    variables.push(new_variable.clone());

    Some(new_variable)
}

#[cfg(test)]
mod tests;
