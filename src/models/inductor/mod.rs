use std::sync::Arc;

use crate::backends::{Col, Row};
use crate::consts::DEFAULT_CONDUCTANCE;

use super::*;

/// A structure representing a bundle of inductors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct InductorBundle {
    name: Arc<String>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Value,
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
        name: Arc<String>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: f64,
    ) -> InductorBundle {
        InductorBundle {
            name,
            node0,
            node1,
            value: value.into(),
        }
    }

    /// Returns the name of the inductor bundle.
    pub fn name(&self) -> Arc<String> {
        self.name.clone()
    }

    /// Returns the triples representing the inductor's contribution to matrix A.
    pub fn triples(&self) -> Triples {
        let node0_idx = if let Some(node) = &self.node0 {
            node.idx()
        } else {
            return Triples::Single((
                Row(self.node1.as_ref().unwrap().idx()),
                Col(self.node1.as_ref().unwrap().idx()),
                DEFAULT_CONDUCTANCE,
            ));
        };

        let node1_idx = if let Some(node) = &self.node1 {
            node.idx()
        } else {
            return Triples::Single((Row(node0_idx), Col(node0_idx), DEFAULT_CONDUCTANCE));
        };

        Triples::Quad([
            (Row(node0_idx), Col(node0_idx), DEFAULT_CONDUCTANCE),
            (Row(node1_idx), Col(node1_idx), DEFAULT_CONDUCTANCE),
            (Row(node0_idx), Col(node1_idx), DEFAULT_CONDUCTANCE),
            (Row(node1_idx), Col(node0_idx), DEFAULT_CONDUCTANCE),
        ])
    }
}

#[cfg(test)]
mod tests;
