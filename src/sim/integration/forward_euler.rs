use super::{IntegrationMethod, SingleStepMethod};
use crate::spot::Numeric;

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
