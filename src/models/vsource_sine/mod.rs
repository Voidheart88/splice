/// The VSourceSin Module. This module encapsulates everything regarding a sinusoidal voltage source bundle.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use num::{Complex, One, Zero};
use std::sync::Arc;

/// A structure representing a sinusoidal voltage source.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VSourceSinBundle {
    name: Arc<str>,
    branch: Variable,
    node0: Option<Variable>,
    node1: Option<Variable>,
    dc_offset: Numeric,
    amplitude: Numeric,
    frequency: Numeric,
    phase: Numeric,
    ac_value: Option<Numeric>,
}

impl VSourceSinBundle {
    /// Creates a new `VSourceSinBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sinusoidal voltage source.
    /// * `branch` - The branch variable.
    /// * `node0` - The first node.
    /// * `node1` - The second node.
    /// * `dc_offset` - The DC offset of the sinusoidal voltage source.
    /// * `amplitude` - The amplitude of the sinusoidal voltage source.
    /// * `frequency` - The frequency of the sinusoidal voltage source.
    /// * `phase` - The phase of the sinusoidal voltage source.
    /// * `ac_value` - The AC value of the sinusoidal voltage source.
    ///
    /// # Returns
    ///
    /// A new `VSourceSinBundle` object.
    pub fn new(
        name: Arc<str>,
        branch: Variable,
        node0: Option<Variable>,
        node1: Option<Variable>,
        dc_offset: Numeric,
        amplitude: Numeric,
        frequency: Numeric,
        phase: Numeric,
        ac_value: Option<Numeric>,
    ) -> Self {
        VSourceSinBundle {
            name,
            branch,
            node0,
            node1,
            dc_offset,
            amplitude,
            frequency,
            phase,
            ac_value,
        }
    }

    /// Returns the name of the sinusoidal voltage source.
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

    /// Returns the AC triples representing matrix A.
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

    /// Returns the pairs representing vector b.
    pub fn pairs(&self, time: Option<&Numeric>) -> Pairs<Numeric, 2> {
        let value = self.dc_offset
            + self.amplitude
                * (2.0 * std::f64::consts::PI * self.frequency * time.unwrap() + self.phase).sin();
        Pairs::new(&[(self.branch_idx(), value)])
    }

    /// Returns the AC pairs representing vector b.
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

    /// Sets the voltage of the sinusoidal voltage source.
    pub fn set_voltage(&mut self, voltage: Numeric) {
        self.dc_offset = voltage;
    }
}

#[cfg(test)]
mod tests;
