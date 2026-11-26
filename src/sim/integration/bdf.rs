use super::{IntegrationMethod, MultiStepMethod};
use crate::spot::Numeric;

/// Backward Differentiation Formula (BDF) integration method
/// BDF is a multi-step method commonly used in circuit simulation
/// BDF1 is equivalent to Backward Euler, BDF2 is second-order, etc.
///
/// BDF formulas:
/// - BDF1 (k=1):  y(t+dt) = y(t) + dt * f(t+dt, y(t+dt))  (same as Backward Euler)
/// - BDF2 (k=2):  y(t+dt) = (4/3)*y(t) - (1/3)*y(t-dt) + (2/3)*dt * f(t+dt, y(t+dt))
/// - BDF3 (k=3):  y(t+dt) = (18/11)*y(t) - (9/11)*y(t-dt) + (2/11)*y(t-2*dt) + (6/11)*dt * f(t+dt, y(t+dt))
/// - etc.
///
/// For circuit simulation, we need to store history values for:
/// - Capacitor voltages: V_prev[0] = most recent, V_prev[1] = previous, etc.
/// - Inductor currents: I_prev[0] = most recent, I_prev[1] = previous, etc.
///
/// The history array stores values in reverse chronological order:
/// history[0] = value at t
/// history[1] = value at t-dt
/// history[2] = value at t-2*dt
/// etc.
#[derive(Debug, Clone)]
pub struct BDF {
    delta_t: Numeric,
    order: usize,
    capacitor_history: Vec<Numeric>,
    inductor_history: Vec<Numeric>,
}

impl BDF {
    /// Creates a new BDF integrator with the given time step and order
    ///
    /// # Arguments
    /// * `delta_t` - Time step size
    /// * `order` - Order of the BDF method (1 to 6)
    ///
    /// # Returns
    /// A new BDF integrator
    ///
    /// # Panics
    /// Panics if order is 0 or greater than 6 (BDF stability limit)
    pub fn new(delta_t: Numeric, order: usize) -> Self {
        // BDF methods are only stable for orders 1-6
        assert!(
            order >= 1 && order <= 6,
            "BDF order must be between 1 and 6, got {}",
            order
        );

        // For order k, we need k previous values (including current)
        // history[0] = current, history[1] = previous, ..., history[k-1] = k-1 steps back
        let capacity = order;

        BDF {
            delta_t,
            order,
            capacitor_history: Vec::with_capacity(capacity),
            inductor_history: Vec::with_capacity(capacity),
        }
    }

    /// Returns the order of this BDF method
    pub fn order(&self) -> usize {
        self.order
    }

    /// Get BDF coefficients for the given order
    /// These are the alpha coefficients for the BDF formula:
    /// sum(alpha_i * y(t-i*dt)) = beta * dt * f(t, y(t))
    /// where beta = 1 for all BDF methods
    pub fn bdf_coefficients(order: usize) -> Vec<Numeric> {
        match order {
            1 => vec![1.0, -1.0], // y(t+dt) - y(t) = dt*f
            2 => vec![4.0 / 3.0, -1.0 / 3.0, -1.0],
            3 => vec![18.0 / 11.0, -9.0 / 11.0, 2.0 / 11.0, -1.0],
            4 => vec![48.0 / 25.0, -36.0 / 25.0, 16.0 / 25.0, -3.0 / 25.0, -1.0],
            5 => vec![
                300.0 / 137.0,
                -300.0 / 137.0,
                200.0 / 137.0,
                -75.0 / 137.0,
                12.0 / 137.0,
                -1.0,
            ],
            6 => vec![
                360.0 / 147.0,
                -450.0 / 147.0,
                400.0 / 147.0,
                -225.0 / 147.0,
                72.0 / 147.0,
                -10.0 / 147.0,
                -1.0,
            ],
            _ => panic!("BDF order must be between 1 and 6"),
        }
    }

    /// Calculate the history contribution for BDF capacitors
    /// Uses all stored history values with appropriate coefficients
    pub fn capacitor_history_contribution(&self, capacitance: Numeric) -> Numeric {
        if self.capacitor_history.is_empty() {
            return 0.0;
        }

        let coeffs = Self::bdf_coefficients(self.order);
        let alpha0 = coeffs[0]; // Coefficient for current value

        // Sum of weighted history values: sum(-alpha[i]/alpha[0] * V[i])
        let mut contribution = 0.0;
        for (i, &v) in self.capacitor_history.iter().enumerate() {
            if i < coeffs.len() - 1 {
                let coeff = -coeffs[i + 1] / alpha0;
                contribution += coeff * v;
            }
        }

        // Multiply by G_eq = (alpha0 * C) / delta_t
        let g_eq = (alpha0 * capacitance) / self.delta_t;
        g_eq * contribution
    }

    /// Calculate the history contribution for BDF inductors
    pub fn inductor_history_contribution(&self, inductance: Numeric) -> Numeric {
        if self.inductor_history.is_empty() {
            return 0.0;
        }

        let coeffs = Self::bdf_coefficients(self.order);
        let alpha0 = coeffs[0];

        let mut contribution = 0.0;
        for (i, &i_val) in self.inductor_history.iter().enumerate() {
            if i < coeffs.len() - 1 {
                let coeff = -coeffs[i + 1] / alpha0;
                contribution += coeff * i_val;
            }
        }

        // For inductor: V = L * dI/dt
        // G_eq = (alpha0 * delta_t) / L
        let g_eq = (alpha0 * self.delta_t) / inductance;
        g_eq * contribution
    }
}

impl IntegrationMethod for BDF {
    fn name(&self) -> &str {
        match self.order {
            1 => "BDF1",
            2 => "BDF2",
            3 => "BDF3",
            4 => "BDF4",
            5 => "BDF5",
            6 => "BDF6",
            _ => "BDF",
        }
    }

    fn delta_t(&self) -> Numeric {
        self.delta_t
    }

    fn capacitor_equivalent_conductance(&self, capacitance: Numeric) -> Numeric {
        // For BDF: G_eq = (alpha[0] * C) / delta_t
        // where alpha[0] is the first BDF coefficient
        let coeffs = Self::bdf_coefficients(self.order);
        (coeffs[0] * capacitance) / self.delta_t
    }

    fn inductor_equivalent_conductance(&self, inductance: Numeric) -> Numeric {
        // For BDF: G_eq = (delta_t * alpha[0]) / L
        (self.delta_t * Self::bdf_coefficients(self.order)[0]) / inductance
    }

    fn capacitor_history_current(
        &self,
        _capacitance: Numeric,
        _previous_voltage: Numeric,
    ) -> Numeric {
        // For BDF, the history contribution is calculated using capacitor_history_contribution
        // This is a simplified version that returns 0 - the actual history contribution
        // is handled by the BDF-specific method
        0.0
    }

    fn inductor_history_current(
        &self,
        _inductance: Numeric,
        _previous_current: Numeric,
    ) -> Numeric {
        0.0
    }

    fn is_implicit(&self) -> bool {
        true
    }

    fn order(&self) -> usize {
        self.order
    }

    fn is_multistep(&self) -> bool {
        true
    }
}

impl MultiStepMethod for BDF {
    fn history_size(&self) -> usize {
        self.order
    }

    fn capacitor_history_contribution(&self, capacitance: Numeric, history: &[Numeric]) -> Numeric {
        // history[0] is the most recent value
        // BDF formula: sum(alpha_i * V_i) = alpha0 * C/dt * V_current
        // History contribution: sum(-alpha[i]/alpha0 * V[i]) * (alpha0 * C/dt)

        if history.is_empty() {
            return 0.0;
        }

        let coeffs = Self::bdf_coefficients(self.order);
        let alpha0 = coeffs[0];
        let g_eq = (alpha0 * capacitance) / self.delta_t;

        let mut contribution = 0.0;
        for (i, &v) in history.iter().enumerate() {
            if i < coeffs.len() - 1 {
                let coeff = -coeffs[i + 1] / alpha0;
                contribution += coeff * v;
            }
        }

        g_eq * contribution
    }

    fn inductor_history_contribution(&self, inductance: Numeric, history: &[Numeric]) -> Numeric {
        if history.is_empty() {
            return 0.0;
        }

        let coeffs = Self::bdf_coefficients(self.order);
        let alpha0 = coeffs[0];
        let g_eq = (alpha0 * self.delta_t) / inductance;

        let mut contribution = 0.0;
        for (i, &i_val) in history.iter().enumerate() {
            if i < coeffs.len() - 1 {
                let coeff = -coeffs[i + 1] / alpha0;
                contribution += coeff * i_val;
            }
        }

        g_eq * contribution
    }

    fn push_capacitor_history(&mut self, voltage: Numeric) {
        self.capacitor_history.insert(0, voltage);
        // Keep only the required number of history values
        if self.capacitor_history.len() > self.order {
            self.capacitor_history.truncate(self.order);
        }
    }

    fn push_inductor_history(&mut self, current: Numeric) {
        self.inductor_history.insert(0, current);
        // Keep only the required number of history values
        if self.inductor_history.len() > self.order {
            self.inductor_history.truncate(self.order);
        }
    }

    fn capacitor_history(&self) -> &[Numeric] {
        &self.capacitor_history
    }

    fn inductor_history(&self) -> &[Numeric] {
        &self.inductor_history
    }

    fn clear_history(&mut self) {
        self.capacitor_history.clear();
        self.inductor_history.clear();
    }
}
