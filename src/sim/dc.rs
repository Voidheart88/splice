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
///
/// This function updates the voltage of a voltage source element.
/// Used in DC sweep analysis to vary the voltage source value.
fn set_vsource_voltage(element: &mut Element, voltage: Numeric) {
    if let Element::VSource(ref mut vs) = element {
        vs.set_voltage(voltage);
    }
}

/// Trait for DC sweep simulation.
///
/// DC analysis performs a sweep of a voltage source over a specified range
/// and collects the circuit response at each step. This is useful for
/// analyzing circuit behavior over different operating conditions.
pub(super) trait DcSimulation<SO: Solver> {
    /// Runs a DC sweep analysis.
    ///
    /// # Arguments
    ///
    /// * `srcnam` - Name of the voltage source to sweep
    /// * `vstart` - Starting voltage for the sweep
    /// * `vstop` - Ending voltage for the sweep
    /// * `vstep` - Voltage step size
    /// * `_optional` - Optional second source for nested sweeps (not yet implemented)
    ///
    /// # Returns
    ///
    /// * `Ok(Sim::Dc)` - DC simulation results containing voltage sweeps
    /// * `Err(SimulatorError)` - If the voltage source is not found or simulation fails
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
        // Find the voltage source to sweep
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

        // Save the original voltage to restore later
        let voltage_0 = self
            .elements
            .get_mut(vsource1_idx)
            .and_then(get_vsource_value)
            .expect("Element should be a VSource");

        let mut dc_results = Vec::new();
        let mut voltage = *vstart;

<<<<<<< HEAD
        // Perform DC sweep from vstart to vstop with step size vstep
        // At each voltage step, find the operating point and store results
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
        while voltage <= *vstop {
            set_vsource_voltage(&mut self.elements[vsource1_idx], voltage);
            dc_results.push(self.find_op()?);
            voltage += vstep;
        }

<<<<<<< HEAD
        // Reset source voltage after sweep
        // TODO: Consider refactoring to reduce nesting complexity
        {
            let source = match &mut self.elements[vsource1_idx] {
                Element::VSource(ref mut vs) => vs,
                _ => unreachable!(),
            };
            source.set_voltage(voltage_0);
        }
=======
        // Restore original voltage after sweep
        set_vsource_voltage(&mut self.elements[vsource1_idx], voltage_0);
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
=======
        // Perform DC sweep from vstart to vstop with step size vstep
        // At each voltage step, find the operating point and store results
        while voltage <= *vstop {
            set_vsource_voltage(&mut self.elements[vsource1_idx], voltage);
            dc_results.push(self.find_op()?);
            voltage += vstep;
        }

        // Restore original voltage after sweep
        set_vsource_voltage(&mut self.elements[vsource1_idx], voltage_0);
=======
        // Perform DC sweep from vstart to vstop with step size vstep
        // At each voltage step, find the operating point and store results
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
        while voltage <= *vstop {
            set_vsource_voltage(&mut self.elements[vsource1_idx], voltage);
            dc_results.push(self.find_op()?);
            voltage += vstep;
        }

<<<<<<< HEAD
        // Reset source voltage after sweep
        // TODO: Consider refactoring to reduce nesting complexity
        {
            let source = match &mut self.elements[vsource1_idx] {
                Element::VSource(ref mut vs) => vs,
                _ => unreachable!(),
            };
            source.set_voltage(voltage_0);
        }
=======
        // Restore original voltage after sweep
        set_vsource_voltage(&mut self.elements[vsource1_idx], voltage_0);
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e

        Ok(Sim::Dc(dc_results))
    }
}
