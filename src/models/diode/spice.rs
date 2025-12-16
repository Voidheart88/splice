use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{DiodeBundle, Element, Unit};

impl ProcessSpiceElement for DiodeBundle {
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
                format!("Missing name in diode: {}", ele)
            ))?
            .as_span().end() - offset;
        let name = &ele[0..name_end];

        //extract Node0
        let node0_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing node0 in diode: {}", name)
            ))?
            .as_span();
        let node0 = &ele[node0_span.start() - offset..node0_span.end() - offset];

        //extract Node1
        let node1_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing node1 in diode: {}", name)
            ))?
            .as_span();
        let node1 = &ele[node1_span.start() - offset..node1_span.end() - offset];

        let dio = DiodeBundle::new(
            Arc::from(name),
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Diode(dio));
        Ok(())
    }
}
