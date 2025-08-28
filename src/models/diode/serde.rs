/// The Diode - yaml parsing module

use std::sync::Arc;
use std::collections::HashMap;

use serde::Deserialize;

use crate::models::{DiodeBundle, Element, Unit};
use crate::frontends::{get_variable, serde::ProcessSerdeElement};

#[derive(Debug, Deserialize)]
pub struct SerdeDiode {
    pub name: String,
    pub anode: String,
    pub cathode: String,
}

impl ProcessSerdeElement for SerdeDiode {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<std::sync::Arc<str>, usize>,
    ) {
        let res = DiodeBundle::new(
            Arc::from(self.name.as_str()),
            get_variable(self.anode.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.cathode.as_str(), Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Diode(res));
    }
}
