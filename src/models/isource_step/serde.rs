// Serde serialization/deserialization for ISourceStepBundle

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::serde::ProcessSerdeElement;
use crate::models::{Element, ISourceStepBundle, Unit};
use crate::spot::Numeric;

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeISourceStep {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub initial_value: Numeric,
    pub final_value: Numeric,
    pub delay_time: Numeric,
}

impl ProcessSerdeElement for SerdeISourceStep {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) {
        let name = Arc::from(self.name.as_str());
        let node0 = get_variable(&self.node0, Unit::Ampere, variables, var_map);
        let node1 = get_variable(&self.node1, Unit::Ampere, variables, var_map);

        let isource = ISourceStepBundle::new(
            name,
            node0,
            node1,
            self.initial_value,
            self.final_value,
            self.delay_time,
        );

        elements.push(Element::ISourceStep(isource));
    }
}
