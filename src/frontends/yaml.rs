use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use super::{Element, Frontend, FrontendError, Simulation};

use crate::models::capacitor::yaml::YamlCapacitor;
use crate::models::diode::yaml::YamlDiode;
use crate::models::inductor::yaml::YamlInductor;
use crate::models::isource::yaml::YamlISource;
use crate::models::mosfet::yaml::YamlMos0;
use crate::models::resistor::yaml::YamlResistor;
use crate::models::vsource::yaml::YamlVSource;
use crate::models::Variable;
use crate::sim::commands::ACMode;
use crate::sim::commands::SimulationCommand;
use crate::sim::options::SimulationOption;
use crate::spot::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum YamlElement {
    #[serde(rename = "resistor")]
    Resistor(YamlResistor),
    #[serde(rename = "inductor")]
    Inductor(YamlInductor),
    #[serde(rename = "capacitor")]
    Capacitor(YamlCapacitor),
    #[serde(rename = "vsource")]
    VSource(YamlVSource),
    #[serde(rename = "csource")]
    ISource(YamlISource),
    #[serde(rename = "diode")]
    Diode(YamlDiode),
    #[serde(rename = "mosfet")]
    Mosfet(YamlMos0),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename = "simulations")]
pub enum YamlSimulation {
    #[serde(rename = "op")]
    OP,
    #[serde(rename = "dc")]
    DC(YamlDC),
    #[serde(rename = "ac")]
    AC(YamlAC),
    #[serde(rename = "tran")]
    TRAN,
}

#[derive(Debug, Deserialize)]
pub struct YamlDC {
    source: String,
    vstart: Numeric,
    vstop: Numeric,
    vstep: Numeric,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename = "simulations")]
pub struct YamlAC {
    start: f64,
    stop: f64,
    step: usize,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename = "option")]
pub struct YamlOption {
    pub out: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "circuit")]
pub struct YamlCircuit {
    pub elements: Vec<YamlElement>,
    pub simulations: Vec<YamlSimulation>,
    #[serde(default)]
    pub options: Vec<YamlOption>,
}

pub struct YamlFrontend {
    commands: Vec<SimulationCommand>,
    options: Vec<SimulationOption>,
    elements: Vec<Element>,
    variables: Vec<Variable>,
}

impl YamlFrontend {
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

        let yaml: YamlCircuit = match serde_yml::from_str(&yaml_string) {
            Ok(yaml) => yaml,
            Err(err) => return Err(FrontendError::ParseError(format!("{}", err))),
        };

        for element in yaml.elements {
            match element {
                YamlElement::Resistor(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                YamlElement::Inductor(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                YamlElement::Capacitor(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                YamlElement::VSource(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                YamlElement::ISource(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
                YamlElement::Diode(ele) => ele.process(&mut variables, &mut elements, &mut var_map),
                YamlElement::Mosfet(ele) => {
                    ele.process(&mut variables, &mut elements, &mut var_map)
                }
            };
        }

        for simulation in yaml.simulations {
            match simulation {
                YamlSimulation::OP => Self::process_op(&mut commands),
                YamlSimulation::DC(yamldc) => Self::process_dc(&mut commands, yamldc),
                YamlSimulation::AC(yamlac) => Self::process_ac(&mut commands, yamlac),
                YamlSimulation::TRAN => Self::process_tran(&mut commands),
            };
        }

        for option in yaml.options {
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

    fn process_dc(commands: &mut Vec<SimulationCommand>, yamldc: YamlDC) {
        commands.push(SimulationCommand::Dc(
            Arc::from(yamldc.source),
            yamldc.vstart,
            yamldc.vstop,
            yamldc.vstep,
            None,
        ));
    }

    fn process_ac(commands: &mut Vec<SimulationCommand>, yamlac: YamlAC) {
        commands.push(SimulationCommand::Ac(
            yamlac.start,
            yamlac.stop,
            yamlac.step,
            ACMode::Dec,
        ))
    }

    fn process_tran(_commands: &mut Vec<SimulationCommand>) {
        todo!()
    }

    fn process_out(options: &mut Vec<SimulationOption>, option: YamlOption) {
        options.push(SimulationOption::Out(vec![Arc::from(option.out.as_str())]))
    }
}

impl Frontend for YamlFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        Ok(Simulation {
            commands: self.commands.clone(),
            options: self.options.clone(),
            elements: self.elements.clone(),
            variables: self.variables.clone(),
        })
    }
}

pub(crate) trait ProcessYamlElement {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    );
}
