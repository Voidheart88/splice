/// The Sine Source  as Spice file:
/// <Name> <Node0> <Node1> sin[e] <dc_offset> <amplitude> <frequency> <phase>
use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, Unit, VSourceSinBundle, Variable};

impl ProcessSpiceElement for VSourceSinBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();

        let name = inner.next().unwrap().as_str();

        let node0 = inner.next().unwrap().as_span();
        let node0_str = &ele[node0.start() - offset..node0.end() - offset];

        let node1 = inner.next().unwrap().as_span();
        let node1_str = &ele[node1.start() - offset..node1.end() - offset];

        // Parse the 3 or 4 values (offset, amplitude, frequency, phase)
        let mut values = Vec::new();
        for pair in inner {
            values.push(pair.as_str());
        }

        // Ensure we have at least 3 values
        if values.len() < 3 {
            panic!("Insufficient values for VSourceSin");
        }

        let dc_offset = values[0].parse::<f64>().unwrap();
        let amplitude = values[1].parse::<f64>().unwrap();
        let frequency = values[2].parse::<f64>().unwrap();

        // Optional phase value
        let phase = match values.get(3) {
            Some(phase_str) => phase_str.parse::<f64>().unwrap(),
            None => 0.0, // Default value when not specified
        };

        let branch = Variable::new(
            Arc::from(format!("branch_{}", name)),
            Unit::Ampere,
            variables.len(),
        );
        variables.push(branch.clone());

        let node0_var = get_variable(node0_str, Unit::Volt, variables, var_map);
        let node1_var = get_variable(node1_str, Unit::Volt, variables, var_map);

        let vsource_sin = VSourceSinBundle::new(
            Arc::from(name),
            branch,
            node0_var,
            node1_var,
            dc_offset,
            amplitude,
            frequency,
            phase,
            None,
        );

        elements.push(Element::VSourceSin(vsource_sin));
    }
}
