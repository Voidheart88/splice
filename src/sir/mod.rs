use crate::sim::integration::{IntegrationMethod, MultiStepMethod};
use crate::spot::Numeric;

/// Trait extension for multi-step integration methods
/// This trait adds history management capabilities to multi-step integration methods
pub trait MultiStepIntegration: IntegrationMethod {
    /// Returns the category name
    fn category_name(&self) -> &'static str {
        "Multi-Step"
    }

    /// Returns the number of history values needed
    /// For BDF-k, this is k (the order)
    fn history_size(&self) -> usize;

    /// Calculates the history contribution for capacitors
    /// Uses all stored history values with appropriate coefficients
    fn capacitor_history_contribution(&self, capacitance: Numeric, history: &[Numeric]) -> Numeric;

    /// Calculates the history contribution for inductors
    /// Uses all stored history values with appropriate coefficients
    fn inductor_history_contribution(&self, inductance: Numeric, history: &[Numeric]) -> Numeric;

    /// Push a new history value for a capacitor
    fn push_capacitor_history(&mut self, voltage: Numeric);

    /// Push a new history value for an inductor
    fn push_inductor_history(&mut self, current: Numeric);

    /// Get the capacitor history values
    fn capacitor_history(&self) -> &[Numeric];

    /// Get the inductor history values
    fn inductor_history(&self) -> &[Numeric];

    /// Clear all history values - call this at the start of a new simulation
    fn clear_history(&mut self);
}

/// BDF State for a single node
/// Stores history values for multi-step methods
#[derive(Debug, Clone)]
pub struct BDFState {
    /// Time steps (most recent first)
    pub history: Vec<Numeric>,
    /// Current time step value
    pub current: Numeric,
}

impl BDFState {
    /// Create a new BDF state with initial value
    pub fn new(initial: Numeric) -> Self {
        BDFState {
            history: Vec::new(),
            current: initial,
        }
    }

    /// Push a new value and update history
    pub fn push(&mut self, value: Numeric) {
        self.history.insert(0, self.current);
        self.current = value;
        // Keep only the last few values
        if self.history.len() > 10 {
            self.history.truncate(10);
        }
    }

    /// Get history values (most recent first)
    pub fn history(&self) -> &[Numeric] {
        &self.history
    }

    /// Get the history contribution for BDF
    /// Uses BDF coefficients to weight the history values
    pub fn history_contribution(&self, alpha0: Numeric, coeffs: &[Numeric]) -> Numeric {
        if self.history.is_empty() {
            return Numeric::zero();
        }

        // Sum of weighted history values: sum(-alpha[i]/alpha[0] * V[i])
        let mut contribution = Numeric::zero();
        for (i, &v) in self.history.iter().enumerate() {
            if i < coeffs.len() - 1 {
                let coeff = -coeffs[i + 1] / alpha0;
                contribution += coeff * v;
            }
        }

        contribution
    }
}

/// BDF Solver for multi-step integration
/// This provides the core BDF algorithm for circuit simulation
/// It manages history for multiple variables (nodes) in the circuit
pub struct BDFSolver {
    /// Time step
    pub delta_t: Numeric,
    /// Order of the BDF method (1-6)
    pub order: usize,
    /// States for each variable (node)
    pub states: Vec<BDFState>,
}

impl BDFSolver {
    /// Create a new BDF solver
    pub fn new(delta_t: Numeric, order: usize, num_variables: usize) -> Self {
        assert!(order >= 1 && order <= 6, "BDF order must be between 1 and 6");
        BDFSolver {
            delta_t,
            order,
            states: (0..num_variables).map(|_| BDFState::new(Numeric::zero())).collect(),
        }
    }

    /// Get BDF coefficients for the given order
    /// These are the alpha coefficients for the BDF formula:
    /// sum(alpha_i * y(t-i*dt)) = beta * dt * f(t, y(t))
    /// where beta = 1 for all BDF methods
    pub fn bdf_coefficients(&self) -> Vec<Numeric> {
        match self.order {
            1 => vec![1.0, -1.0],  // y(t+dt) - y(t) = dt*f
            2 => vec![4.0/3.0, -1.0/3.0, -1.0],
            3 => vec![18.0/11.0, -9.0/11.0, 2.0/11.0, -1.0],
            4 => vec![48.0/25.0, -36.0/25.0, 16.0/25.0, -3.0/25.0, -1.0],
            5 => vec![300.0/137.0, -300.0/137.0, 200.0/137.0, -75.0/137.0, 12.0/137.0, -1.0],
            6 => vec![360.0/147.0, -450.0/147.0, 400.0/147.0, -225.0/147.0, 72.0/147.0, -10.0/147.0, -1.0],
            _ => panic!("BDF order must be between 1 and 6"),
        }
    }

    /// Get the BDF coefficient for a specific step
    pub fn bdf_coefficient(&self, step: usize) -> Numeric {
        let coeffs = self.bdf_coefficients();
        if step < coeffs.len() {
            coeffs[step]
        } else {
            Numeric::zero()
        }
    }

    /// Update all states with new values
    pub fn update_states(&mut self, values: &[Numeric]) {
        for (i, &val) in values.iter().enumerate() {
            if i < self.states.len() {
                self.states[i].push(val);
            }
        }
    }

    /// Get the current state values
    pub fn current_values(&self) -> Vec<Numeric> {
        self.states.iter().map(|s| s.current).collect()
    }

    /// Get history values for a specific variable
    pub fn history_values(&self, var_idx: usize) -> &[Numeric] {
        if var_idx < self.states.len() {
            self.states[var_idx].history()
        } else {
            &[]
        }
    }

    /// Compute the history contribution for a capacitor at a given node
    pub fn capacitor_history_contribution(&self, var_idx: usize, capacitance: Numeric) -> Numeric {
        if var_idx >= self.states.len() {
            return Numeric::zero();
        }

        let state = &self.states[var_idx];
        let coeffs = self.bdf_coefficients();
        let alpha0 = coeffs[0];
        let g_eq = (alpha0 * capacitance) / self.delta_t;

        g_eq * state.history_contribution(alpha0, &coeffs)
    }

    /// Compute the history contribution for an inductor at a given node
    pub fn inductor_history_contribution(&self, var_idx: usize, inductance: Numeric) -> Numeric {
        if var_idx >= self.states.len() {
            return Numeric::zero();
        }

        let state = &self.states[var_idx];
        let coeffs = self.bdf_coefficients();
        let alpha0 = coeffs[0];
        let g_eq = (alpha0 * self.delta_t) / inductance;

        g_eq * state.history_contribution(alpha0, &coeffs)
    }
}
