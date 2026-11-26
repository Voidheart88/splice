// SPICE parsing for integrator blocks
// Syntax: .INTEGRATOR name input output [gain] [ic]
// Note: Integrators are behavioral blocks, not standard SPICE elements

use crate::frontends::get_variable;
use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, IntegratorBundle, Unit};
use crate::{FrontendError, Variable};
use pest::iterators::Pair;
use std::sync::Arc;

impl ProcessSpiceElement for IntegratorBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<crate::Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);

        // Parse name and nodes
        let name = parser.parse_name("integrator block")?;
        let input_str = parser.parse_node("integrator block", name, "input")?;
        let output_str = parser.parse_node("integrator block", name, "output")?;

        // Parse optional parameters with defaults
        let gain = parser.parse_value_or_default("integrator block", name, "gain", 1.0)?;
        let initial_condition =
            parser.parse_value_or_default("integrator block", name, "initial_condition", 0.0)?;

        // Create variables
        let input = get_variable(input_str, Unit::Volt, variables, var_map);
        let output = get_variable(output_str, Unit::Volt, variables, var_map);

        // Create the integrator block
        let integrator =
            IntegratorBundle::new(Arc::from(name), input, output, gain, initial_condition);

        elements.push(Element::Integrator(integrator));
        Ok(())
    }
}
