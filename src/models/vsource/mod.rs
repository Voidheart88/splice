use std::sync::Arc;

use super::*;

/// A structure representing a bundle of voltage sources.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct VSourceBundle {
    name: Arc<String>,
    branch: Variable,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Value,
}

impl VSourceBundle {
    /// Creates a new `VSourceBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the voltage source bundle.
    /// * `branch` - The name of the branch.
    /// * `node0` - The name of the first node.
    /// * `node1` - The name of the second node.
    /// * `a` - The triples representing matrix A.
    /// * `b` - The doubles representing vector b.
    /// * `value` - The value of the voltage source.
    ///
    /// # Returns
    ///
    /// A new `VSourceBundle` object.
    pub fn new(
        name: Arc<String>,
        branch: Variable,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: f64,
    ) -> Self {
        VSourceBundle {
            name,
            branch,
            node0,
            node1,
            value: Value(value),
        }
    }

    /// Returns the name of the voltage source bundle.
    pub fn name(&self) -> Arc<String> {
        self.name.clone()
    }

    /// Returns the index of node0 if it exists.
    pub fn branch_idx(&self) -> usize {
        self.branch.idx()
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

    /// Returns the value of the voltage source.
    pub fn value(&self) -> f64 {
        self.value.0
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self) -> Triples {
        let branch_idx = self.branch_idx();
        let node0_idx = match self.node0_idx() {
            Some(node0_idx) => node0_idx,
            None => {
                return Triples::Double([
                    (Row(self.branch_idx()), Col(self.node1_idx().unwrap()), 1.0),
                    (Row(self.node1_idx().unwrap()), Col(self.branch_idx()), 1.0),
                ]);
            }
        };
        let node1_idx = match self.node1_idx() {
            Some(node1_idx) => node1_idx,
            None => {
                return Triples::Double([
                    (Row(self.branch_idx()), Col(self.node0_idx().unwrap()), -1.0),
                    (Row(self.node0_idx().unwrap()), Col(self.branch_idx()), -1.0),
                ])
            }
        };

        Triples::Quad([
            (Row(branch_idx), Col(node0_idx), 1.0),
            (Row(node0_idx), Col(branch_idx), 1.0),
            (Row(branch_idx), Col(node1_idx), -1.0),
            (Row(node1_idx), Col(branch_idx), -1.0),
        ])
    }

    /// Returns a reference to the doubles representing vector b.
    pub fn doubles(&self) -> Doubles {
        Doubles::Single((Row(self.branch_idx()), *self.value))
    }

    pub fn set_voltage(&mut self, voltage: f64) {
        self.value = Value(voltage);
    }
}

#[cfg(test)]
mod tests;
