/// The BJT - yaml parsing module
use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{Bjt0Bundle, Bjt1Bundle, Element, Unit};

/// BJT0Options for serialization
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bjt0OptionsSerde {
    pub is: f64,
    pub n: f64,
    pub beta: f64,
}

/// BJT1Options for serialization
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bjt1OptionsSerde {
    pub is: f64,
    pub n: f64,
    pub beta: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeBjt0 {
    pub name: String,
    pub base: String,
    pub collector: String,
    pub emitter: String,
    pub options: Option<Bjt0OptionsSerde>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeBjt1 {
    pub name: String,
    pub base: String,
    pub collector: String,
    pub emitter: String,
    pub options: Option<Bjt1OptionsSerde>,
}

impl ProcessSerdeElement for SerdeBjt0 {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let options = self.options.as_ref().map(|opts| super::bjt0::Bjt0Options {
            is: opts.is,
            n: opts.n,
            beta: opts.beta,
        });

        let bjt = Bjt0Bundle::new(
            Arc::from(self.name.as_str()),
            get_variable(self.base.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.collector.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.emitter.as_str(), Unit::Volt, variables, var_map),
            options,
        );
        elements.push(Element::Bjt0(bjt));
    }
}

impl ProcessSerdeElement for SerdeBjt1 {
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let options = self.options.as_ref().map(|opts| super::bjt1::Bjt1Options {
            is: opts.is,
            n: opts.n,
            beta: opts.beta,
        });

        let bjt = Bjt1Bundle::new(
            Arc::from(self.name.as_str()),
            get_variable(self.base.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.collector.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.emitter.as_str(), Unit::Volt, variables, var_map),
            options,
        );
        elements.push(Element::Bjt1(bjt));
    }
}
