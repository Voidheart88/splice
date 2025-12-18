use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, Unit, VSourceBundle};
use crate::spot::*;

impl ProcessSpiceElement for VSourceBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);

        // Parse using the abstracted helper methods
        let name = parser.parse_name("voltage source")?;
        let node0 = parser.parse_node("voltage source", name, "node0")?;
        let node1 = parser.parse_node("voltage source", name, "node1")?;
        let value = parser.parse_value("voltage source", name, "value")?;

        // Parse optional AC value using the helper's remaining values
        let remaining = parser.parse_remaining_values();
        let ac_value = if !remaining.is_empty() {
            let ac_val_str = remaining[0].split(" ").nth(1).ok_or_else(|| {
                // Fixme: replace with use because this is too long
                crate::frontends::FrontendError::ParseError(format!(
                    "Missing AC value in voltage source: {}",
                    name
                ))
            })?;
            let ac_val = ac_val_str.parse::<Numeric>().map_err(|_| {
                // Fixme: replace with use because this is too long
                crate::frontends::FrontendError::ParseError(format!(
                    "Invalid AC value in voltage source '{}': must be a number",
                    name
                ))
            })?;
            Some(ac_val)
        } else {
            None
        };

        // Create branch variable
        let branch_var = get_variable(&format!("{name}#branch"), Unit::Ampere, variables, var_map)
            .ok_or_else(|| {
                // Fixme: replace with use because this is too long
                crate::frontends::FrontendError::ParseError(format!(
                    "Failed to create branch variable for voltage source: {}",
                    name
                ))
            })?;

        let src = VSourceBundle::new(
            Arc::from(name),
            branch_var,
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            value,
            ac_value,
        );

        elements.push(Element::VSource(src));
        Ok(())
    }
}
