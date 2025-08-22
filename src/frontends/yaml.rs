use std::collections::HashMap;
use std::sync::Arc;

use super::{Element, Frontend, FrontendError, Simulation};
use crate::models::resistor::yaml::YamlResistor;
use crate::models::{Unit, Variable};
use crate::sim::commands::SimulationCommand;
use crate::sim::options::SimulationOption;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum YamlElement {
    Resistor(YamlResistor),
}

#[derive(Deserialize)]
pub struct YamlSimulation {}

pub struct YamlFrontend {}

impl YamlFrontend {
    pub fn new_from_path(path: String) -> Self {
        let mut commands: Vec<SimulationCommand> = Vec::new();
        let mut options:Vec<SimulationOption> = Vec::new();
        let mut elements: Vec<Element> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map:HashMap<Arc<str>, usize> = HashMap::new();
        
        
        
        Self {}
    }

    pub fn new_from_string(yaml_string: String) -> Self {
        Self {}
    }

    fn parse_element(
        elem: YamlElement,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Result<Element, FrontendError> {
        match elem {
            YamlElement::Resistor(r) => Ok(r.into()),
        }
    }

    fn parse_yaml(&self) -> Result<Vec<Element>, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}

impl Frontend for YamlFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        let elements = self.parse_yaml()?;
        Ok(Simulation {
            commands: Vec::new(),
            options: Vec::new(),
            elements,
            variables: Vec::new(),
        })
    }
}
