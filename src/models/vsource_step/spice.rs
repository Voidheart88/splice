/// The Step Source as Spice file:
/// <Name> <Node0> <Node1> step <initial_value> <final_value> <step_time>
use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, Unit, VSourceStepBundle, Variable};

impl ProcessSpiceElement for VSourceStepBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();

        let name = inner
            .next()
            .ok_or_else(|| {
                // Fixme: replace with use
                crate::frontends::FrontendError::ParseError(format!(
                    "Missing name in step voltage source: {}",
                    ele
                ))
            })?
            .as_str();

        let node0 = inner
            .next()
            .ok_or_else(|| {
                // Fixme: replace with use
                crate::frontends::FrontendError::ParseError(format!(
                    "Missing node0 in step voltage source: {}",
                    name
                ))
            })?
            .as_span();
        let node0_str = &ele[node0.start() - offset..node0.end() - offset];

        let node1 = inner
            .next()
            .ok_or_else(|| {
                // Fixme: replace with use
                crate::frontends::FrontendError::ParseError(format!(
                    "Missing node1 in step voltage source: {}",
                    name
                ))
            })?
            .as_span();
        let node1_str = &ele[node1.start() - offset..node1.end() - offset];

        // Parse the 3 values (initial_value, final_value, step_time)
        let mut values = Vec::new();
        for pair in inner {
            values.push(pair.as_str());
        }

        // Ensure we have exactly 3 values
        if values.len() < 3 {
            // Fixme: replace with use
            return Err(crate::frontends::FrontendError::ParseError(
                format!("Insufficient values for step voltage source '{}': expected 3 values (initial_value, final_value, step_time)", name)
            ));
        }

        let initial_value = values[0].parse::<f64>().map_err(|_| {
            // Fixme: replace with use
            crate::frontends::FrontendError::ParseError(format!(
                "Invalid initial value in step voltage source '{}': must be a number",
                name
            ))
        })?;

        let final_value = values[1].parse::<f64>().map_err(|_| {
            // Fixme: replace with use
            crate::frontends::FrontendError::ParseError(format!(
                "Invalid final value in step voltage source '{}': must be a number",
                name
            ))
        })?;

        let step_time = values[2].parse::<f64>().map_err(|_| {
            // Fixme: replace with use
            crate::frontends::FrontendError::ParseError(format!(
                "Invalid step time in step voltage source '{}': must be a number",
                name
            ))
        })?;

        let branch = Variable::new(
            Arc::from(format!("branch_{}", name)),
            Unit::Ampere,
            variables.len(),
        );
        variables.push(branch.clone());

        let node0_var = get_variable(node0_str, Unit::Volt, variables, var_map);
        let node1_var = get_variable(node1_str, Unit::Volt, variables, var_map);

        let vsource_step = VSourceStepBundle::new(
            Arc::from(name),
            branch,
            node0_var,
            node1_var,
            initial_value,
            final_value,
            step_time,
            None,
        );

        elements.push(Element::VSourceStep(vsource_step));
        Ok(())
    }
}
