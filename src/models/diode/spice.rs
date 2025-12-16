use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{DiodeBundle, Element, Unit};

impl ProcessSpiceElement for DiodeBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);
        
        // Parse using the abstracted helper methods
        let name = parser.parse_name("diode")?;
        let node0 = parser.parse_node("diode", name, "node0")?;
        let node1 = parser.parse_node("diode", name, "node1")?;

        // Create the diode element
        let dio = DiodeBundle::new(
            Arc::from(name),
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Diode(dio));
        Ok(())
    }
}
