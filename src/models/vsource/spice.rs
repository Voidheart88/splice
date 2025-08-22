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

        let ac_value = if let Some(val) = inner.next() {
            let val = val.as_str().split(" ").nth(1).unwrap();
            let val = val.parse().unwrap();
            Some(val)
        } else {
            None
        };

        let src = VSourceBundle::new(
            Arc::from(name),
            get_variable(&format!("{name}#branch"), Unit::Ampere, variables, var_map).unwrap(),
            get_variable(node0, Unit::Volt, variables, var_map),
            get_variable(node1, Unit::Volt, variables, var_map),
            value,
            ac_value,
        );

        elements.push(Element::VSource(src));
    }
}
