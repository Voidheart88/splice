use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use super::{Element, Frontend, FrontendError, Simulation};

use crate::models::capacitor::serde::SerdeCapacitor;
use crate::models::diode::serde::SerdeDiode;
use crate::models::inductor::serde::SerdeInductor;
use crate::models::isource::serde::SerdeISource;
use crate::models::mosfet::serde::SerdeMos0;
use crate::models::resistor::serde::SerdeResistor;
use crate::models::vsource::serde::SerdeVSource;
use crate::models::Variable;
use crate::sim::commands::ACMode;
use crate::sim::commands::SimulationCommand;
use crate::sim::options::SimulationOption;
use crate::spot::*;
use serde::Deserialize;

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
    #[serde(rename = "isource")]
    ISource(SerdeISource),
    #[serde(rename = "diode")]
    Diode(SerdeDiode),
    #[serde(rename = "mosfet")]
    Mosfet(SerdeMos0),
}

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
    TRAN,
}

#[derive(Debug, Deserialize)]
pub struct SerdeDC {
    source: String,
    vstart: Numeric,
    vstop: Numeric,
    vstep: Numeric,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename = "simulations")]
pub struct SerdeAC {
    fstart: Numeric,
    fstop: Numeric,
    fstep: usize,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename = "option")]
pub struct SerdeOption {
    pub out: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "circuit")]
pub struct SerdeCircuit {
    pub elements: Vec<SerdeElement>,
    pub simulations: Vec<SerdeSimulation>,
    #[serde(default)]
    pub options: Vec<SerdeOption>,
}

pub struct SerdeFrontend {
    commands: Vec<SimulationCommand>,
    options: Vec<SimulationOption>,
    elements: Vec<Element>,
    variables: Vec<Variable>,
}

impl SerdeFrontend {
    pub fn try_new_from_path(path: String) -> Result<Self, FrontendError> {
        let mut circuit_string = String::new();
        match File::open(path) {
            Ok(mut file) => file.read_to_string(&mut circuit_string)?,
            Err(err) => return Err(FrontendError::FileReadError(format!("{}", err))),
        };

        Self::try_new_from_string(circuit_string)
    }

    pub fn try_new_from_string(yaml_string: String) -> Result<Self, FrontendError> {
        let mut commands: Vec<SimulationCommand> = Vec::new();
        let mut options: Vec<SimulationOption> = Vec::new();
        let mut elements: Vec<Element> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();

        let circuit: SerdeCircuit = match serde_yml::from_str(&yaml_string) {
            Ok(yaml) => yaml,
            Err(err) => return Err(FrontendError::ParseError(format!("{}", err))),
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
                SerdeElement::ISource(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                SerdeElement::Diode(ele) => ele.process(&mut variables, &mut elements, &mut var_map),
                SerdeElement::Mosfet(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
            };
        }

        for simulation in circuit.simulations {
            match simulation {
                SerdeSimulation::OP => Self::process_op(&mut commands),
                SerdeSimulation::DC(yamldc) => Self::process_dc(&mut commands, yamldc),
                SerdeSimulation::AC(yamlac) => Self::process_ac(&mut commands, yamlac),
                SerdeSimulation::TRAN => Self::process_tran(&mut commands),
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

    fn process_op(commands: &mut Vec<SimulationCommand>) {
        commands.push(SimulationCommand::Op)
    }

    fn process_dc(commands: &mut Vec<SimulationCommand>, yamldc: SerdeDC) {
        commands.push(SimulationCommand::Dc(
            Arc::from(yamldc.source),
            yamldc.vstart,
            yamldc.vstop,
            yamldc.vstep,
            None,
        ));
    }

    fn process_ac(commands: &mut Vec<SimulationCommand>, yamlac: SerdeAC) {
        commands.push(SimulationCommand::Ac(
            yamlac.fstart,
            yamlac.fstop,
            yamlac.fstep,
            ACMode::Dec,
        ))
    }

    fn process_tran(_commands: &mut Vec<SimulationCommand>) {
        todo!()
    }

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

pub(crate) trait ProcessSerdeElement {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    );
}
