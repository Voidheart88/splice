/// The Resistor Module. As every module this module encapsulates exerything regarding a resistor bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub(crate) mod serde;
pub(crate) mod spice;

use std::sync::Arc;

use num::traits::FloatConst;
use num::{Complex, One, Zero};

use super::*;
use crate::spot::*;

/// A structure representing a bundle of inductors.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InductorBundle {
    pub name: Arc<str>,
    pub node0: Option<Variable>,
    pub node1: Option<Variable>,
    pub value: Numeric,
    /// Previous current through the inductor for transient simulation
    /// This stores the current from the last time step for proper integration
    pub previous_current: Numeric,
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
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
    ) -> InductorBundle {
        InductorBundle {
            name,
            node0,
            node1,
            value,
            previous_current: Numeric::zero(), // Initialize to 0A
        }
    }

    /// Returns the name of the inductor bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Updates the previous current for transient simulation
    /// This should be called after each time step with the current current
    pub fn update_previous_current(&mut self, current: Numeric) {
        self.previous_current = current;
    }

    /// Returns the previous current through the inductor
    pub fn previous_current(&self) -> Numeric {
        self.previous_current
    }

    /// Get node indices as Option<usize> for both nodes
    fn get_node_indices(&self) -> (Option<usize>, Option<usize>) {
        (
            self.node0.as_ref().map(|node| node.idx()),
            self.node1.as_ref().map(|node| node.idx()),
        )
    }

    /// Create AC triple for single-node connection (grounded inductor)
    fn create_single_node_ac_triple(&self, node_idx: usize, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        let impedance = Complex {
            re: Numeric::zero(),
            im: Numeric::one() / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
        };
        Triples::new(&[(node_idx, node_idx, impedance)])
    }

    /// Create full AC triples for two-node connection
    fn create_ac_triples(&self, node0_idx: usize, node1_idx: usize, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        let impedance = Complex {
            re: Numeric::zero(),
            im: Numeric::one() / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
        };
        Triples::new(&[
            (node0_idx, node0_idx, impedance),
            (node1_idx, node1_idx, impedance),
            (node0_idx, node1_idx, -impedance),
            (node1_idx, node0_idx, -impedance),
        ])
    }

    /// Returns the triples representing the inductor's contribution to matrix A.
    /// If `delta_t` is provided, the transient resistance is calculated using Euler integration.
    ///
    /// # Arguments
    ///
    /// * `delta_t` - Optional time step for transient simulation.
    pub fn triples(&self, delta_t: Option<&Numeric>) -> Triples<Numeric, 4> {
        // Äquivalenter Leitwert für die Induktivität
        let equivalent_conductance = match delta_t {
            Some(dt) => dt / self.value, // L / delta_t → Leitwert = delta_t / L
            None => DEFAULT_CONDUCTANCE, // Standardwert für DC/AC-Analyse
        };

        let node0_idx = if let Some(node) = &self.node0 {
            node.idx()
        } else {
            // If node0 doesn't exist, inductor is connected to ground through node1
            let node1_idx = self
                .node1
                .as_ref()
                .expect("Inductor must have at least one node connected")
                .idx();
            return Triples::new(&[(node1_idx, node1_idx, equivalent_conductance)]);
        };

        let node1_idx = if let Some(node) = &self.node1 {
            node.idx()
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

    /// Returns the index of node0 if it exists.
    pub fn node0_idx(&self) -> Option<usize> {
        self.node0.as_ref().map(|v| v.idx())
    }

    /// Returns the index of node1 if it exists.
    pub fn node1_idx(&self) -> Option<usize> {
        self.node1.as_ref().map(|v| v.idx())
    }

    /// Returns the pairs representing the right-hand side (RHS) for transient simulation
    /// This implements the backward Euler integration: v = L * (i_current - i_prev) / Δt
    /// Which rearranges to: (L/Δt)*i_current - (L/Δt)*i_prev = v
    /// In MNA, this becomes part of the RHS vector as: b = (L/Δt) * i_prev
    pub fn pairs(&self, delta_t: &Numeric) -> Pairs<Numeric, 2> {
        let r = delta_t / self.value; // Equivalent resistance = Δt/L
        let i_prev = self.previous_current;

        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            // If node0 doesn't exist, inductor is connected to ground through node1
            let node1_idx = self
                .node1_idx()
                .expect("Inductor must have at least one node connected");
            return Pairs::new(&[(node1_idx, r * i_prev)]);
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Pairs::new(&[(node0_idx, -r * i_prev)]);
        };

        // The RHS represents the voltage contribution from the previous time step
        // For backward Euler: v = L * (i_current - i_prev) / Δt
        // Rearranged: L*i_current/Δt - L*i_prev/Δt = v
        // In MNA, the RHS should be: (L/Δt) * i_prev (voltage drop across the inductor)
        Pairs::new(&[(node0_idx, r * i_prev), (node1_idx, -r * i_prev)])
    }

    /// Returns the pairs representing the right-hand side (RHS) for transient simulation using trapezoidal integration
    /// This implements the trapezoidal rule: v = L * (i_current - i_prev) / (Δt/2)
    /// Which rearranges to: (2L/Δt)*i_current - (2L/Δt)*i_prev = v
    /// In MNA, this becomes part of the RHS vector as: b = (2L/Δt) * i_prev
    pub fn pairs_trapezoidal(&self, delta_t: &Numeric) -> Pairs<Numeric, 2> {
        let r = (delta_t * 2.0) / self.value; // Equivalent resistance for trapezoidal rule = 2Δt/L
        let i_prev = self.previous_current;
        
        // Get node indices with early returns for special cases
        let (node0_idx, node1_idx) = self.get_node_indices();
        
        // Handle special cases where only one node is connected
        if node0_idx.is_none() {
            let node1_idx = node1_idx.expect("Inductor must have at least one node connected");
            return Pairs::new(&[(node1_idx, r * i_prev)]);
        }
        
        if node1_idx.is_none() {
            let node0_idx = node0_idx.expect("Inductor must have at least one node connected");
            return Pairs::new(&[(node0_idx, -r * i_prev)]);
        }

        // The RHS represents the voltage contribution from the previous time step
        // For trapezoidal rule: v = L * (i_current - i_prev) / (Δt/2)
        // Rearranged: (2L/Δt)*i_current - (2L/Δt)*i_prev = v
        // In MNA, the RHS should be: (2L/Δt) * i_prev (voltage drop across the inductor)
        Pairs::new(&[(node0_idx.unwrap(), r * i_prev), (node1_idx.unwrap(), -r * i_prev)])
    }

        // Get node indices with early returns for special cases
        let (node0_idx, node1_idx) = self.get_node_indices();
        
        // Handle special cases where only one node is connected
        if node0_idx.is_none() {
            let node1_idx = node1_idx.expect("Inductor must have at least one node connected");
            return self.create_single_node_ac_triple(node1_idx, freq);
        }
        
        if node1_idx.is_none() {
            let node0_idx = node0_idx.expect("Inductor must have at least one node connected");
            return self.create_single_node_ac_triple(node0_idx, freq);
        }
        
        // Create full AC triples for both nodes connected
        self.create_ac_triples(node0_idx.unwrap(), node1_idx.unwrap(), freq)
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
    }
=======
    /// Returns the triples representing the inductor's contribution to matrix A.
    pub fn ac_triples(&self, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        // Get node indices with early returns for special cases
        let (node0_idx, node1_idx) = self.get_node_indices();
        
        // Handle special cases where only one node is connected
        if node0_idx.is_none() {
            let node1_idx = node1_idx.expect("Inductor must have at least one node connected");
            return self.create_single_node_ac_triple(node1_idx, freq);
        }
        
        if node1_idx.is_none() {
            let node0_idx = node0_idx.expect("Inductor must have at least one node connected");
            return self.create_single_node_ac_triple(node0_idx, freq);
        }
        
        // Create full AC triples for both nodes connected
        self.create_ac_triples(node0_idx.unwrap(), node1_idx.unwrap(), freq)
    }
=======
        // Get node indices with early returns for special cases
        let (node0_idx, node1_idx) = self.get_node_indices();
        
        // Handle special cases where only one node is connected
        if node0_idx.is_none() {
            let node1_idx = node1_idx.expect("Inductor must have at least one node connected");
            return self.create_single_node_ac_triple(node1_idx, freq);
        }
        
        if node1_idx.is_none() {
            let node0_idx = node0_idx.expect("Inductor must have at least one node connected");
            return self.create_single_node_ac_triple(node0_idx, freq);
        }
        
        // Create full AC triples for both nodes connected
        self.create_ac_triples(node0_idx.unwrap(), node1_idx.unwrap(), freq)
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
    }
}

#[cfg(test)]
mod tests;
