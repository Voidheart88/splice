// SPICE parsing for pulse voltage sources
// Syntax: Vxxx n+ n- PULSE(initial pulsed delay risetime falltime pulsewidth period)

use crate::frontends::get_variable;
use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::frontends::spice_parser_helpers::SpiceElementParser;
use crate::models::{Element, Unit, VSourcePulseBundle};
use crate::{FrontendError, Variable};
use pest::iterators::Pair;
use std::sync::Arc;

impl ProcessSpiceElement for VSourcePulseBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<crate::Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Use the helper parser for common parsing logic
        let mut parser = SpiceElementParser::new(element);

        // Parse name and nodes
        let name = parser.parse_name("pulse voltage source")?;
        let node0_str = parser.parse_node("pulse voltage source", name, "node0")?;
        let node1_str = parser.parse_node("pulse voltage source", name, "node1")?;

        // Parse pulse parameters
        let initial_value = parser.parse_value("pulse voltage source", name, "initial_value")?;
        let pulsed_value = parser.parse_value("pulse voltage source", name, "pulsed_value")?;
        let delay_time = parser.parse_value("pulse voltage source", name, "delay_time")?;
        let rise_time = parser.parse_value("pulse voltage source", name, "rise_time")?;
        let fall_time = parser.parse_value("pulse voltage source", name, "fall_time")?;
        let pulse_width = parser.parse_value("pulse voltage source", name, "pulse_width")?;
        let period = parser.parse_value("pulse voltage source", name, "period")?;

        // Create branch and node variables
        let branch = get_variable(
            &format!("{}#branch", name),
            Unit::Ampere,
            variables,
            var_map,
        )
        .expect("Failed to create branch variable for VSourcePulse");
        let node0 = get_variable(node0_str, Unit::Volt, variables, var_map);
        let node1 = get_variable(node1_str, Unit::Volt, variables, var_map);

        // Create the pulse voltage source (ac_value not parsed from SPICE)
        let vsource = VSourcePulseBundle::new(
            Arc::from(name),
            branch,
            node0,
            node1,
            initial_value,
            pulsed_value,
            delay_time,
            rise_time,
            fall_time,
            pulse_width,
            period,
            None,
        );

        elements.push(Element::VSourcePulse(vsource));
        Ok(())
    }
}
