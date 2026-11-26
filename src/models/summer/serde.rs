// Serde serialization/deserialization for SummerBundle

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::serde::ProcessSerdeElement;
use crate::models::{Element, SummerBundle, Unit};
use crate::spot::Numeric;

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeSummer {
    pub name: String,
    pub inputs: Vec<String>,
    pub output: String,
    pub gains: Option<Vec<Numeric>>,
}

impl ProcessSerdeElement for SerdeSummer {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) {
        let name = Arc::from(self.name.as_str());

        // Create input variables (filter out ground nodes)
        let inputs: Vec<_> = self
            .inputs
            .iter()
            .filter_map(|input| get_variable(input, Unit::Volt, variables, var_map))
            .collect();

        let output = get_variable(&self.output, Unit::Volt, variables, var_map);

        let summer = SummerBundle::new(name, inputs, output, self.gains.clone());

        elements.push(Element::Summer(summer));
    }
}
