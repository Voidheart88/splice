pub mod serde;
/// The Capacitor Module. As every module this module encapsulates exerything regarding this bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub(crate) mod spice;

use std::sync::Arc;

use num::traits::FloatConst;
use num::{Complex, One, Zero};

use super::*;
use crate::spot::*;

/// A structure representing a bundle of capacitors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CapacitorBundle {
    pub name: Arc<str>,
    pub node0: Option<Variable>,
    pub node1: Option<Variable>,
    pub value: Numeric,
    /// Previous voltage across the capacitor for transient simulation
    /// This stores the voltage from the last time step for proper integration
    pub previous_voltage: Numeric,
}

impl CapacitorBundle {
    /// Creates a new `CapacitorBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the capacitor bundle.
    /// * `node0` - The name of the first node.
    /// * `node1` - The name of the second node.
    /// * `node0_idx` - The index of the first node.
    /// * `node1_idx` - The index of the second node.
    /// * `value` - The value of the capacitor.
    ///
    /// # Returns
    ///
    /// A new `CapacitorBundle` object.
    pub fn new(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
    ) -> CapacitorBundle {
        CapacitorBundle {
            name,
            node0,
            node1,
            value,
            previous_voltage: Numeric::zero(), // Initialize to 0V
        }
    }

    /// Returns the name of the capacitor bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the index of node0 if it exists.
    pub fn node0_idx(&self) -> Option<usize> {
        self.node0.as_ref().map(|v| v.idx())
    }

    /// Returns the index of node1 if it exists.
    pub fn node1_idx(&self) -> Option<usize> {
        self.node1.as_ref().map(|v| v.idx())
    }

    /// Updates the previous voltage for transient simulation
    /// This should be called after each time step with the current voltage
    pub fn update_previous_voltage(&mut self, voltage: Numeric) {
        self.previous_voltage = voltage;
    }

    /// Returns the previous voltage across the capacitor
    pub fn previous_voltage(&self) -> Numeric {
        self.previous_voltage
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self, delta_t: Option<&Numeric>) -> Triples<Numeric, 4> {
        let equivalent_conductance = match delta_t {
            Some(delta_t) => self.value / delta_t,
            None => Numeric::zero(),
        };

        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            // If node0 doesn't exist, capacitor is connected to ground through node1
            let node1_idx = self.node1_idx().expect("Capacitor must have at least one node connected");
            return Triples::new(&[(
                node1_idx,
                node1_idx,
                equivalent_conductance,
            )]);
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Triples::new(&[(node0_idx, node0_idx, equivalent_conductance)]);
        };

        Triples::new(&[
            (node0_idx, node0_idx, equivalent_conductance),
            (node1_idx, node1_idx, equivalent_conductance),
            (node0_idx, node1_idx, -equivalent_conductance),
            (node1_idx, node0_idx, -equivalent_conductance),
        ])
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn ac_triples(&self, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            // If node0 doesn't exist, capacitor is connected to ground through node1
            let node1_idx = self.node1_idx().expect("Capacitor must have at least one node connected");
            return Triples::new(&[(
                node1_idx,
                node1_idx,
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            )]);
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Triples::new(&[(
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            )]);
        };

        Triples::new(&[
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
            (
                node0_idx,
                node1_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
            (
                node1_idx,
                node0_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
                },
            ),
        ])
    }

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

    /// Returns the pairs representing the right-hand side (RHS) for transient simulation
    /// This implements the backward Euler integration: i = C * (v_current - v_prev) / Δt
    /// Which rearranges to: (C/Δt)*v_current - (C/Δt)*v_prev = i
    /// In MNA, this becomes part of the RHS vector as: b = (C/Δt) * v_prev
    pub fn pairs(&self, delta_t: &Numeric) -> Pairs<Numeric, 2> {
        let g = self.value / delta_t; // Equivalent conductance
        let v_prev = self.previous_voltage;
        
        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            // If node0 doesn't exist, capacitor is connected to ground through node1
            let node1_idx = self.node1_idx().expect("Capacitor must have at least one node connected");
            return Pairs::new(&[(node1_idx, -g * v_prev)]);
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Pairs::new(&[(node0_idx, g * v_prev)]);
        };

        // The RHS represents the current contribution from the previous time step
        // For backward Euler: i = C * (v_current - v_prev) / Δt
        // Rearranged: C*v_current/Δt - C*v_prev/Δt = i
        // In MNA, the RHS should be: -C*v_prev/Δt (current flowing INTO the node)
        Pairs::new(&[
            (node0_idx, -g * v_prev),
            (node1_idx, g * v_prev),
        ])
    }
}

#[cfg(test)]
mod tests;
