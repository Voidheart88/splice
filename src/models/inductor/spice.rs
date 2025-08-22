use crate::frontends::get_variable;
use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, InductorBundle, Unit};
use crate::spot::*;
use std::sync::Arc;

impl ProcessSpiceElement for InductorBundle {
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

        //extract Node0
        let node0 = inner.next().unwrap().as_span();
        let node0 = &ele[node0.start() - offset..node0.end() - offset];

        //extract Node1
        let node1 = inner.next().unwrap().as_span();
        let node1 = &ele[node1.start() - offset..node1.end() - offset];

        //extract Value
        let value = inner.next().unwrap().as_span();
        let value = ele[value.start() - offset..value.end() - offset]
            .parse::<Numeric>()
            .unwrap();

        let ind = InductorBundle::new(
            Arc::from(name),
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );
        elements.push(Element::Inductor(ind));
    }
}
