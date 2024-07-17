use std::sync::Arc;

use super::*;

/// A structure representing a bundle of current sources.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct ISourceBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: f64,
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
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: f64,
    ) -> Self {
        ISourceBundle {
            name,
            node0,
            node1,
            value: value,
        }
    }

    /// Returns the name of the current source bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the pair representing the current source contributions to the vector b.
    pub fn pairs(&self) -> Pairs {
        match (&self.node0, &self.node1) {
            (None, None) => Pairs::Empty,
            (Some(node0), None) => Pairs::Single((node0.idx(), -self.value)),
            (None, Some(node1)) => Pairs::Single((node1.idx(), self.value)),
            (Some(node0), Some(node1)) => {
                Pairs::Double([(node0.idx(), -self.value), (node1.idx(), self.value)])
            }
        }
    }
}

#[cfg(test)]
mod tests;
