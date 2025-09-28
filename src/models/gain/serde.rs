use std::collections::HashMap;
use std::sync::Arc;
use serde::Deserialize;
use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{GainBundle, Element, Unit};

/// A structure for deserializing a Gain from YAML or other formats.
#[derive(Debug, Deserialize)]
pub struct SerdeGain {
    pub name: String,
    pub input: String,
    pub output: String,
    pub value: f64,
}

impl ProcessSerdeElement for SerdeGain {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let input_var = get_variable(
            self.input.as_str(),
            Unit::Volt,
            variables,
            var_map,
        );
        let output_var = get_variable(
            self.output.as_str(),
            Unit::Volt,
            variables,
            var_map,
        );

        let gain = GainBundle::new(
            Arc::from(self.name.as_str()),
            input_var,
            output_var,
            self.value,
        );

        elements.push(Element::Gain(gain));
    }
}
