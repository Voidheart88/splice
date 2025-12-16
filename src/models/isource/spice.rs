use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, ISourceBundle, Unit};
use crate::spot::*;
use std::sync::Arc;

impl ProcessSpiceElement for ISourceBundle {
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
                format!("Missing name in current source: {}", ele)
            ))?
            .as_span().end() - offset;
        let name = &ele[0..name_end];

        //extract Node0
        let node0_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing node0 in current source: {}", name)
            ))?
            .as_span();
        let node0 = &ele[node0_span.start() - offset..node0_span.end() - offset];

        //extract Node1
        let node1_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing node1 in current source: {}", name)
            ))?
            .as_span();
        let node1 = &ele[node1_span.start() - offset..node1_span.end() - offset];

        //extract Value
        let value_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing value in current source: {}", name)
            ))?
            .as_span();
        let value = ele[value_span.start() - offset..value_span.end() - offset]
            .parse::<Numeric>()
            .map_err(|_| crate::frontends::FrontendError::ParseError(
                format!("Invalid value in current source '{}': must be a number", name)
            ))?;

        let src = ISourceBundle::new(
            Arc::from(name),
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );

        elements.push(Element::ISource(src));
        Ok(())
    }
}
