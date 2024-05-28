use std::sync::Arc;

use super::*;

/// A structure representing a bundle of current sources.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct ISourceBundle {
    name: Arc<String>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Value,
}

impl ISourceBundle {
    /// Creates a new `CurrentSourceBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the current source bundle.
    /// * `node0` - The first node of the current source.
    /// * `node1` - The second node of the current source.
    /// * `value` - The value of the current source.
    ///
    /// # Returns
    ///
    /// A new `CurrentSourceBundle` object.
    pub fn new(
        name: Arc<String>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: f64,
    ) -> Self {
        ISourceBundle {
            name,
            node0,
            node1,
            value: Value(value),
        }
    }

    /// Returns the name of the current source bundle.
    pub fn name(&self) -> Arc<String> {
        self.name.clone()
    }

    /// Returns the doubles representing the current source contributions to the vector b.
    pub fn doubles(&self) -> Doubles {
        match (&self.node0, &self.node1) {
            (None, None) => Doubles::Empty,
            (Some(node0), None) => Doubles::Single((Row(node0.idx()), -*self.value)),
            (None, Some(node1)) => Doubles::Single((Row(node1.idx()), *self.value)),
            (Some(node0), Some(node1)) => Doubles::Double([
                (Row(node0.idx()), -*self.value),
                (Row(node1.idx()), *self.value),
            ]),
        }
    }
}
