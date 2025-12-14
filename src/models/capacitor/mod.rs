pub(crate) mod serde;
/// The Capacitor Module. As every module this module encapsulates exerything regarding this bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub(crate) mod spice;

use std::sync::Arc;

use num::traits::FloatConst;
use num::{Complex, One, Zero};

use super::*;
use crate::spot::*;

/// A structure representing a bundle of capacitors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CapacitorBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Numeric,
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
        value: Numeric,
    ) -> CapacitorBundle {
        CapacitorBundle {
            name,
            node0,
            node1,
            value,
        }
    }

    /// Returns the name of the capacitor bundle.
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

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self, delta_t: Option<&Numeric>) -> Triples<Numeric, 4> {
        let equivalent_conductance = match delta_t {
            Some(delta_t) => self.value / delta_t,
            None => Numeric::zero(),
        };

        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return Triples::new(&[(
                self.node1_idx().unwrap(),
                self.node1_idx().unwrap(),
                equivalent_conductance,
            )]);
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Triples::new(&[(node0_idx, node0_idx, equivalent_conductance)]);
        };

        Triples::new(&[
            (node0_idx, node0_idx, equivalent_conductance),
            (node1_idx, node1_idx, equivalent_conductance),
            (node0_idx, node1_idx, -equivalent_conductance),
            (node1_idx, node0_idx, -equivalent_conductance),
        ])
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn ac_triples(&self, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            return Triples::new(&[(
                self.node1_idx().unwrap(),
                self.node1_idx().unwrap(),
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            )]);
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Triples::new(&[(
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            )]);
        };

        Triples::new(&[
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
            (
                node0_idx,
                node1_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
            (
                node1_idx,
                node0_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
        ])
    }

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
}

#[cfg(test)]
mod tests;
