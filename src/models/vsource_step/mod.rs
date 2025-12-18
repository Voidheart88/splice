/// The VSourceStep Module. This module encapsulates everything regarding a step voltage source bundle.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use num::{Complex, One, Zero};
use std::sync::Arc;

/// A structure representing a step voltage source.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VSourceStepBundle {
    name: Arc<str>,
    branch: Variable,
    node0: Option<Variable>,
    node1: Option<Variable>,
    initial_value: Numeric,
    final_value: Numeric,
    step_time: Numeric,
    ac_value: Option<Numeric>,
}

impl VSourceStepBundle {
    /// Creates a new `VSourceStepBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the step voltage source.
    /// * `branch` - The branch variable.
    /// * `node0` - The first node.
    /// * `node1` - The second node.
    /// * `initial_value` - The initial voltage value before the step.
    /// * `final_value` - The final voltage value after the step.
    /// * `step_time` - The time at which the step occurs.
    /// * `ac_value` - The AC value of the step voltage source.
    ///
    /// # Returns
    ///
    /// A new `VSourceStepBundle` object.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Arc<str>,
        branch: Variable,
        node0: Option<Variable>,
        node1: Option<Variable>,
        initial_value: Numeric,
        final_value: Numeric,
        step_time: Numeric,
        ac_value: Option<Numeric>,
    ) -> Self {
        VSourceStepBundle {
            name,
            branch,
            node0,
            node1,
            initial_value,
            final_value,
            step_time,
            ac_value,
        }
    }

    /// Returns the name of the step voltage source.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the index of the branch.
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

    /// Returns the triples representing matrix A.
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let branch_idx = self.branch_idx();
        let node0_idx = match self.node0_idx() {
            Some(node0_idx) => node0_idx,
            None => {
                // If node0 doesn't exist, voltage source is connected to ground through node1
                let node1_idx = self
                    .node1_idx()
                    .expect("Voltage source must have at least one node connected");
                return Triples::new(&[
                    (self.branch_idx(), node1_idx, Numeric::one()),
                    (node1_idx, self.branch_idx(), Numeric::one()),
                ]);
            }
        };
        let node1_idx = match self.node1_idx() {
            Some(node1_idx) => node1_idx,
            None => {
                // If node1 doesn't exist, voltage source is connected to ground through node0
                let node0_idx = self
                    .node0_idx()
                    .expect("Voltage source must have at least one node connected");
                return Triples::new(&[
                    (self.branch_idx(), node0_idx, -Numeric::one()),
                    (node0_idx, self.branch_idx(), -Numeric::one()),
                ]);
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

    /// Returns the pairs representing vector b.
    pub fn pairs(&self, time: Option<&Numeric>) -> Pairs<Numeric, 2> {
        let value = match time {
            Some(t) => {
                // For transient analysis: step function
                if *t >= self.step_time {
                    self.final_value
                } else {
                    self.initial_value
                }
            }
            None => {
                // For OP analysis: use initial value
                self.initial_value
            }
        };
        Pairs::new(&[(self.branch_idx(), value)])
    }

    /// Returns the AC pairs representing vector b.
    pub fn ac_pairs(&self) -> Pairs<ComplexNumeric, 2> {
        // Fixme: this match nests too deep
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

    /// Sets the voltage of the step voltage source.
    pub fn set_voltage(&mut self, voltage: Numeric) {
        self.initial_value = voltage;
        self.final_value = voltage;
    }
}

#[cfg(test)]
mod tests;
