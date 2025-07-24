use std::sync::Arc;

use num::{Complex, One, Zero};

use crate::models::triples::TripleIdx;

use super::*;

/// A structure representing a bundle of voltage sources.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct VSourceBundle {
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
    pub fn value(&self) -> Numeric {
        self.value
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let branch_idx = self.branch_idx();
        let node0_idx = match self.node0_idx() {
            Some(node0_idx) => node0_idx,
            None => {
                return Triples::new(&[
                    (self.branch_idx(), self.node1_idx().unwrap(), Numeric::one()),
                    (self.node1_idx().unwrap(), self.branch_idx(), Numeric::one()),
                ]);
            }
        };
        let node1_idx = match self.node1_idx() {
            Some(node1_idx) => node1_idx,
            None => {
                return Triples::new(&[
                    (
                        self.branch_idx(),
                        self.node0_idx().unwrap(),
                        -Numeric::one(),
                    ),
                    (
                        self.node0_idx().unwrap(),
                        self.branch_idx(),
                        -Numeric::one(),
                    ),
                ])
            }
        };

        Triples::new(&[
            (branch_idx, node0_idx, Numeric::one()),
            (node0_idx, branch_idx, Numeric::one()),
            (branch_idx, node1_idx, -Numeric::one()),
            (node1_idx, branch_idx, -Numeric::one()),
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

    /// Returns a reference to the triples representing matrix A.
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        let branch_idx = self.branch_idx();
        let node0_idx = match self.node0_idx() {
            Some(node0_idx) => node0_idx,
            None => {
                return Triples::new(&[
                    (
                        self.branch_idx(),
                        self.node1_idx().unwrap(),
                        Complex {
                            re: Numeric::one(),
                            im: Numeric::zero(),
                        },
                    ),
                    (
                        self.node1_idx().unwrap(),
                        self.branch_idx(),
                        Complex {
                            re: Numeric::one(),
                            im: Numeric::zero(),
                        },
                    ),
                ]);
            }
        };
        let node1_idx = match self.node1_idx() {
            Some(node1_idx) => node1_idx,
            None => {
                return Triples::new(&[
                    (
                        self.branch_idx(),
                        self.node0_idx().unwrap(),
                        Complex {
                            re: -Numeric::one(),
                            im: Numeric::zero(),
                        },
                    ),
                    (
                        self.node0_idx().unwrap(),
                        self.branch_idx(),
                        Complex {
                            re: -Numeric::one(),
                            im: Numeric::zero(),
                        },
                    ),
                ])
            }
        };

        Triples::new(&[
            (
                branch_idx,
                node0_idx,
                Complex {
                    re: Numeric::one(),
                    im: Numeric::zero(),
                },
            ),
            (
                node0_idx,
                branch_idx,
                Complex {
                    re: Numeric::one(),
                    im: Numeric::zero(),
                },
            ),
            (
                branch_idx,
                node1_idx,
                Complex {
                    re: -Numeric::one(),
                    im: Numeric::zero(),
                },
            ),
            (
                node1_idx,
                branch_idx,
                Complex {
                    re: -Numeric::one(),
                    im: Numeric::zero(),
                },
            ),
        ])
    }

    /// Returns a reference to the pair representing vector b.
    pub fn pairs(&self) -> Pairs<Numeric, 2> {
        Pairs::new(&[(self.branch_idx(), self.value)])
    }

    /// Returns the pair representing the current source contributions to the vector b.
    pub fn pair_idx(&self) -> Option<PairIdx<2>> {
        match (&self.node0, &self.node1) {
            (None, None) => None,
            (Some(node0), None) => Some(PairIdx::new(&[node0.idx()])),
            (None, Some(node1)) => Some(PairIdx::new(&[node1.idx()])),
            (Some(node0), Some(node1)) => Some(PairIdx::new(&[node0.idx(), node1.idx()])),
        }
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
