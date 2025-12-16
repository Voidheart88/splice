use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, Unit, VSourceBundle};
use crate::spot::*;

impl ProcessSpiceElement for VSourceBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) -> Result<(), crate::frontends::FrontendError> {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        
        //extract Name
        let name_end = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing name in voltage source: {}", ele)
            ))?
            .as_span().end() - offset;
        let name = &ele[0..name_end];

        //extract Node0
        let node0_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing node0 in voltage source: {}", name)
            ))?
            .as_span();
        let node0 = &ele[node0_span.start() - offset..node0_span.end() - offset];

        //extract Node1
        let node1_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing node1 in voltage source: {}", name)
            ))?
            .as_span();
        let node1 = &ele[node1_span.start() - offset..node1_span.end() - offset];

        //extract Value
        let value_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing value in voltage source: {}", name)
            ))?
            .as_span();
        let value = ele[value_span.start() - offset..value_span.end() - offset]
            .parse::<Numeric>()
            .map_err(|_| crate::frontends::FrontendError::ParseError(
                format!("Invalid value in voltage source '{}': must be a number", name)
            ))?;

        //extract AC value (optional)
        let ac_value = if let Some(val) = inner.next() {
            let ac_val_str = val.as_str().split(" ")
                .nth(1)
                .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                    format!("Missing AC value in voltage source: {}", name)
                ))?;
            let ac_val = ac_val_str.parse::<Numeric>()
                .map_err(|_| crate::frontends::FrontendError::ParseError(
                    format!("Invalid AC value in voltage source '{}': must be a number", name)
                ))?;
            Some(ac_val)
        } else {
            None
        };

        // Create branch variable
        let branch_var = get_variable(&format!("{name}#branch"), Unit::Ampere, variables, var_map)
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Failed to create branch variable for voltage source: {}", name)
            ))?;

        let src = VSourceBundle::new(
            Arc::from(name),
            branch_var,
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            value,
            ac_value,
        );

        elements.push(Element::VSource(src));
        Ok(())
    }
}
