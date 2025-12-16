use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, ResistorBundle, Unit};
use crate::spot::*;

impl ProcessSpiceElement for ResistorBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);
        
        // Parse using the abstracted helper methods
        let name = parser.parse_name("resistor")?;
        let node0 = parser.parse_node("resistor", name, "node0")?;
        let node1 = parser.parse_node("resistor", name, "node1")?;
        let value = parser.parse_value("resistor", name, "value")?;

        // Create the resistor element
        let res = ResistorBundle::new(
            Arc::from(name),
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );
        elements.push(Element::Resistor(res));
        Ok(())
    }
}
