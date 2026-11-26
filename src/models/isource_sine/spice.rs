use crate::frontends::get_variable;
/// SPICE parsing for sinusoidal current sources
/// Syntax: Ixxx n+ n- SIN(dc_offset amplitude frequency [phase])
use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, ISourceSinBundle, Unit};
use crate::{FrontendError, Variable};
use pest::iterators::Pair;
use std::sync::Arc;

impl ProcessSpiceElement for ISourceSinBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<crate::Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);

        // Parse name and nodes
        let name = parser.parse_name("sinusoidal current source")?;
        let node0_str = parser.parse_node("sinusoidal current source", name, "node0")?;
        let node1_str = parser.parse_node("sinusoidal current source", name, "node1")?;

        // Parse sine parameters: SIN(dc_offset amplitude frequency [phase])
        let dc_offset = parser.parse_value("sinusoidal current source", name, "dc_offset")?;
        let amplitude = parser.parse_value("sinusoidal current source", name, "amplitude")?;
        let frequency = parser.parse_value("sinusoidal current source", name, "frequency")?;
        // Phase is optional, default to 0
        let phase =
            parser.parse_value_or_default("sinusoidal current source", name, "phase", 0.0)?;

        // Create node variables
        let node0 = get_variable(node0_str, Unit::Ampere, variables, var_map);
        let node1 = get_variable(node1_str, Unit::Ampere, variables, var_map);

        // Create the sinusoidal current source (ac_value not parsed from SPICE)
        let isource = ISourceSinBundle::new(
            Arc::from(name),
            node0,
            node1,
            dc_offset,
            amplitude,
            frequency,
            phase,
            None,
        );

        elements.push(Element::ISourceSin(isource));
        Ok(())
    }
}
