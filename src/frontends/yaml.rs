use std::collections::HashMap;
use std::sync::Arc;

use super::{Element, Frontend, FrontendError, Simulation};
use crate::models::resistor::yaml::YamlResistor;
use crate::models::{Unit, Variable};
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

    fn get_variable(
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
