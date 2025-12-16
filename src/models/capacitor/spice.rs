use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{CapacitorBundle, Element, Unit};

use std::sync::Arc;

impl ProcessSpiceElement for CapacitorBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);
        
        // Parse using the abstracted helper methods
        let name = parser.parse_name("capacitor")?;
        let node0 = parser.parse_node("capacitor", name, "node0")?;
        let node1 = parser.parse_node("capacitor", name, "node1")?;
        let value = parser.parse_value("capacitor", name, "value")?;

        // Create the capacitor element
        let cap = CapacitorBundle::new(
            Arc::from(name),
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );
        elements.push(Element::Capacitor(cap));
        Ok(())
    }
}
