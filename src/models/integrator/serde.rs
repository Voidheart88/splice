// Serde serialization/deserialization for IntegratorBundle

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::serde::ProcessSerdeElement;
use crate::models::{Element, IntegratorBundle, Unit};
use crate::spot::Numeric;

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeIntegrator {
    pub name: String,
    pub input: String,
    pub output: String,
    pub gain: Numeric,
    pub initial_condition: Numeric,
}

impl ProcessSerdeElement for SerdeIntegrator {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) {
        let name = Arc::from(self.name.as_str());
        let input = get_variable(&self.input, Unit::Volt, variables, var_map);
        let output = get_variable(&self.output, Unit::Volt, variables, var_map);

        let integrator =
            IntegratorBundle::new(name, input, output, self.gain, self.initial_condition);

        elements.push(Element::Integrator(integrator));
    }
}
