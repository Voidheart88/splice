use std::sync::Arc;

use crate::backends::{Col, Row};

use super::*;
/// A structure representing a bundle of resistors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct ResistorBundle {
    name: Arc<String>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Value,
}

impl ResistorBundle {
    /// Creates a new `ResistorBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the resistor bundle.
    /// * `node0` - The name of the first node.
    /// * `node1` - The name of the second node.
    /// * `value` - The value of the resistor.
    ///
    /// # Returns
    ///
    /// A new `ResistorBundle` object.
    pub fn new(
        name: Arc<String>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: f64,
    ) -> ResistorBundle {
        ResistorBundle {
            name,
            node0,
            node1,
            value: value.into(),
        }
    }

    /// Returns the name of the resistor bundle.
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    /// Returns the index of node0 if it exists.
    pub fn node0_idx(&self) -> Option<usize> {
        match &self.node0 {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }

    /// Returns the index of node1 if it exists.
    pub fn node1_idx(&self) -> Option<usize> {
        match &self.node1 {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }

    /// Returns triples representing this elements contribution to the a matrix
    pub fn triples(&self) -> Triples {
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return Triples::Single((
                Row(self.node1_idx().unwrap()),
                Col(self.node1_idx().unwrap()),
                1.0 / (*self.value),
            ));
        };
        let Some(node1_idx) = self.node1_idx() else {
            return Triples::Single((Row(node0_idx), Col(node0_idx), 1.0 / (*self.value)));
        };

        Triples::Quad([
            (Row(node0_idx), Col(node0_idx), 1.0 / (*self.value)),
            (Row(node1_idx), Col(node1_idx), 1.0 / (*self.value)),
            (Row(node0_idx), Col(node1_idx), -1.0 / (*self.value)),
            (Row(node1_idx), Col(node0_idx), -1.0 / (*self.value)),
        ])
    }
}

#[cfg(test)]
mod tests;
