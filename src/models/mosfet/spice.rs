use std::sync::Arc;

use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, Mos0Bundle, Unit};
use crate::spot::*;

impl ProcessSpiceElement for Mos0Bundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract gate node
        let gate_node = inner.next().unwrap().as_span();
        let gate_node = &ele[gate_node.start() - offset..gate_node.end() - offset];

        //extract drain node
        let drain_node = inner.next().unwrap().as_span();
        let drain_node = &ele[drain_node.start() - offset..drain_node.end() - offset];

        //extract drain node
        let source_node = inner.next().unwrap().as_span();
        let source_node = &ele[source_node.start() - offset..source_node.end() - offset];

        let mosfet = Mos0Bundle::new(
            Arc::from(name),
            get_variable(gate_node, Unit::Volt, variables, var_map),
            get_variable(drain_node, Unit::Volt, variables, var_map),
            get_variable(source_node, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Mos0(mosfet));
    }
}
