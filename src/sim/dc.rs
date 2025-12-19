use std::sync::Arc;

use crate::models::Element;
use crate::sim::get_vsource_value;
use crate::sim::is_vsource_with_name;
use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;

/// Helper function to set voltage source voltage
fn set_vsource_voltage(element: &mut Element, voltage: Numeric) {
    if let Element::VSource(ref mut vs) = element {
        vs.set_voltage(voltage);
    }
}

pub(super) trait DcSimulation<SO: Solver> {
    fn run_dc(
        &mut self,
        srcnam: &Arc<str>,
        vstart: &Numeric,
        vstop: &Numeric,
        vstep: &Numeric,
        _optional: &Option<(Arc<str>, Numeric, Numeric, Numeric)>,
    ) -> Result<Sim, SimulatorError>;
}

impl<SO: Solver> DcSimulation<SO> for Simulator<SO> {
    fn run_dc(
        &mut self,
        srcnam: &Arc<str>,
        vstart: &Numeric,
        vstop: &Numeric,
        vstep: &Numeric,
        _optional: &Option<(Arc<str>, Numeric, Numeric, Numeric)>,
    ) -> Result<Sim, SimulatorError> {
        let vsource1_idx = self
            .elements
            .iter()
            .enumerate()
            .find(|&(_, element)| is_vsource_with_name(element, srcnam))
            .map(|(index, _)| index);

        let vsource1_idx = match vsource1_idx {
            Some(index) => index,
            None => return Err(SimulatorError::VoltageSourceNotFound(srcnam.to_string())),
        };

        let voltage_0 = self
            .elements
            .get_mut(vsource1_idx)
            .and_then(get_vsource_value)
            .expect("Element should be a VSource");

        let mut dc_results = Vec::new();
        let mut voltage = *vstart;

        // Perform DC sweep
        while voltage <= *vstop {
            set_vsource_voltage(&mut self.elements[vsource1_idx], voltage);
            dc_results.push(self.find_op()?);
            voltage += vstep;
        }

        // Restore original voltage
        set_vsource_voltage(&mut self.elements[vsource1_idx], voltage_0);

        Ok(Sim::Dc(dc_results))
    }
}
