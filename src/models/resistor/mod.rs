/// The Resistor Module. As every module this module encapsulates exerything regarding a resistor bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub mod serde;
pub(crate) mod spice;

use std::sync::Arc;

use num::{Complex, One, Zero};

use super::*;

/// A structure representing a bundle of resistors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ResistorBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Numeric,
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
        value: Numeric,
    ) -> ResistorBundle {
        ResistorBundle {
            name,
            node0,
            node1,
            value,
        }
    }

    /// Returns the name of the resistor bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }
    /// Returns the index of node0 if it exists.
    pub fn node0_idx(&self) -> Option<usize> {
        self.node0.as_ref().map(|v| v.idx())
    }

    /// Returns the index of node1 if it exists.
    pub fn node1_idx(&self) -> Option<usize> {
        self.node1.as_ref().map(|v| v.idx())
    }

    /// Returns the value of the resistor.
    pub fn value(&self) -> Numeric {
        self.value
    }

    /// Returns triples representing this elements contribution to the a matrix
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let conductance = Numeric::one() / self.value;
        let node0_idx = self.node0_idx();
        let node1_idx = self.node1_idx();

        // Handle different connection cases using pattern matching
        match (node0_idx, node1_idx) {
            (None, Some(node1_idx)) => {
                // Resistor connected to ground through node1
                Triples::new(&[(node1_idx, node1_idx, conductance)])
            },
            (Some(node0_idx), None) => {
                // Resistor connected to ground through node0
                Triples::new(&[(node0_idx, node0_idx, conductance)])
            },
            (Some(node0_idx), Some(node1_idx)) => {
                // Resistor connected between two nodes
                Triples::new(&[
                    (node0_idx, node0_idx, conductance),
                    (node1_idx, node1_idx, conductance),
                    (node0_idx, node1_idx, -conductance),
                    (node1_idx, node0_idx, -conductance),
                ])
            },
            (None, None) => {
                // This should not happen as resistors must have at least one connection
                Triples::new(&[])
            }
        }
    }

    /// Returns the triples indices.
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        match (self.node0_idx(), self.node1_idx()) {
            (None, None) => None,
            (None, Some(idx_1)) => Some(TripleIdx::new(&[(idx_1, idx_1)])),
            (Some(idx_0), None) => Some(TripleIdx::new(&[(idx_0, idx_0)])),
            (Some(idx_0), Some(idx_1)) => Some(TripleIdx::new(&[
                (idx_0, idx_0),
                (idx_1, idx_1),
                (idx_0, idx_1),
                (idx_1, idx_0),
            ])),
        }
    }

    /// Returns triples representing this elements contribution to the a matrix
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        let conductance = Complex {
            re: Numeric::one() / self.value,
            im: Numeric::zero(),
        };

        let node0_idx = self.node0_idx();
        let node1_idx = self.node1_idx();

        // Handle different connection cases using pattern matching
        match (node0_idx, node1_idx) {
            (None, Some(node1_idx)) => {
                // Resistor connected to ground through node1
                Triples::new(&[(node1_idx, node1_idx, conductance)])
            },
            (Some(node0_idx), None) => {
                // Resistor connected to ground through node0
                Triples::new(&[(node0_idx, node0_idx, conductance)])
            },
            (Some(node0_idx), Some(node1_idx)) => {
                // Resistor connected between two nodes
                Triples::new(&[
                    (node0_idx, node0_idx, conductance),
                    (node1_idx, node1_idx, conductance),
                    (node0_idx, node1_idx, -conductance),
                    (node1_idx, node0_idx, -conductance),
                ])
            },
            (None, None) => {
                // This should not happen as resistors must have at least one connection
                Triples::new(&[])
            }
        }
    }
}

#[cfg(test)]
mod tests;
