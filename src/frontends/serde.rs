use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use super::{Element, Frontend, FrontendError, Simulation};

use crate::models::capacitor::serde::SerdeCapacitor;
use crate::models::diode::serde::SerdeDiode;
use crate::models::gain::serde::SerdeGain;
use crate::models::inductor::serde::SerdeInductor;
use crate::models::isource::serde::SerdeISource;
use crate::models::mosfet::serde::SerdeMos0;
use crate::models::resistor::serde::SerdeResistor;
use crate::models::vsource::serde::SerdeVSource;
use crate::models::vsource_sine::serde::SerdeVSourceSin;
use crate::models::Variable;
use crate::sim::commands::ACMode;
use crate::sim::commands::SimulationCommand;
use crate::sim::options::SimulationOption;
use crate::spot::*;
use serde::Deserialize;

/// Represents the types of electrical elements that can be defined in a circuit.
/// Each variant corresponds to a specific circuit element (e.g., resistor, capacitor).
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SerdeElement {
    #[serde(rename = "resistor")]
    Resistor(SerdeResistor),
    #[serde(rename = "inductor")]
    Inductor(SerdeInductor),
    #[serde(rename = "capacitor")]
    Capacitor(SerdeCapacitor),
    #[serde(rename = "vsource")]
    VSource(SerdeVSource),
    #[serde(rename = "vsource_sine")]
    VSourceSin(SerdeVSourceSin),
    #[serde(rename = "isource")]
    ISource(SerdeISource),
    #[serde(rename = "diode")]
    Diode(SerdeDiode),
    #[serde(rename = "mosfet")]
    Mosfet(SerdeMos0),
    #[serde(rename = "gain")]
    Gain(SerdeGain),
}

/// Represents the types of simulations that can be performed on a circuit.
/// Each variant corresponds to a specific simulation type (e.g., OP, DC, AC, transient).
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename = "simulations")]
pub enum SerdeSimulation {
    #[serde(rename = "op")]
    OP,
    #[serde(rename = "dc")]
    DC(SerdeDC),
    #[serde(rename = "ac")]
    AC(SerdeAC),
    #[serde(rename = "tran")]
    Tran(SerdeTran),
}

/// Configuration for a DC sweep simulation.
/// Specifies the source, start voltage, stop voltage, and step size.
#[derive(Debug, Deserialize)]
pub struct SerdeDC {
    source: String,
    vstart: Numeric,
    vstop: Numeric,
    vstep: Numeric,
}

/// Configuration for an AC analysis simulation.
/// Specifies the start frequency, stop frequency, and number of steps.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename = "simulations")]
pub struct SerdeAC {
    fstart: Numeric,
    fstop: Numeric,
    fstep: usize,
}

/// Configuration for an AC analysis simulation.
/// Specifies the start frequency, stop frequency, and number of steps.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename = "simulations")]
pub struct SerdeTran {
    tstep: Numeric,
    tend: Numeric,
}

/// Represents simulation output options.
/// Currently only supports specifying output variables.
#[derive(Default, Debug, Deserialize)]
#[serde(rename = "option")]
pub struct SerdeOption {
    /// The output variable or node to save.
    pub out: String,
}

/// Represents a circuit defined in a serialization format (e.g., YAML or JSON).
/// Contains a list of elements, simulations, and options.
#[derive(Debug, Deserialize)]
#[serde(rename = "circuit")]
pub struct SerdeCircuit {
    pub elements: Vec<SerdeElement>,
    pub simulations: Vec<SerdeSimulation>,
    #[serde(default)]
    pub options: Vec<SerdeOption>,
}

/// Frontend for parsing and processing circuit definitions from serialized formats (YAML/JSON).
/// Converts serialized circuit data into internal representations for simulation.
pub struct SerdeFrontend {
    commands: Vec<SimulationCommand>,
    options: Vec<SimulationOption>,
    elements: Vec<Element>,
    variables: Vec<Variable>,
}

/// Specifies the serialization format for the circuit definition.
pub(crate) enum SerdeFormat {
    /// YAML format.
    Yaml,
    /// JSON format.
    Json,
}

impl SerdeFrontend {
    /// Attempts to create a new `SerdeFrontend` by reading a circuit definition from a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file containing the circuit definition.
    /// * `format` - The format of the file (YAML or JSON).
    ///
    /// # Returns
    ///
    /// * `Result<Self, FrontendError>` - A new `SerdeFrontend` instance if successful, or an error.
    pub fn try_new_from_path(path: String, format: SerdeFormat) -> Result<Self, FrontendError> {
        let mut circuit_string = String::new();
        match File::open(path) {
            Ok(mut file) => file.read_to_string(&mut circuit_string)?,
            Err(err) => return Err(FrontendError::FileReadError(format!("{err}"))),
        };

        Self::try_new_from_string(circuit_string, format)
    }

    /// Attempts to create a new `SerdeFrontend` by parsing a circuit definition from a string.
    ///
    /// # Arguments
    ///
    /// * `circuit_string` - The string containing the circuit definition.
    /// * `format` - The format of the string (YAML or JSON).
    ///
    /// # Returns
    ///
    /// * `Result<Self, FrontendError>` - A new `SerdeFrontend` instance if successful, or an error.
    pub fn try_new_from_string(
        circuit_string: String,
        format: SerdeFormat,
    ) -> Result<Self, FrontendError> {
        let mut commands: Vec<SimulationCommand> = Vec::new();
        let mut options: Vec<SimulationOption> = Vec::new();
        let mut elements: Vec<Element> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();

        let circuit: SerdeCircuit = match format {
            SerdeFormat::Yaml => serde_yml::from_str(&circuit_string)?,
            SerdeFormat::Json => serde_json::from_str(&circuit_string)?,
        };

        for element in circuit.elements {
            match element {
                SerdeElement::Resistor(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::Inductor(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::Capacitor(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::VSource(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::VSourceSin(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::ISource(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::Diode(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::Mosfet(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::Gain(ele) => ele.process(&mut variables, &mut elements, &mut var_map),
            };
        }

        for simulation in circuit.simulations {
            match simulation {
                SerdeSimulation::OP => Self::process_op(&mut commands),
                SerdeSimulation::DC(serdedc) => Self::process_dc(&mut commands, serdedc),
                SerdeSimulation::AC(serdeac) => Self::process_ac(&mut commands, serdeac),
                SerdeSimulation::Tran(serdetran) => Self::process_tran(&mut commands, serdetran),
            };
        }

        for option in circuit.options {
            Self::process_out(&mut options, option);
        }

        Ok(Self {
            commands,
            options,
            elements,
            variables,
        })
    }

    /// Processes an operating point analysis simulation.
    fn process_op(commands: &mut Vec<SimulationCommand>) {
        commands.push(SimulationCommand::Op)
    }

    /// Processes a DC sweep simulation.
    fn process_dc(commands: &mut Vec<SimulationCommand>, serdedc: SerdeDC) {
        commands.push(SimulationCommand::Dc(
            Arc::from(serdedc.source),
            serdedc.vstart,
            serdedc.vstop,
            serdedc.vstep,
            None,
        ));
    }

    /// Processes an AC analysis simulation.
    fn process_ac(commands: &mut Vec<SimulationCommand>, serdeac: SerdeAC) {
        commands.push(SimulationCommand::Ac(
            serdeac.fstart,
            serdeac.fstop,
            serdeac.fstep,
            ACMode::Dec,
        ))
    }

    /// Processes a transient analysis simulation.
    fn process_tran(commands: &mut Vec<SimulationCommand>, serdetran: SerdeTran) {
        commands.push(SimulationCommand::Tran(serdetran.tstep, serdetran.tend))
    }

    /// Processes output options.
    fn process_out(options: &mut Vec<SimulationOption>, option: SerdeOption) {
        options.push(SimulationOption::Out(vec![Arc::from(option.out.as_str())]))
    }
}

impl Frontend for SerdeFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        Ok(Simulation {
            commands: self.commands.clone(),
            options: self.options.clone(),
            elements: self.elements.clone(),
            variables: self.variables.clone(),
        })
    }
}

/// Trait for processing serialized circuit elements into internal representations.
pub(crate) trait ProcessSerdeElement {
    /// Processes the element, updating the provided vectors and hashmap.
    ///
    /// # Arguments
    ///
    /// * `variables` - A mutable reference to the vector of variables.
    /// * `elements` - A mutable reference to the vector of elements.
    /// * `var_map` - A mutable reference to the hashmap mapping variable names to indices.
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    );
}
