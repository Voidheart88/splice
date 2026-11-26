// SPICE parsing for step current sources
// Syntax: Ixxx n+ n- PULSE(initial final delay)

use crate::frontends::get_variable;
use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, ISourceStepBundle, Unit};
use crate::{FrontendError, Variable};
use pest::iterators::Pair;
use std::sync::Arc;

impl ProcessSpiceElement for ISourceStepBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<crate::Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);

        // Parse name and nodes
        let name = parser.parse_name("step current source")?;
        let node0_str = parser.parse_node("step current source", name, "node0")?;
        let node1_str = parser.parse_node("step current source", name, "node1")?;

        // Parse step parameters
        let initial_value = parser.parse_value("step current source", name, "initial_value")?;
        let final_value = parser.parse_value("step current source", name, "final_value")?;
        let delay_time = parser.parse_value("step current source", name, "delay_time")?;

        // Create node variables
        let node0 = get_variable(node0_str, Unit::Ampere, variables, var_map);
        let node1 = get_variable(node1_str, Unit::Ampere, variables, var_map);

        // Create the step current source
        let isource = ISourceStepBundle::new(
            Arc::from(name),
            node0,
            node1,
            initial_value,
            final_value,
            delay_time,
        );

        elements.push(Element::ISourceStep(isource));
        Ok(())
    }
}
