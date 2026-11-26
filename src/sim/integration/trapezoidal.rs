use super::{IntegrationMethod, SingleStepMethod};
use crate::spot::Numeric;

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
