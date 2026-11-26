use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use super::{Element, Frontend, FrontendError, Simulation};

use crate::models::SerdeCoupledInductors;
use crate::models::Variable;
use crate::models::bjt::serde::{SerdeBjt0, SerdeBjt1};
use crate::models::capacitor::serde::SerdeCapacitor;
use crate::models::controlled_sources::serde::{SerdeCCCS, SerdeCCVS, SerdeVCCS, SerdeVCVS};
use crate::models::diode::serde::SerdeDiode;
use crate::models::gain::serde::SerdeGain;
use crate::models::inductor::serde::SerdeInductor;
use crate::models::isource::serde::SerdeISource;
use crate::models::mosfet::serde::SerdeMos0;
use crate::models::resistor::serde::SerdeResistor;
use crate::models::vsource::serde::SerdeVSource;
use crate::models::vsource_sine::serde::SerdeVSourceSin;
use crate::models::vsource_step::serde::SerdeVSourceStep;
use crate::sim::commands::ACMode;
use crate::sim::commands::SimulationCommand;
use crate::sim::options::{IntegrationMethodEnum, SimulationOption};
use crate::spot::*;
use serde::{Deserialize, Serialize};

/// Represents the types of electrical elements that can be defined in a circuit.
/// Each variant corresponds to a specific circuit element (e.g., resistor, capacitor).
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SerdeElement {
    #[serde(rename = "resistor")]
    Resistor(SerdeResistor),
    #[serde(rename = "inductor")]
    Inductor(SerdeInductor),
    #[serde(rename = "coupled_inductors")]
    CoupledInductors(SerdeCoupledInductors),
    #[serde(rename = "capacitor")]
    Capacitor(SerdeCapacitor),
    #[serde(rename = "vsource")]
    VSource(SerdeVSource),
    #[serde(rename = "vsource_sine")]
    VSourceSin(SerdeVSourceSin),
    #[serde(rename = "vsource_step")]
    VSourceStep(SerdeVSourceStep),
    #[serde(rename = "isource")]
    ISource(SerdeISource),
    #[serde(rename = "diode")]
    Diode(SerdeDiode),
    #[serde(rename = "mosfet")]
    Mosfet(SerdeMos0),
    #[serde(rename = "bjt0")]
    Bjt0(SerdeBjt0),
    #[serde(rename = "bjt1")]
    Bjt1(SerdeBjt1),
    #[serde(rename = "gain")]
    Gain(SerdeGain),
    #[serde(rename = "vcvs")]
    VCVS(SerdeVCVS),
    #[serde(rename = "vccs")]
    VCCS(SerdeVCCS),
    #[serde(rename = "cccs")]
    CCCS(SerdeCCCS),
    #[serde(rename = "ccvs")]
    CCVS(SerdeCCVS),
}

/// Represents the types of simulations that can be performed on a circuit.
/// Each variant corresponds to a specific simulation type (e.g., OP, DC, AC, transient).
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeDC {
    source: String,
    vstart: Numeric,
    vstop: Numeric,
    vstep: Numeric,
}

impl SerdeDC {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn vstart(&self) -> Numeric {
        self.vstart
    }

    pub fn vstop(&self) -> Numeric {
        self.vstop
    }

    pub fn vstep(&self) -> Numeric {
        self.vstep
    }

    pub fn new(source: String, vstart: Numeric, vstop: Numeric, vstep: Numeric) -> Self {
        Self {
            source,
            vstart,
            vstop,
            vstep,
        }
    }
}

/// Configuration for an AC analysis simulation.
/// Specifies the start frequency, stop frequency, and number of steps.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename = "simulations")]
pub struct SerdeAC {
    fstart: Numeric,
    fstop: Numeric,
    fstep: usize,
}

impl SerdeAC {
    pub fn fstart(&self) -> Numeric {
        self.fstart
    }

    pub fn fstop(&self) -> Numeric {
        self.fstop
    }

    pub fn fstep(&self) -> usize {
        self.fstep
    }
}

/// Configuration for an AC analysis simulation.
/// Specifies the start frequency, stop frequency, and number of steps.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename = "simulations")]
pub struct SerdeTran {
    tstep: Numeric,
    tend: Numeric,
}

impl SerdeTran {
    pub fn tstep(&self) -> Numeric {
        self.tstep
    }

    pub fn tend(&self) -> Numeric {
        self.tend
    }
}

/// Represents simulation output options.
/// Simulation option for serialized circuit formats (YAML/JSON).
/// Supports output variables, integration method, GMIN, and RSHUNT.
#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename = "option")]
pub struct SerdeOption {
    /// The output variable or node to save.
    pub out: Option<String>,
    /// Integration method: "be", "forward_euler", "trapezoidal", or "gear"
    pub integration: Option<String>,
    /// Minimum conductance to ground (default: 1e-12 S)
    pub gmin: Option<f64>,
    /// Shunt resistance to ground (default: 1e9 Ω)
    pub rshunt: Option<f64>,
}

/// Represents a circuit defined in a serialization format (e.g., YAML or JSON).
/// Contains a list of elements, simulations, and options.
#[derive(Debug, Deserialize, Serialize)]
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
    pub(crate) fn try_new_from_path(
        path: String,
        format: SerdeFormat,
    ) -> Result<Self, FrontendError> {
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
    pub(crate) fn try_new_from_string(
        circuit_string: String,
        format: SerdeFormat,
    ) -> Result<Self, FrontendError> {
        let circuit: SerdeCircuit = match format {
            SerdeFormat::Yaml => serde_yml::from_str(&circuit_string)?,
            SerdeFormat::Json => serde_json::from_str(&circuit_string)?,
        };

        let mut elements: Vec<Element> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();

        circuit.elements.iter().for_each(|element| {
            element.process(&mut variables, &mut elements, &mut var_map);
        });

        let commands = circuit.simulations.into_iter().map(Into::into).collect();
        let options = circuit.options.into_iter().map(Into::into).collect();

        Ok(Self {
            commands,
            options,
            elements,
            variables,
        })
    }
}

impl From<SerdeSimulation> for SimulationCommand {
    fn from(simulation: SerdeSimulation) -> Self {
        match simulation {
            SerdeSimulation::OP => SimulationCommand::Op,
            SerdeSimulation::DC(serdedc) => SimulationCommand::Dc(
                Arc::from(serdedc.source),
                serdedc.vstart,
                serdedc.vstop,
                serdedc.vstep,
                None,
            ),
            SerdeSimulation::AC(serdeac) => {
                SimulationCommand::Ac(serdeac.fstart, serdeac.fstop, serdeac.fstep, ACMode::Dec)
            }
            SerdeSimulation::Tran(serdetran) => {
                SimulationCommand::Tran(serdetran.tstep, serdetran.tend)
            }
        }
    }
}

impl From<SerdeOption> for SimulationOption {
    fn from(option: SerdeOption) -> Self {
        // Check for GMIN first
        if let Some(gmin) = option.gmin {
            return SimulationOption::GMin(gmin);
        }

        // Check for RSHUNT
        if let Some(rshunt) = option.rshunt {
            return SimulationOption::RShunt(rshunt);
        }

        // Check for integration method
        if let Some(method) = option.integration {
            let integration_method = match method.as_str() {
                "be" | "backward_euler" => IntegrationMethodEnum::BackwardEuler,
                "forward_euler" => IntegrationMethodEnum::ForwardEuler,
                "trapezoidal" => IntegrationMethodEnum::Trapezoidal,
                "gear" | "bdf" => IntegrationMethodEnum::GearBdf { order: 2 },
                _ => IntegrationMethodEnum::BackwardEuler,
            };
            return SimulationOption::IntegrationMethod(integration_method);
        }

        // Default to OUT (backward compatibility)
        let out_name = option.out.unwrap_or_default();
        SimulationOption::Out(vec![Arc::from(out_name.as_str())])
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

impl ProcessSerdeElement for SerdeElement {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        match self {
            SerdeElement::Resistor(ele) => ele.process(variables, elements, var_map),
            SerdeElement::Inductor(ele) => ele.process(variables, elements, var_map),
            SerdeElement::CoupledInductors(ele) => ele.process(variables, elements, var_map),
            SerdeElement::Capacitor(ele) => ele.process(variables, elements, var_map),
            SerdeElement::VSource(ele) => ele.process(variables, elements, var_map),
            SerdeElement::VSourceSin(ele) => ele.process(variables, elements, var_map),
            SerdeElement::VSourceStep(ele) => ele.process(variables, elements, var_map),
            SerdeElement::ISource(ele) => ele.process(variables, elements, var_map),
            SerdeElement::Diode(ele) => ele.process(variables, elements, var_map),
            SerdeElement::Bjt0(ele) => ele.process(variables, elements, var_map),
            SerdeElement::Bjt1(ele) => ele.process(variables, elements, var_map),
            SerdeElement::Mosfet(ele) => ele.process(variables, elements, var_map),
            SerdeElement::Gain(ele) => ele.process(variables, elements, var_map),
            SerdeElement::VCVS(ele) => ele.process(variables, elements, var_map),
            SerdeElement::VCCS(ele) => ele.process(variables, elements, var_map),
            SerdeElement::CCCS(ele) => ele.process(variables, elements, var_map),
            SerdeElement::CCVS(ele) => ele.process(variables, elements, var_map),
        }
    }
}

impl ProcessSerdeElement for SerdeCoupledInductors {
    fn process(
        &self,
        _variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        _var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let coupled_inductors = crate::models::CoupledInductorsBundle::new(
            Arc::from(self.name.as_str()),
            Arc::from(self.inductor1.as_str()),
            Arc::from(self.inductor2.as_str()),
            self.coupling_factor,
        );
        elements.push(Element::CoupledInductors(coupled_inductors));
    }
}
