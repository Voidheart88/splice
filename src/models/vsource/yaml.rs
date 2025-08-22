use std::sync::Arc;

use serde::Deserialize;

use crate::{
    frontends::{get_variable, yaml::ProcessYamlElement},
    models::{Element, Unit, VSourceBundle},
    spot::Numeric,
};

#[derive(Debug, Deserialize)]
pub struct YamlVSource {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub value: Numeric,
    pub ac_value: Option<Numeric>,
}

impl ProcessYamlElement for YamlVSource {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let name = self.name.as_str();
        let ele = VSourceBundle::new(
            Arc::from(name),
            get_variable(&format!("{name}#branch"), Unit::Ampere, variables, var_map).unwrap(),
            get_variable(&self.node0, Unit::Volt, variables, var_map),
            get_variable(&self.node1, Unit::Volt, variables, var_map),
            self.value,
            self.ac_value,
        );
        elements.push(Element::VSource(ele));
    }
}
