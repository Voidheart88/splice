use std::{f64::consts::PI, sync::Arc};

use num::Complex;

use crate::consts::DEFAULT_CONDUCTANCE;

use super::*;

/// A structure representing a bundle of inductors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct InductorBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: f64,
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
        value: f64,
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
    pub fn triples(&self) -> Triples {
        let node0_idx = if let Some(node) = &self.node0 {
            node.idx()
        } else {
            return Triples::Single((
                self.node1.as_ref().unwrap().idx(),
                self.node1.as_ref().unwrap().idx(),
                DEFAULT_CONDUCTANCE,
            ));
        };

        let node1_idx = if let Some(node) = &self.node1 {
            node.idx()
        } else {
            return Triples::Single((node0_idx, node0_idx, DEFAULT_CONDUCTANCE));
        };

        Triples::Quad([
            (node0_idx, node0_idx, DEFAULT_CONDUCTANCE),
            (node1_idx, node1_idx, DEFAULT_CONDUCTANCE),
            (node0_idx, node1_idx, DEFAULT_CONDUCTANCE),
            (node1_idx, node0_idx, DEFAULT_CONDUCTANCE),
        ])
    }

    /// Returns the triples representing the inductor's contribution to matrix A.
    pub fn ac_triples(&self, freq: f64) -> ComplexTriples {
        let node0_idx = if let Some(node) = &self.node0 {
            node.idx()
        } else {
            return ComplexTriples::Single((
                self.node1.as_ref().unwrap().idx(),
                self.node1.as_ref().unwrap().idx(),
                Complex {
                    re: 0.0,
                    im: 1.0 / (2.0 * PI * freq * self.value),
                },
            ));
        };

        let node1_idx = if let Some(node) = &self.node1 {
            node.idx()
        } else {
            return ComplexTriples::Single((
                node0_idx,
                node0_idx,
                Complex {
                    re: 0.0,
                    im: 1.0 / (2.0 * PI * freq * self.value),
                },
            ));
        };

        ComplexTriples::Quad([
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: 0.0,
                    im: 1.0 / (2.0 * PI * freq * self.value),
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: 0.0,
                    im: 1.0 / (2.0 * PI * freq * self.value),
                },
            ),
            (
                node0_idx,
                node1_idx,
                -Complex {
                    re: 0.0,
                    im: 1.0 / (2.0 * PI * freq * self.value),
                },
            ),
            (
                node1_idx,
                node0_idx,
                -Complex {
                    re: 0.0,
                    im: 1.0 / (2.0 * PI * freq * self.value),
                },
            ),
        ])
    }
}

#[cfg(test)]
mod tests;
