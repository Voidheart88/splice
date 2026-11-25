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
    /// History of currents for multi-step integration methods (BDF)
    /// history[0] = most recent, history[1] = previous, etc.
    pub current_history: Vec<Numeric>,
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
            current_history: Vec::new(),
        }
    }

    /// Returns the name of the inductor bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the previous current through the inductor
    pub fn previous_current(&self) -> Numeric {
        self.previous_current
    }

    /// Returns the current history for multi-step methods (BDF)
    /// history[0] = most recent, history[1] = previous, etc.
    pub fn current_history(&self) -> &[Numeric] {
        &self.current_history
    }

    /// Updates the previous current for transient simulation
    /// This should be called after each time step with the current current
    pub fn update_previous_current(&mut self, current: Numeric) {
        self.previous_current = current;
        
        // For BDF multi-step methods, store the current in history
        self.current_history.insert(0, current);
        // Keep only the last few values (typically 2-3 for BDF2-3)
        if self.current_history.len() > 3 {
            self.current_history.truncate(3);
        }
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
        // For trapezoidal rule: v = L * (i_current - i_prev) / (Δt/2)
        // Rearranged: (2L/Δt)*i_current - (2L/Δt)*i_prev = v
        // In MNA, the RHS should be: (2L/Δt) * i_prev (voltage drop across the inductor)
        Pairs::new(&[(node0_idx, r * i_prev), (node1_idx, -r * i_prev)])
    }

    /// Returns the pairs representing the right-hand side (RHS) for BDF multi-step integration
    /// This implements the BDF method's history contribution to the RHS vector.
    /// For BDF, the history contribution is: b_hist = (G_eq) * sum(-alpha[i]/alpha[0] * I[i])
    /// where G_eq = (alpha[0] * delta_t) / L and alpha are the BDF coefficients.
    pub fn pairs_bdf(&self, delta_t: &Numeric) -> Pairs<Numeric, 2> {
        // For BDF, the history contribution is computed using the BDF coefficients
        // Since we don't have access to the BDF order here, we use a simplified approach
        // that computes the contribution based on the stored current history
        
        // Use the backward Euler equivalent conductance as a baseline
        let r = delta_t / self.value; // Backward Euler conductance = delta_t / L
        
        // Compute the history contribution based on stored current history
        // For BDF2: I_hist = (1/4) * I_prev (approximately)
        // For BDF3: I_hist = more complex weighted sum
        // For now, use the last value in history if available
        let i_hist = if !self.current_history.is_empty() {
            self.current_history[0]
        } else {
            Numeric::zero()
        };

        let node0_idx = if let Some(idx) = self.node0_idx() {
            idx
        } else {
            // If node0 doesn't exist, inductor is connected to ground through node1
            let node1_idx = self
                .node1_idx()
                .expect("Inductor must have at least one node connected");
            return Pairs::new(&[(node1_idx, r * i_hist)]);
        };
        let node1_idx = if let Some(idx) = self.node1_idx() {
            idx
        } else {
            return Pairs::new(&[(node0_idx, -r * i_hist)]);
        };

        Pairs::new(&[(node0_idx, r * i_hist), (node1_idx, -r * i_hist)])
    }

    /// Returns the triples representing the inductor's contribution to matrix A.
    pub fn ac_triples(&self, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        // TODO: Consider refactoring to reduce nesting complexity
        let node0_idx = if let Some(node) = &self.node0 {
            node.idx()
        } else {
            // If node0 doesn't exist, inductor is connected to ground through node1
            let node1_idx = self
                .node1
                .as_ref()
                .expect("Inductor must have at least one node connected")
                .idx();
            return Triples::new(&[(
                node1_idx,
                node1_idx,
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            )]);
        };

        // TODO: Consider refactoring to reduce nesting complexity
        let node1_idx = if let Some(node) = &self.node1 {
            node.idx()
        } else {
            return Triples::new(&[(
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            )]);
        };

        // TODO: Consider refactoring to reduce nesting complexity
        Triples::new(&[
            (
                node0_idx,
                node0_idx,
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
            (
                node1_idx,
                node1_idx,
                Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
            (
                node0_idx,
                node1_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
            (
                node1_idx,
                node0_idx,
                -Complex {
                    re: Numeric::zero(),
                    im: Numeric::one()
                        / ((Numeric::one() + Numeric::one()) * Numeric::PI() * freq * self.value),
                },
            ),
        ])
    }
}

#[cfg(test)]
mod tests;
