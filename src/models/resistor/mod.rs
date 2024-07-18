use std::sync::Arc;

use num::Complex;

use super::*;
/// A structure representing a bundle of resistors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct ResistorBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: f64,
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
        name: Arc<str>,
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
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
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
                self.node1_idx().unwrap(),
                self.node1_idx().unwrap(),
                1.0 / self.value,
            ));
        };
        let Some(node1_idx) = self.node1_idx() else {
            return Triples::Single((node0_idx, node0_idx, 1.0 / self.value));
        };

        Triples::Quad([
            (node0_idx, node0_idx, 1.0 / self.value),
            (node1_idx, node1_idx, 1.0 / self.value),
            (node0_idx, node1_idx, -1.0 / self.value),
            (node1_idx, node0_idx, -1.0 / self.value),
        ])
    }

    /// Returns triples representing this elements contribution to the a matrix
    pub fn ac_triples(&self) -> ComplexTriples {
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return ComplexTriples::Single((
                self.node1_idx().unwrap(),
                self.node1_idx().unwrap(),
                Complex {
                    re: 1.0 / self.value,
                    im: 0.0,
                },
            ));
        };
        let Some(node1_idx) = self.node1_idx() else {
            return ComplexTriples::Single((
                node0_idx,
                node0_idx,
                Complex {
                    re: 1.0 / self.value,
                    im: 0.0,
                },
            ));
        };

        ComplexTriples::Quad([
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: 1.0 / self.value,
                    im: 0.0,
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: 1.0 / self.value,
                    im: 0.0,
                },
            ),
            (
                node0_idx,
                node1_idx,
                Complex {
                    re: -1.0 / self.value,
                    im: 0.0,
                },
            ),
            (
                node1_idx,
                node0_idx,
                Complex {
                    re: -1.0 / self.value,
                    im: 0.0,
                },
            ),
        ])
    }
}

#[cfg(test)]
mod tests;
