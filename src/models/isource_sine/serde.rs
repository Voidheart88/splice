// Serde serialization/deserialization for ISourceSinBundle

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::serde::ProcessSerdeElement;
use crate::models::{Element, ISourceSinBundle, Unit};
use crate::spot::Numeric;

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeISourceSin {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub dc_offset: Numeric,
    pub amplitude: Numeric,
    pub frequency: Numeric,
    pub phase: Numeric,
    pub ac_value: Option<Numeric>,
}

impl ProcessSerdeElement for SerdeISourceSin {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) {
        let name = Arc::from(self.name.as_str());
        let node0 = get_variable(&self.node0, Unit::Ampere, variables, var_map);
        let node1 = get_variable(&self.node1, Unit::Ampere, variables, var_map);

        let isource = ISourceSinBundle::new(
            name,
            node0,
            node1,
            self.dc_offset,
            self.amplitude,
            self.frequency,
            self.phase,
            self.ac_value,
        );

        elements.push(Element::ISourceSin(isource));
    }
}
