use std::sync::Arc;

use serde::Deserialize;

use crate::{
    frontends::{get_variable, yaml::ProcessYamlElement},
    models::{Element, Mos0Bundle, Unit},
    spot::Numeric,
};

#[derive(Debug, Deserialize)]
pub struct YamlMos0 {
    pub name: String,
    pub gate: String,
    pub drain: String,
    pub source: String,
}

impl ProcessYamlElement for YamlMos0 {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let name = self.name.as_str();
        let ele = Mos0Bundle::new(
            Arc::from(name),
            get_variable(&self.gate, Unit::Volt, variables, var_map),
            get_variable(&self.drain, Unit::Volt, variables, var_map),
            get_variable(&self.source, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Mos0(ele));
    }
}
