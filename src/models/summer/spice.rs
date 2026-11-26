// SPICE parsing for summer blocks
// Syntax: .SUMMER name output input1 input2 ... [GAINS gain1 gain2 ...]
// Note: Summers are behavioral blocks, not standard SPICE elements

use crate::frontends::get_variable;
use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, SummerBundle, Unit};
use crate::spot::Numeric;
use crate::{FrontendError, Variable};
use pest::iterators::Pair;
use std::sync::Arc;

impl ProcessSpiceElement for SummerBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<crate::Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);

        // Parse name
        let name = parser.parse_name("summer block")?;

        // Parse output node first
        let output_str = parser.parse_node("summer block", name, "output")?;
        let output = get_variable(output_str, Unit::Volt, variables, var_map);

        // Parse all remaining values as strings
        let remaining = parser.parse_remaining_values();

        if remaining.is_empty() {
            return Err(FrontendError::ParseError(format!(
                "Summer block '{}' requires at least one input",
                name
            )));
        }

        // Check if we have a GAINS keyword
        let (input_strs, gain_strs) = if let Some(pos) = remaining
            .iter()
            .position(|&s| s.eq_ignore_ascii_case("GAINS"))
        {
            // Split at GAINS keyword
            (&remaining[..pos], &remaining[pos + 1..])
        } else {
            // No GAINS keyword - all are inputs, use default gains
            (&remaining[..], &[][..])
        };

        // Create input variables (filter out ground nodes)
        let inputs: Vec<_> = input_strs
            .iter()
            .filter_map(|input_str| get_variable(input_str, Unit::Volt, variables, var_map))
            .collect();

        // Parse gains if provided
        let gains = if !gain_strs.is_empty() {
            if gain_strs.len() != inputs.len() {
                return Err(FrontendError::ParseError(format!(
                    "Summer block '{}' has {} inputs but {} gains provided",
                    name,
                    inputs.len(),
                    gain_strs.len()
                )));
            }
            gain_strs
                .iter()
                .map(|s| {
                    s.parse::<Numeric>().map_err(|_| {
                        FrontendError::ParseError(format!(
                            "Invalid gain '{}' in summer block '{}'",
                            s, name
                        ))
                    })
                })
                .collect::<Result<Vec<_>, FrontendError>>()?
        } else {
            // Default gains of 1.0
            vec![1.0; inputs.len()]
        };

        // Create the summer block
        let summer = SummerBundle::new(Arc::from(name), inputs, output, Some(gains));

        elements.push(Element::Summer(summer));
        Ok(())
    }
}
