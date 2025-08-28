use std::sync::Arc;

use serde::Deserialize;

use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{Element, InductorBundle, Unit};
use crate::spot::*;

#[derive(Debug, Deserialize)]
pub struct SerdeInductor {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub value: Numeric,
}

impl ProcessSerdeElement for SerdeInductor {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let res = InductorBundle::new(
            Arc::from(self.name.as_str()),
            get_variable(self.node0.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.node1.as_str(), Unit::Volt, variables, var_map),
            self.value,
        );
        elements.push(Element::Inductor(res));
    }
}
