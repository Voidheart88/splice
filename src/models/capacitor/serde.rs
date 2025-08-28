use std::sync::Arc;

use serde::Deserialize;

use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{CapacitorBundle, Element, Unit};
use crate::spot::Numeric;

#[derive(Debug, Deserialize)]
pub struct SerdeCapacitor {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub value: Numeric,
}

impl ProcessSerdeElement for SerdeCapacitor {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let res = CapacitorBundle::new(
            Arc::from(self.name.as_str()),
            get_variable(self.node0.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.node1.as_str(), Unit::Volt, variables, var_map),
            self.value,
        );
        elements.push(Element::Capacitor(res));
    }
}
