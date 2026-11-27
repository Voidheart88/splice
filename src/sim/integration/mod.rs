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

mod backward_euler;
mod bdf;
mod forward_euler;
mod trapezoidal;

pub use backward_euler::BackwardEuler;
pub use bdf::BDF;
pub use forward_euler::ForwardEuler;
pub use trapezoidal::Trapezoidal;

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

/// Default integration method used when none is specified
/// This is typically Backward Euler for stability
pub fn default_integration_method() -> impl IntegrationMethod {
    BackwardEuler::new(1e-6) // Default time step
}
