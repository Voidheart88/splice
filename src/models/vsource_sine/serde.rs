use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{Element, Unit, VSourceSinBundle, Variable};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SerdeVSourceSin {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub dc_offset: f64,
    pub amplitude: f64,
    pub frequency: f64,
    pub phase: f64,
    pub ac_value: Option<f64>,
}

impl ProcessSerdeElement for SerdeVSourceSin {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let branch = Variable::new(
            Arc::from(format!("branch_{}", self.name)),
            Unit::Ampere,
            variables.len(),
        );
        variables.push(branch.clone());

        let node0 = get_variable(self.node0.as_str(), Unit::Volt, variables, var_map);
        let node1 = get_variable(self.node1.as_str(), Unit::Volt, variables, var_map);

        let vsource_sin = VSourceSinBundle::new(
            Arc::from(self.name.as_str()),
            branch,
            node0,
            node1,
            self.dc_offset,
            self.amplitude,
            self.frequency,
            self.phase,
            self.ac_value,
        );

        elements.push(Element::VSourceSin(vsource_sin));
    }
}
