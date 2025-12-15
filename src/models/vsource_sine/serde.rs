use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{Element, Unit, VSourceSinBundle, Variable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeVSourceSin {
    /// Name of the sinusoidal voltage source.
    pub name: String,
    /// Node 0 of the sinusoidal voltage source.
    pub node0: String,
    /// Node 1 of the sinusoidal voltage source.
    pub node1: String,
    /// DC offset of the sinusoidal voltage source.
    #[serde(rename = "dc-offset")]
    pub dc_offset: f64,
    /// Amplitude of the sinusoidal voltage source.
    pub amplitude: f64,
    /// Frequency of the sinusoidal voltage source.
    pub frequency: f64,
    /// Phase of the sinusoidal voltage source.
    pub phase: f64,
    /// AC value of the sinusoidal voltage source.
    #[serde(rename = "ac-value")]
    pub ac_value: Option<f64>,
}

impl ProcessSerdeElement for SerdeVSourceSin {
    fn process(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let branch = Variable::new(
            Arc::from(format!("branch_{}", self.name)),
            Unit::Ampere,
            variables.len(),
        );
        variables.push(branch.clone());

        let node0 = get_variable(self.node0.as_str(), Unit::Volt, variables, var_map);
        let node1 = get_variable(self.node1.as_str(), Unit::Volt, variables, var_map);

        let vsource_sin = VSourceSinBundle::new(
            Arc::from(self.name.as_str()),
            branch,
            node0,
            node1,
            self.dc_offset,
            self.amplitude,
            self.frequency,
            self.phase,
            self.ac_value,
        );

        elements.push(Element::VSourceSin(vsource_sin));
    }
}
