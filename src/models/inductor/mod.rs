/// The Resistor Module. As every module this module encapsulates exerything regarding a resistor bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub(crate) mod spice;
pub(crate) mod yaml;

use std::sync::Arc;

use num::traits::FloatConst;
use num::{Complex, One, Zero};

use super::*;
use crate::spot::*;

/// A structure representing a bundle of inductors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InductorBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Numeric,
}

impl InductorBundle {
    /// Creates a new `InductorBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the inductor bundle.
    /// * `node0` - The first node of the inductor.
    /// * `node1` - The second node of the inductor.
    /// * `value` - The value of the inductor.
    ///
    /// # Returns
    ///
    /// A new `InductorBundle` object.
    pub fn new(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
    ) -> InductorBundle {
        InductorBundle {
            name,
            node0,
            node1,
            value: value,
        }
    }

    /// Returns the name of the inductor bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the triples representing the inductor's contribution to matrix A.
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let node0_idx = if let Some(node) = &self.node0 {
            node.idx()
        } else {
            return Triples::new(&[(
                self.node1.as_ref().unwrap().idx(),
                self.node1.as_ref().unwrap().idx(),
                DEFAULT_CONDUCTANCE,
            )]);
        };

        let node1_idx = if let Some(node) = &self.node1 {
            node.idx()
        } else {
            return Triples::new(&[(node0_idx, node0_idx, DEFAULT_CONDUCTANCE)]);
        };

        Triples::new(&[
            (node0_idx, node0_idx, DEFAULT_CONDUCTANCE),
            (node1_idx, node1_idx, DEFAULT_CONDUCTANCE),
            (node0_idx, node1_idx, DEFAULT_CONDUCTANCE),
            (node1_idx, node0_idx, DEFAULT_CONDUCTANCE),
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

    /// Returns the triples representing the inductor's contribution to matrix A.
    pub fn ac_triples(&self, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        let node0_idx = if let Some(node) = &self.node0 {
            node.idx()
        } else {
            return Triples::new(&[(
                self.node1.as_ref().unwrap().idx(),
                self.node1.as_ref().unwrap().idx(),
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            )]);
        };

        let node1_idx = if let Some(node) = &self.node1 {
            node.idx()
        } else {
            return Triples::new(&[(
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            )]);
        };

        Triples::new(&[
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
            (
                node0_idx,
                node1_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
            (
                node1_idx,
                node0_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
        ])
    }
}

#[cfg(test)]
mod tests;
