use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{CapacitorBundle, Element, Unit};
use crate::spot::Numeric;

/// Serde-compatible capacitor structure for JSON/YAML serialization.
///
/// This structure represents a capacitor in a format suitable for serialization
/// and deserialization using Serde. It's used for reading and writing circuit
/// descriptions in JSON and YAML formats.
#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeCapacitor {
    /// Name of the capacitor component
    pub name: String,
    /// First node connection point
    pub node0: String,
    /// Second node connection point
    pub node1: String,
    /// Capacitance value in farads
    pub value: Numeric,
}

impl ProcessSerdeElement for SerdeCapacitor {
    /// Processes a Serde capacitor and converts it to the internal capacitor representation.
    ///
    /// This method takes the serialized capacitor data and creates the corresponding
    /// internal capacitor bundle with proper variable references for the circuit simulation.
    ///
    /// # Arguments
    ///
    /// * `variables` - Vector of circuit variables (nodes, voltages, etc.)
    /// * `elements` - Vector to store the created circuit elements
    /// * `var_map` - HashMap mapping variable names to indices
    fn process(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let res = CapacitorBundle::new(
            Arc::from(self.name.as_str()),
            get_variable(self.node0.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.node1.as_str(), Unit::Volt, variables, var_map),
            self.value,
        );
        elements.push(Element::Capacitor(res));
    }
}
