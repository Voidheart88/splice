use std::sync::Arc;

use num::Complex;
use std::f64::consts::PI;

use super::*;

/// A structure representing a bundle of capacitors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct CapacitorBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: f64,
}

impl CapacitorBundle {
    /// Creates a new `CapacitorBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the capacitor bundle.
    /// * `node0` - The name of the first node.
    /// * `node1` - The name of the second node.
    /// * `node0_idx` - The index of the first node.
    /// * `node1_idx` - The index of the second node.
    /// * `value` - The value of the capacitor.
    ///
    /// # Returns
    ///
    /// A new `CapacitorBundle` object.
    pub fn new(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: f64,
    ) -> CapacitorBundle {
        CapacitorBundle {
            name,
            node0,
            node1,
            value: value.into(),
        }
    }

    /// Returns the name of the capacitor bundle.
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

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self) -> Triples {
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return Triples::Single(
                (self.node1_idx().unwrap(), self.node1_idx().unwrap(), 0.0).into(),
            );
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Triples::Single((node0_idx, node0_idx, 0.0));
        };

        Triples::Quad([
            (node0_idx, node0_idx, 0.0),
            (node1_idx, node1_idx, 0.0),
            (node0_idx, node1_idx, 0.0),
            (node1_idx, node0_idx, 0.0),
        ])
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn ac_triples(&self, freq: f64) -> ComplexTriples {
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return ComplexTriples::Single(
                (
                    self.node1_idx().unwrap(),
                    self.node1_idx().unwrap(),
                    Complex {
                        re: 0.0,
                        im: -2.0 * PI * freq * self.value,
                    },
                )
                    .into(),
            );
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return ComplexTriples::Single((
                node0_idx,
                node0_idx,
                Complex {
                    re: 0.0,
                    im: -2.0 * PI * freq * self.value,
                },
            ));
        };

        ComplexTriples::Quad([
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: 0.0,
                    im: -2.0 * PI * freq * self.value,
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: 0.0,
                    im: -2.0 * PI * freq * self.value,
                },
            ),
            (
                node0_idx,
                node1_idx,
                -Complex {
                    re: 0.0,
                    im: -2.0 * PI * freq * self.value,
                },
            ),
            (
                node1_idx,
                node0_idx,
                -Complex {
                    re: 0.0,
                    im: -2.0 * PI * freq * self.value,
                },
            ),
        ])
    }
}

#[cfg(test)]
mod tests;
