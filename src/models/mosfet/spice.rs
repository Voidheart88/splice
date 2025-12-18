use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, Mos0Bundle, Unit};

impl ProcessSpiceElement for Mos0Bundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);

        // Parse using the abstracted helper methods
        let name = parser.parse_name("MOSFET")?;
        let gate_node = parser.parse_node("MOSFET", name, "gate node")?;
        let drain_node = parser.parse_node("MOSFET", name, "drain node")?;
        let source_node = parser.parse_node("MOSFET", name, "source node")?;

        // Create the MOSFET element
        let mosfet = Mos0Bundle::new(
            Arc::from(name),
            get_variable(gate_node, Unit::Volt, variables, var_map),
            get_variable(drain_node, Unit::Volt, variables, var_map),
            get_variable(source_node, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Mos0(mosfet));
        Ok(())
    }
}
