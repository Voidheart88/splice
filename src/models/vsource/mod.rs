/// The Vsource Module. As every module this module encapsulates exerything regarding a Vsource bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub mod serde;
pub(crate) mod spice;

use std::sync::Arc;

use num::{Complex, One, Zero};

use super::*;

/// A structure representing a bundle of voltage sources.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VSourceBundle {
    name: Arc<str>,
    branch: Variable,
    node0: Option<Variable>,
    node1: Option<Variable>,
    value: Numeric,
    ac_value: Option<Numeric>,
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
    /// * `b` - The pairs representing vector b.
    /// * `value` - The value of the voltage source.
    ///
    /// # Returns
    ///
    /// A new `VSourceBundle` object.
    pub fn new(
        name: Arc<str>,
        branch: Variable,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
        ac_value: Option<Numeric>,
    ) -> Self {
        VSourceBundle {
            name,
            branch,
            node0,
            node1,
            value,
            ac_value,
        }
    }

    /// Returns the name of the voltage source bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the index of node0 if it exists.
    pub fn branch_idx(&self) -> usize {
        self.branch.idx()
    }

    /// Returns the index of node0 if it exists.
    pub fn node0_idx(&self) -> Option<usize> {
        self.node0.as_ref().map(|v| v.idx())
    }

    /// Returns the index of node1 if it exists.
    pub fn node1_idx(&self) -> Option<usize> {
        self.node1.as_ref().map(|v| v.idx())
    }

    /// Returns the value of the voltage source.
    pub fn value(&self) -> Numeric {
        self.value
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let branch_idx = self.branch_idx();
        let node0_idx = self.node0_idx();
        let node1_idx = self.node1_idx();

        // Handle different connection cases using pattern matching
        match (node0_idx, node1_idx) {
            (None, Some(node1_idx)) => {
                // Voltage source connected to ground through node1
                Triples::new(&[
                    (branch_idx, node1_idx, Numeric::one()),
                    (node1_idx, branch_idx, Numeric::one()),
                ])
            },
            (Some(node0_idx), None) => {
                // Voltage source connected to ground through node0
                Triples::new(&[
                    (branch_idx, node0_idx, -Numeric::one()),
                    (node0_idx, branch_idx, -Numeric::one()),
                ])
            },
            (Some(node0_idx), Some(node1_idx)) => {
                // Voltage source connected between two nodes
                Triples::new(&[
                    (branch_idx, node0_idx, Numeric::one()),
                    (node0_idx, branch_idx, Numeric::one()),
                    (branch_idx, node1_idx, -Numeric::one()),
                    (node1_idx, branch_idx, -Numeric::one()),
                ])
            },
            (None, None) => {
                // This should not happen as voltage sources must have at least one connection
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

    /// Returns a reference to the triples representing matrix A.
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        let branch_idx = self.branch_idx();
        let node0_idx = self.node0_idx();
        let node1_idx = self.node1_idx();
        let one_complex = Complex {
            re: Numeric::one(),
            im: Numeric::zero(),
        };
        let minus_one_complex = Complex {
            re: -Numeric::one(),
            im: Numeric::zero(),
        };

        // Handle different connection cases using pattern matching
        match (node0_idx, node1_idx) {
            (None, Some(node1_idx)) => {
                // Voltage source connected to ground through node1
                Triples::new(&[
                    (branch_idx, node1_idx, one_complex),
                    (node1_idx, branch_idx, one_complex),
                ])
            },
            (Some(node0_idx), None) => {
                // Voltage source connected to ground through node0
                Triples::new(&[
                    (branch_idx, node0_idx, minus_one_complex),
                    (node0_idx, branch_idx, minus_one_complex),
                ])
            },
            (Some(node0_idx), Some(node1_idx)) => {
                // Voltage source connected between two nodes
                Triples::new(&[
                    (branch_idx, node0_idx, one_complex),
                    (node0_idx, branch_idx, one_complex),
                    (branch_idx, node1_idx, minus_one_complex),
                    (node1_idx, branch_idx, minus_one_complex),
                ])
            },
            (None, None) => {
                // This should not happen as voltage sources must have at least one connection
                Triples::new(&[])
            }
        }
    }

    /// Returns a reference to the pair representing vector b.
    pub fn pairs(&self) -> Pairs<Numeric, 2> {
        Pairs::new(&[(self.branch_idx(), self.value)])
    }

    /// Returns a reference to the pair representing vector b.
    pub fn ac_pairs(&self) -> Pairs<ComplexNumeric, 2> {
        match self.ac_value {
            Some(ac_val) => Pairs::new(&[(
                self.branch_idx(),
                Complex {
                    re: ac_val,
                    im: Numeric::zero(),
                },
            )]),
            None => Pairs::new(&[]),
        }
    }

    pub fn set_voltage(&mut self, voltage: Numeric) {
        self.value = voltage;
    }
}

#[cfg(test)]
mod tests;
