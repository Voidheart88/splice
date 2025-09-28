use std::sync::Arc;

use crate::frontends::spice::ProcessSpiceElement;
use crate::models::{Element, GainBundle, Unit};
use crate::frontends::get_variable;

impl ProcessSpiceElement for GainBundle {
    fn process(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();

        // Extrahiere den Namen des Gain-Blocks (ohne das führende 'A')
        let name_rule = inner.next().unwrap();
        let name = &ele[name_rule.as_span().start() - offset + 1..name_rule.as_span().end() - offset];

        // Extrahiere den Input-Knoten
        let input_node = inner.next().unwrap().as_span();
        let input_node_str = &ele[input_node.start() - offset..input_node.end() - offset];

        // Extrahiere den Output-Knoten
        let output_node = inner.next().unwrap().as_span();
        let output_node_str = &ele[output_node.start() - offset..output_node.end() - offset];

        // Extrahiere den Verstärkungsfaktor
        let value_str = inner.next().unwrap().as_span();
        let value_str = &ele[value_str.start() - offset..value_str.end() - offset];
        let value: f64 = value_str.parse().expect("Gain value must be a valid number");

        // Erstelle die Input- und Output-Variablen
        let input_var = get_variable(input_node_str, Unit::Volt, variables, var_map);
        let output_var = get_variable(output_node_str, Unit::Volt, variables, var_map);

        // Erstelle das GainBundle
        let gain = GainBundle::new(
            Arc::from(name),
            input_var,
            output_var,
            value,
        );

        // Füge das GainBundle als Element hinzu
        elements.push(Element::Gain(gain));
    }
}
