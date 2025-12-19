use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::frontends::{get_variable, serde::ProcessSerdeElement};
use crate::models::{Element, ResistorBundle, Unit};
use crate::spot::Numeric;

/// Serde-compatible resistor structure for JSON/YAML serialization.
///
/// This structure represents a resistor in a format suitable for serialization
/// and deserialization using Serde. It's used for reading and writing circuit
/// descriptions in JSON and YAML formats.
#[derive(Debug, Deserialize, Serialize)]
pub struct SerdeResistor {
    /// Name of the resistor component
    pub name: String,
    /// First node connection point
    pub node0: String,
    /// Second node connection point
    pub node1: String,
    /// Resistance value in ohms
    pub value: Numeric,
}

impl ProcessSerdeElement for SerdeResistor {
    /// Processes a Serde resistor and converts it to the internal resistor representation.
    ///
    /// This method takes the serialized resistor data and creates the corresponding
    /// internal resistor bundle with proper variable references for the circuit simulation.
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
        let res = ResistorBundle::new(
            Arc::from(self.name.as_str()),
            get_variable(self.node0.as_str(), Unit::Volt, variables, var_map),
            get_variable(self.node1.as_str(), Unit::Volt, variables, var_map),
            self.value,
        );
        elements.push(Element::Resistor(res));
    }
}
