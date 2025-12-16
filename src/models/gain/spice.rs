use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, GainBundle, Unit};

impl ProcessSpiceElement for GainBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);
        
        // Parse the name (without the leading 'A') - special case for gain blocks
        let remaining = parser.parse_remaining_values();
        if remaining.is_empty() {
            return Err(crate::frontends::FrontendError::ParseError(
                "Missing values in gain block".to_string()
            ));
        }
        let name = &remaining[0][1..]; // Skip the leading 'A'

        // Parse remaining values manually since we already consumed them
        if remaining.len() < 3 {
            return Err(crate::frontends::FrontendError::ParseError(
                format!("Insufficient values in gain block '{}': expected input_node, output_node, value", name)
            ));
        }
        let input_node_str = remaining[1];
        let output_node_str = remaining[2];
        let value = remaining[3].parse::<f64>()
            .map_err(|_| crate::frontends::FrontendError::ParseError(
                format!("Invalid gain value in gain block '{}': must be a number", name)
            ))?;

        // Create the input and output variables
        let input_var = get_variable(input_node_str, Unit::Volt, variables, var_map);
        let output_var = get_variable(output_node_str, Unit::Volt, variables, var_map);

        // Create the GainBundle
        let gain = GainBundle::new(Arc::from(name), input_var, output_var, value);

        // Add the GainBundle as element
        elements.push(Element::Gain(gain));
        Ok(())
    }
}
