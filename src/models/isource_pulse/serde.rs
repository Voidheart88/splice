// Serde serialization/deserialization for ISourcePulseBundle

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::serde::ProcessSerdeElement;
use crate::models::{Element, ISourcePulseBundle, Unit};
use crate::spot::Numeric;

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeISourcePulse {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub initial_value: Numeric,
    pub pulsed_value: Numeric,
    pub delay_time: Numeric,
    pub rise_time: Numeric,
    pub fall_time: Numeric,
    pub pulse_width: Numeric,
    pub period: Numeric,
}

impl ProcessSerdeElement for SerdeISourcePulse {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) {
        let name = Arc::from(self.name.as_str());
        let node0 = get_variable(&self.node0, Unit::Ampere, variables, var_map);
        let node1 = get_variable(&self.node1, Unit::Ampere, variables, var_map);

        let isource = ISourcePulseBundle::new(
            name,
            node0,
            node1,
            self.initial_value,
            self.pulsed_value,
            self.delay_time,
            self.rise_time,
            self.fall_time,
            self.pulse_width,
            self.period,
        );

        elements.push(Element::ISourcePulse(isource));
    }
}
