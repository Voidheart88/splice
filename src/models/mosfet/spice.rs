use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, Mos0Bundle, Unit};

impl ProcessSpiceElement for Mos0Bundle {
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
                format!("Missing name in MOSFET: {}", ele)
            ))?
            .as_span().end() - offset;
        let name = &ele[0..name_end];

        //extract gate node
        let gate_node_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing gate node in MOSFET: {}", name)
            ))?
            .as_span();
        let gate_node = &ele[gate_node_span.start() - offset..gate_node_span.end() - offset];

        //extract drain node
        let drain_node_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing drain node in MOSFET: {}", name)
            ))?
            .as_span();
        let drain_node = &ele[drain_node_span.start() - offset..drain_node_span.end() - offset];

        //extract source node
        let source_node_span = inner.next()
            .ok_or_else(|| crate::frontends::FrontendError::ParseError(
                format!("Missing source node in MOSFET: {}", name)
            ))?
            .as_span();
        let source_node = &ele[source_node_span.start() - offset..source_node_span.end() - offset];

        let mosfet = Mos0Bundle::new(
            Arc::from(name),
            get_variable(gate_node, Unit::Volt, variables, var_map),
            get_variable(drain_node, Unit::Volt, variables, var_map),
            get_variable(source_node, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Mos0(mosfet));
        Ok(())
    }
}
