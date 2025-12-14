use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::models::{Element, Unit, VSourceStepBundle, Variable};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SerdeVSourceStep {
    pub name: String,
    pub node0: Option<String>,
    pub node1: Option<String>,
    pub initial_value: f64,
    pub final_value: f64,
    pub step_time: f64,
    pub ac_value: Option<f64>,
}

impl From<SerdeVSourceStep> for Element {
    fn from(value: SerdeVSourceStep) -> Self {
        let branch = Variable::new(
            Arc::from(format!("branch_{}", value.name)),
            Unit::Ampere,
            0, // Will be updated during processing
        );
        
        let node0 = value.node0.map(|n| Variable::new(Arc::from(n), Unit::Volt, 0));
        let node1 = value.node1.map(|n| Variable::new(Arc::from(n), Unit::Volt, 0));
        
        Element::VSourceStep(VSourceStepBundle::new(
            Arc::from(value.name),
            branch,
            node0,
            node1,
            value.initial_value,
            value.final_value,
            value.step_time,
            value.ac_value,
        ))
    }
}

impl From<VSourceStepBundle> for SerdeVSourceStep {
    fn from(value: VSourceStepBundle) -> Self {
        SerdeVSourceStep {
            name: value.name().to_string(),
            node0: value.node0.map(|v| v.name().to_string()),
            node1: value.node1.map(|v| v.name().to_string()),
            initial_value: value.initial_value,
            final_value: value.final_value,
            step_time: value.step_time,
            ac_value: None, // AC value not stored in bundle
        }
    }
}

use std::collections::HashMap;
use crate::frontends::get_variable;
use crate::frontends::serde::ProcessSerdeElement;

impl ProcessSerdeElement for SerdeVSourceStep {
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

        let node0 = get_variable(self.node0.as_deref().unwrap_or(""), Unit::Volt, variables, var_map);
        let node1 = get_variable(self.node1.as_deref().unwrap_or(""), Unit::Volt, variables, var_map);

        let vsource_step = VSourceStepBundle::new(
            Arc::from(self.name.as_str()),
            branch,
            node0,
            node1,
            self.initial_value,
            self.final_value,
            self.step_time,
            self.ac_value,
        );

        elements.push(Element::VSourceStep(vsource_step));
    }
}