use super::{IntegrationMethod, SingleStepMethod};
use crate::spot::Numeric;

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
