pub(crate) mod serde;
/// The Resistor Module. As every module this module encapsulates exerything regarding a resistor bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
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

    /// Returns triples representing this elements contribution to the a matrix
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return Triples::new(&[(
                self.node1_idx().unwrap(),
                self.node1_idx().unwrap(),
                Numeric::one() / self.value,
            )]);
        };
        let Some(node1_idx) = self.node1_idx() else {
            return Triples::new(&[(node0_idx, node0_idx, Numeric::one() / self.value)]);
        };

        Triples::new(&[
            (node0_idx, node0_idx, Numeric::one() / self.value),
            (node1_idx, node1_idx, Numeric::one() / self.value),
            (node0_idx, node1_idx, -Numeric::one() / self.value),
            (node1_idx, node0_idx, -Numeric::one() / self.value),
        ])
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
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return Triples::new(&[(
                self.node1_idx().unwrap(),
                self.node1_idx().unwrap(),
                Complex {
                    re: Numeric::one() / self.value,
                    im: Numeric::zero(),
                },
            )]);
        };
        let Some(node1_idx) = self.node1_idx() else {
            return Triples::new(&[(
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::one() / self.value,
                    im: Numeric::zero(),
                },
            )]);
        };

        Triples::new(&[
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::one() / self.value,
                    im: Numeric::zero(),
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: Numeric::one() / self.value,
                    im: Numeric::zero(),
                },
            ),
            (
                node0_idx,
                node1_idx,
                Complex {
                    re: -Numeric::one() / self.value,
                    im: Numeric::zero(),
                },
            ),
            (
                node1_idx,
                node0_idx,
                Complex {
                    re: -Numeric::one() / self.value,
                    im: Numeric::zero(),
                },
            ),
        ])
    }
}

#[cfg(test)]
mod tests;
