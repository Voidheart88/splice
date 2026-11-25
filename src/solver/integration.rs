// Integration method traits and implementations
// This module provides a trait-based abstraction for different integration methods
// used in transient simulation (e.g., Backward Euler, Forward Euler, Trapezoidal)
//
// The integration methods are implemented as a separate trait system rather than being
// generic parameters to models. This design choice provides several advantages:
// 1. Clean separation of concerns - integration is orthogonal to circuit elements
// 2. Runtime flexibility - integration methods can be changed without recompilation
// 3. Simplified type system - models don't need to be generic over integration methods
// 4. Better for dynamic switching - integrators can be changed on-the-fly for non-convergence
//
// This approach is more flexible than making models generic like capacitor<BackwardEuler>
// because it allows the same circuit elements to work with different integration methods
// without requiring monomorphization for each combination.

use crate::spot::Numeric;

/// Trait for integration methods used in transient simulation
/// This trait abstracts the mathematical operations needed for different
/// numerical integration methods
pub trait IntegrationMethod {
    /// Returns the name of the integration method
    fn name(&self) -> &str;

    /// Returns the time step size
    fn delta_t(&self) -> Numeric;

    /// Calculates the equivalent conductance for a capacitor
    /// For a capacitor: I = C * dV/dt
    /// Different integration methods transform this into different equivalent conductances
    fn capacitor_equivalent_conductance(&self, capacitance: Numeric) -> Numeric;

    /// Calculates the equivalent conductance for an inductor
    /// For an inductor: V = L * dI/dt
    /// Different integration methods transform this into different equivalent conductances
    fn inductor_equivalent_conductance(&self, inductance: Numeric) -> Numeric;

    /// Calculates the history current source for a capacitor
    /// This represents the contribution from previous time steps
    fn capacitor_history_current(&self, capacitance: Numeric, previous_voltage: Numeric)
    -> Numeric;

    /// Calculates the history current source for an inductor
    /// This represents the contribution from previous time steps
    fn inductor_history_current(&self, inductance: Numeric, previous_current: Numeric) -> Numeric;

    /// Calculates the equivalent conductance adjustment for nonlinear elements
    /// This is a placeholder for future extensions (e.g., MOSFETs with charge conservation)
    fn nonlinear_conductance_adjustment(&self, base_conductance: Numeric) -> Numeric {
        // Default: no adjustment for linear elements
        base_conductance
    }

    /// Indicates whether this method is implicit (requires matrix solution)
    fn is_implicit(&self) -> bool;

    /// Returns the order of the integration method
    /// For BDF methods: BDF1=1, BDF2=2, etc.
    /// For single-step methods: returns 1
    fn order(&self) -> usize;

    /// Indicates whether this method is a multi-step method
    /// Multi-step methods require history from multiple previous time steps
    fn is_multistep(&self) -> bool;
}

/// Trait for single-step integration methods
/// Single-step methods only need the current state to compute the next state
pub trait SingleStepMethod: IntegrationMethod {
    /// Returns the category name
    fn category_name(&self) -> &'static str {
        "Single-Step"
    }
}

/// Trait for multi-step integration methods
/// Multi-step methods require history from multiple previous time steps
pub trait MultiStepMethod: IntegrationMethod {
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

/// Backward Euler integration method
/// This is the default method used in most circuit simulators
/// It's implicit and unconditionally stable
#[derive(Debug, Clone)]
pub struct BackwardEuler {
    delta_t: Numeric,
}

impl BackwardEuler {
    /// Creates a new BackwardEuler integrator with the given time step
    pub fn new(delta_t: Numeric) -> Self {
        Self { delta_t }
    }
}

impl IntegrationMethod for BackwardEuler {
    fn name(&self) -> &str {
        "Backward Euler"
    }

    fn delta_t(&self) -> Numeric {
        self.delta_t
    }

    fn capacitor_equivalent_conductance(&self, capacitance: Numeric) -> Numeric {
        // For Backward Euler: G_eq = C / delta_t
        capacitance / self.delta_t
    }

    fn inductor_equivalent_conductance(&self, inductance: Numeric) -> Numeric {
        // For Backward Euler: G_eq = delta_t / L
        self.delta_t / inductance
    }

    fn capacitor_history_current(
        &self,
        capacitance: Numeric,
        previous_voltage: Numeric,
    ) -> Numeric {
        // For Backward Euler: I_hist = (C / delta_t) * V_prev
        (capacitance / self.delta_t) * previous_voltage
    }

    fn inductor_history_current(&self, inductance: Numeric, previous_current: Numeric) -> Numeric {
        // For Backward Euler: I_hist = (delta_t / L) * I_prev
        (self.delta_t / inductance) * previous_current
    }

    fn is_implicit(&self) -> bool {
        true
    }

    fn order(&self) -> usize {
        1
    }

    fn is_multistep(&self) -> bool {
        false
    }
}

impl SingleStepMethod for BackwardEuler {}

/// Forward Euler integration method
/// This is an explicit method that's conditionally stable
/// It's simpler but requires smaller time steps
#[derive(Debug, Clone)]
pub struct ForwardEuler {
    delta_t: Numeric,
}

impl ForwardEuler {
    /// Creates a new ForwardEuler integrator with the given time step
    pub fn new(delta_t: Numeric) -> Self {
        Self { delta_t }
    }
}

impl IntegrationMethod for ForwardEuler {
    fn name(&self) -> &str {
        "Forward Euler"
    }

    fn delta_t(&self) -> Numeric {
        self.delta_t
    }

    fn capacitor_equivalent_conductance(&self, capacitance: Numeric) -> Numeric {
        // For Forward Euler: G_eq = C / delta_t (same as Backward Euler for capacitors)
        capacitance / self.delta_t
    }

    fn inductor_equivalent_conductance(&self, inductance: Numeric) -> Numeric {
        // For Forward Euler: G_eq = delta_t / L (same as Backward Euler for inductors)
        self.delta_t / inductance
    }

    fn capacitor_history_current(
        &self,
        capacitance: Numeric,
        previous_voltage: Numeric,
    ) -> Numeric {
        // For Forward Euler: I_hist = (C / delta_t) * V_prev (same as Backward Euler)
        (capacitance / self.delta_t) * previous_voltage
    }

    fn inductor_history_current(&self, inductance: Numeric, previous_current: Numeric) -> Numeric {
        // For Forward Euler: I_hist = (delta_t / L) * I_prev (same as Backward Euler)
        (self.delta_t / inductance) * previous_current
    }

    fn is_implicit(&self) -> bool {
        false
    }

    fn order(&self) -> usize {
        1
    }

    fn is_multistep(&self) -> bool {
        false
    }
}

impl SingleStepMethod for ForwardEuler {}

/// Trapezoidal integration method
/// This is an implicit method that's more accurate than Backward Euler
/// It's A-stable and has better accuracy properties
#[derive(Debug, Clone)]
pub struct Trapezoidal {
    delta_t: Numeric,
}

impl Trapezoidal {
    /// Creates a new Trapezoidal integrator with the given time step
    pub fn new(delta_t: Numeric) -> Self {
        Self { delta_t }
    }
}

impl IntegrationMethod for Trapezoidal {
    fn name(&self) -> &str {
        "Trapezoidal"
    }

    fn delta_t(&self) -> Numeric {
        self.delta_t
    }

    fn capacitor_equivalent_conductance(&self, capacitance: Numeric) -> Numeric {
        // For Trapezoidal: G_eq = (2C) / delta_t
        (2.0 * capacitance) / self.delta_t
    }

    fn inductor_equivalent_conductance(&self, inductance: Numeric) -> Numeric {
        // For Trapezoidal: G_eq = (2 * delta_t) / L
        (2.0 * self.delta_t) / inductance
    }

    fn capacitor_history_current(
        &self,
        capacitance: Numeric,
        previous_voltage: Numeric,
    ) -> Numeric {
        // For Trapezoidal: I_hist = (2C / delta_t) * V_prev
        (2.0 * capacitance / self.delta_t) * previous_voltage
    }

    fn inductor_history_current(&self, inductance: Numeric, previous_current: Numeric) -> Numeric {
        // For Trapezoidal: I_hist = (2 * delta_t / L) * I_prev
        (2.0 * self.delta_t / inductance) * previous_current
    }

    fn is_implicit(&self) -> bool {
        true
    }

    fn order(&self) -> usize {
        2
    }

    fn is_multistep(&self) -> bool {
        false
    }
}

impl SingleStepMethod for Trapezoidal {}

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

/// Default integration method used when none is specified
/// This is typically Backward Euler for stability
pub fn default_integration_method() -> impl IntegrationMethod {
    BackwardEuler::new(1e-6) // Default time step
}

//Opens some Methods for Testing reasons
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backward_euler() {
        let method = BackwardEuler::new(1e-6);
        assert_eq!(method.name(), "Backward Euler");
        assert_eq!(method.delta_t(), 1e-6);
        assert_eq!(method.order(), 1);
        assert!(!method.is_multistep());
    }

    #[test]
    fn test_forward_euler() {
        let method = ForwardEuler::new(1e-6);
        assert_eq!(method.name(), "Forward Euler");
        assert_eq!(method.delta_t(), 1e-6);
        assert_eq!(method.order(), 1);
        assert!(!method.is_multistep());
    }

    #[test]
    fn test_trapezoidal() {
        let method = Trapezoidal::new(1e-6);
        assert_eq!(method.name(), "Trapezoidal");
        assert_eq!(method.delta_t(), 1e-6);
        assert_eq!(method.order(), 2);
        assert!(!method.is_multistep());
    }

    #[test]
    fn test_bdf1() {
        let method = BDF::new(1e-6, 1);
        assert_eq!(method.name(), "BDF1");
        assert_eq!(method.delta_t(), 1e-6);
        assert_eq!(method.order(), 1);
        assert!(method.is_multistep());
    }

    #[test]
    fn test_bdf2() {
        let method = BDF::new(1e-6, 2);
        assert_eq!(method.name(), "BDF2");
        assert_eq!(method.order(), 2);
        assert!(method.is_multistep());
    }

    #[test]
    fn test_bdf3() {
        let method = BDF::new(1e-6, 3);
        assert_eq!(method.name(), "BDF3");
        assert_eq!(method.order(), 3);
    }

    #[test]
    #[should_panic(expected = "BDF order must be between 1 and 6")]
    fn test_bdf_invalid_order_zero() {
        BDF::new(1e-6, 0);
    }

    #[test]
    #[should_panic(expected = "BDF order must be between 1 and 6")]
    fn test_bdf_invalid_order_too_high() {
        BDF::new(1e-6, 7);
    }

    #[test]
    fn test_bdf_coefficients() {
        // BDF1: y(t+dt) - y(t) = dt*f
        let coeffs1 = BDF::bdf_coefficients(1);
        assert!((coeffs1[0] - 1.0).abs() < 1e-10);
        assert!((coeffs1[1] - (-1.0)).abs() < 1e-10);

        // BDF2: (4/3)*y(t+dt) - (1/3)*y(t) = (2/3)*dt*f
        let coeffs2 = BDF::bdf_coefficients(2);
        assert!((coeffs2[0] - 4.0 / 3.0).abs() < 1e-10);
        assert!((coeffs2[1] - (-1.0 / 3.0)).abs() < 1e-10);
        assert!((coeffs2[2] - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_capacitor_history() {
        let mut method = BDF::new(1e-6, 3);

        // Push history values
        // Note: push_capacitor_history inserts at position 0 (most recent first)
        // After truncation to 3 values, we get [most_recent, ..., oldest]
        method.push_capacitor_history(1.0);
        method.push_capacitor_history(0.9);
        method.push_capacitor_history(0.8);
        method.push_capacitor_history(0.7); // Should be truncated, keeping [0.7, 0.8, 0.9]

        let history = method.capacitor_history();
        assert_eq!(history.len(), 3); // Should only keep 3 values
        // history[0] = 0.7 (most recent), history[1] = 0.8, history[2] = 0.9 (oldest)
        assert!(
            (history[0] - 0.7).abs() < 1e-9,
            "history[0] = {}, expected 0.7",
            history[0]
        );
        assert!(
            (history[1] - 0.8).abs() < 1e-9,
            "history[1] = {}, expected 0.8",
            history[1]
        );
        assert!(
            (history[2] - 0.9).abs() < 1e-9,
            "history[2] = {}, expected 0.9",
            history[2]
        );
    }

    #[test]
    fn test_history_clearing() {
        let mut method = BDF::new(1e-6, 2);
        method.push_capacitor_history(1.0);
        method.push_capacitor_history(0.9);
        assert_eq!(method.capacitor_history().len(), 2);

        method.clear_history();
        assert!(method.capacitor_history().is_empty());
    }

    #[test]
    fn test_capacitor_history_contribution() {
        let method = BDF::new(1e-6, 2);
        let c = 1e-9; // 1 nF

        // With BDF2: (4/3)*V_current - (1/3)*V_prev = (2/3)*dt * I
        // History contribution should account for the -(-1/3)/(4/3) = 1/4 factor

        let contribution = method.capacitor_history_contribution(c);
        // Since history is empty, contribution should be 0
        assert!((contribution - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_integration_method_default() {
        let method = crate::solver::integration::default_integration_method();
        assert_eq!(method.name(), "Backward Euler");
    }
}
