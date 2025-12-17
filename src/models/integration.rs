// Integration method traits and implementations
// This module provides a trait-based abstraction for different integration methods
// used in transient simulation (e.g., Backward Euler, Forward Euler, Trapezoidal)

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
    fn capacitor_history_current(&self, capacitance: Numeric, previous_voltage: Numeric) -> Numeric;
    
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
    
    fn capacitor_history_current(&self, capacitance: Numeric, previous_voltage: Numeric) -> Numeric {
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
}

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
    
    fn capacitor_history_current(&self, capacitance: Numeric, previous_voltage: Numeric) -> Numeric {
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
}

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
    
    fn capacitor_history_current(&self, capacitance: Numeric, previous_voltage: Numeric) -> Numeric {
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
}

/// Default integration method used when none is specified
/// This is typically Backward Euler for stability
pub fn default_integration_method() -> impl IntegrationMethod {
    BackwardEuler::new(1e-6) // Default time step
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_backward_euler_capacitor() {
        let method = BackwardEuler::new(1e-6);
        let conductance = method.capacitor_equivalent_conductance(1e-6); // 1µF
        assert_eq!(conductance, 1.0); // C/delta_t = 1e-6/1e-6 = 1
    }
    
    #[test]
    fn test_backward_euler_inductor() {
        let method = BackwardEuler::new(1e-6);
        let conductance = method.inductor_equivalent_conductance(1e-3); // 1mH
        assert_eq!(conductance, 1e-3); // delta_t/L = 1e-6/1e-3 = 1e-3
    }
    
    #[test]
    fn test_trapezoidal_capacitor() {
        let method = Trapezoidal::new(1e-6);
        let conductance = method.capacitor_equivalent_conductance(1e-6); // 1µF
        assert_eq!(conductance, 2.0); // 2C/delta_t = 2*1e-6/1e-6 = 2
    }
    
    #[test]
    fn test_method_properties() {
        let backward_euler = BackwardEuler::new(1e-6);
        assert_eq!(backward_euler.name(), "Backward Euler");
        assert!(backward_euler.is_implicit());
        
        let forward_euler = ForwardEuler::new(1e-6);
        assert_eq!(forward_euler.name(), "Forward Euler");
        assert!(!forward_euler.is_implicit());
        
        let trapezoidal = Trapezoidal::new(1e-6);
        assert_eq!(trapezoidal.name(), "Trapezoidal");
        assert!(trapezoidal.is_implicit());
    }
}
