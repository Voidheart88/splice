use std::sync::Arc;

use serde::Deserialize;

use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{Element, ISourceBundle, Unit};
use crate::spot::*;

#[derive(Debug, Deserialize)]
pub struct SerdeISource {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub value: Numeric,
}

impl ProcessSerdeElement for SerdeISource {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let name = self.name.as_str();
        let ele = ISourceBundle::new(
            Arc::from(name),
            get_variable(&self.node0, Unit::Volt, variables, var_map),
            get_variable(&self.node1, Unit::Volt, variables, var_map),
            self.value,
        );
        elements.push(Element::ISource(ele));
    }
}
