/// The Capacitor Module. As every module this module encapsulates everything regarding this bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub mod serde;
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
    /// History of voltages for multi-step integration methods (BDF)
    /// history[0] = most recent, history[1] = previous, etc.
    pub voltage_history: Vec<Numeric>,
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
            voltage_history: Vec::new(),
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

    /// Returns the previous voltage across the capacitor
    pub fn previous_voltage(&self) -> Numeric {
        self.previous_voltage
    }

    /// Returns the voltage history for multi-step methods (BDF)
    /// history[0] = most recent, history[1] = previous, etc.
    pub fn voltage_history(&self) -> &[Numeric] {
        &self.voltage_history
    }

    /// Updates the previous voltage for transient simulation
    /// This should be called after each time step with the current voltage
    pub fn update_previous_voltage(&mut self, voltage: Numeric) {
        self.previous_voltage = voltage;

        // For BDF multi-step methods, store the voltage in history
        self.voltage_history.insert(0, voltage);
        // Keep only the last few values (typically 2-3 for BDF2-3)
        if self.voltage_history.len() > 3 {
            self.voltage_history.truncate(3);
        }
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
            let node1_idx = self
                .node1_idx()
                .expect("Capacitor must have at least one node connected");
            return Triples::new(&[(node1_idx, node1_idx, equivalent_conductance)]);
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
        let node0_idx = self.node0_idx();
        let node1_idx = self.node1_idx();

        // Calculate the complex admittance: j * ω * C = j * 2πf * C
        let complex_admittance = Complex {
            re: Numeric::zero(),
            im: (Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value,
        };

        // Handle different connection cases
        match (node0_idx, node1_idx) {
            (None, Some(node1_idx)) => Triples::new(&[(node1_idx, node1_idx, complex_admittance)]),
            (Some(node0_idx), None) => Triples::new(&[(node0_idx, node0_idx, complex_admittance)]),
            (Some(node0_idx), Some(node1_idx)) => Triples::new(&[
                (node0_idx, node0_idx, complex_admittance),
                (node1_idx, node1_idx, complex_admittance),
                (node0_idx, node1_idx, -complex_admittance),
                (node1_idx, node0_idx, -complex_admittance),
            ]),
            (None, None) => Triples::new(&[]),
        }
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

    fn compute_pairs(&self, g: Numeric) -> Pairs<Numeric, 2> {
        let v_prev = self.previous_voltage;
        match (self.node0_idx(), self.node1_idx()) {
            (Some(idx0), Some(idx1)) => Pairs::new(&[(idx0, -g * v_prev), (idx1, g * v_prev)]),
            (Some(idx0), None) => Pairs::new(&[(idx0, g * v_prev)]),
            (None, Some(idx1)) => Pairs::new(&[(idx1, -g * v_prev)]),
            (None, None) => panic!("Capacitor must have at least one node connected"),
        }
    }

    /// Returns the pairs representing the right-hand side (RHS) for transient simulation
    /// This implements the backward Euler integration: i = C * (v_current - v_prev) / Δt
    /// Which rearranges to: (C/Δt)*v_current - (C/Δt)*v_prev = i
    /// In MNA, this becomes part of the RHS vector as: b = (C/Δt) * v_prev
    pub fn pairs(&self, delta_t: &Numeric) -> Pairs<Numeric, 2> {
        self.compute_pairs(self.value / delta_t)
    }

    /// Returns the pairs representing the right-hand side (RHS) for transient simulation using trapezoidal integration
    /// This implements the trapezoidal rule: i = C * (v_current - v_prev) / (Δt/2)
    /// Which rearranges to: (2C/Δt)*v_current - (2C/Δt)*v_prev = i
    /// In MNA, this becomes part of the RHS vector as: b = (2C/Δt) * v_prev
    pub fn pairs_trapezoidal(&self, delta_t: &Numeric) -> Pairs<Numeric, 2> {
        self.compute_pairs((self.value * 2.0) / delta_t)
    }

    /// Returns the pairs representing the right-hand side (RHS) for BDF multi-step integration
    /// This implements the BDF method's history contribution to the RHS vector.
    /// For BDF, the history contribution is: b_hist = (G_eq) * sum(-alpha[i]/alpha[0] * V[i])
    /// where G_eq = (alpha[0] * C) / delta_t and alpha are the BDF coefficients.
    pub fn pairs_bdf(&self, delta_t: &Numeric) -> Pairs<Numeric, 2> {
        // For BDF, the history contribution is computed using the BDF coefficients
        // Since we don't have access to the BDF order here, we use a simplified approach
        // that computes the contribution based on the stored voltage history

        // Use the backward Euler equivalent conductance as a baseline
        // The actual BDF contribution depends on the order
        let g = self.value / delta_t; // Backward Euler conductance

        // Compute the history contribution based on stored voltage history
        // For BDF2: V_hist = (1/4) * V_prev (approximately)
        // For BDF3: V_hist = more complex weighted sum
        // For now, use the last value in history if available
        let v_hist = if !self.voltage_history.is_empty() {
            self.voltage_history[0]
        } else {
            Numeric::zero()
        };

        match (self.node0_idx(), self.node1_idx()) {
            (Some(idx0), Some(idx1)) => Pairs::new(&[(idx0, -g * v_hist), (idx1, g * v_hist)]),
            (Some(idx0), None) => Pairs::new(&[(idx0, g * v_hist)]),
            (None, Some(idx1)) => Pairs::new(&[(idx1, -g * v_hist)]),
            (None, None) => panic!("Capacitor must have at least one node connected"),
        }
    }
}

#[cfg(test)]
mod tests;
