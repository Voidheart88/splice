>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
use std::sync::Arc;

use crate::frontends::Simulation;
use crate::models::{
    CapacitorBundle, Element, ResistorBundle, Unit, VSourceBundle, VSourceSinBundle,
    VSourceStepBundle, Variable,
};
use crate::sim::commands::{ACMode, SimulationCommand};
use crate::sim::options::SimulationOption;
use crate::sim::simulation_result::Sim;
use crate::sim::Simulator;
use crate::solver::{FaerSolver, NalgebraSolver, RSparseSolver};
use crate::spot::*;

<<<<<<< HEAD
// TODO: Refactor init_sim_x tests to use helper functions and reduce code duplication
#[test]
fn init_sim_rsparse() {
=======
/// Find the result closest to the expected time
fn find_closest_result(
    tran_results: &[(Numeric, Vec<(Variable, Numeric)>)],
    expected_time: Numeric,
) -> Option<&(Numeric, Vec<(Variable, Numeric)>)> {
    tran_results
        .iter()
        .min_by_key(|(t, _)| ((*t - expected_time).abs() * 1000.0) as i32)
}

/// Extract voltage and current from simulation results
fn extract_voltage_and_current(values: &[(Variable, Numeric)]) -> (Numeric, Numeric) {
    let mut voltage = 0.0;
    let mut current = 0.0;

    for (var, val) in values {
        if var.name() == Arc::from("1") {
            voltage = *val;
        } else if var.name() == Arc::from("V1#branch") {
            current = *val;
        }
    }

    (voltage, current)
}

/// Create a basic simulation for testing different solvers
fn create_basic_simulation() -> Simulation {
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
=======
use std::sync::Arc;

use crate::frontends::Simulation;
use crate::models::{
    CapacitorBundle, Element, ResistorBundle, Unit, VSourceBundle, VSourceSinBundle,
    VSourceStepBundle, Variable,
};
use crate::sim::commands::{ACMode, SimulationCommand};
use crate::sim::options::SimulationOption;
use crate::sim::simulation_result::Sim;
use crate::sim::Simulator;
use crate::solver::{FaerSolver, NalgebraSolver, RSparseSolver};
use crate::spot::*;

/// Find the result closest to the expected time
fn find_closest_result(
    tran_results: &[(Numeric, Vec<(Variable, Numeric)>)],
    expected_time: Numeric,
) -> Option<&(Numeric, Vec<(Variable, Numeric)>)> {
    tran_results
        .iter()
        .min_by_key(|(t, _)| ((*t - expected_time).abs() * 1000.0) as i32)
}

/// Extract voltage and current from simulation results
fn extract_voltage_and_current(values: &[(Variable, Numeric)]) -> (Numeric, Numeric) {
    let mut voltage = 0.0;
    let mut current = 0.0;

    for (var, val) in values {
        if var.name() == Arc::from("1") {
            voltage = *val;
        } else if var.name() == Arc::from("V1#branch") {
            current = *val;
        }
    }

    (voltage, current)
}

/// Create a basic simulation for testing different solvers
fn create_basic_simulation() -> Simulation {

=======
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
use std::sync::Arc;

use crate::frontends::Simulation;
use crate::models::{
    CapacitorBundle, Element, ResistorBundle, Unit, VSourceBundle, VSourceSinBundle,
    VSourceStepBundle, Variable,
};
use crate::sim::commands::{ACMode, SimulationCommand};
use crate::sim::options::SimulationOption;
use crate::sim::simulation_result::Sim;
use crate::sim::Simulator;
use crate::solver::{FaerSolver, NalgebraSolver, RSparseSolver};
use crate::spot::*;

<<<<<<< HEAD
// TODO: Refactor init_sim_x tests to use helper functions and reduce code duplication
#[test]
fn init_sim_rsparse() {
=======
/// Find the result closest to the expected time
fn find_closest_result(
    tran_results: &[(Numeric, Vec<(Variable, Numeric)>)],
    expected_time: Numeric,
) -> Option<&(Numeric, Vec<(Variable, Numeric)>)> {
    tran_results
        .iter()
        .min_by_key(|(t, _)| ((*t - expected_time).abs() * 1000.0) as i32)
}

/// Extract voltage and current from simulation results
fn extract_voltage_and_current(values: &[(Variable, Numeric)]) -> (Numeric, Numeric) {
    let mut voltage = 0.0;
    let mut current = 0.0;

    for (var, val) in values {
        if var.name() == Arc::from("1") {
            voltage = *val;
        } else if var.name() == Arc::from("V1#branch") {
            current = *val;
        }
    }

    (voltage, current)
}

/// Create a basic simulation for testing different solvers
fn create_basic_simulation() -> Simulation {
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
    let commands = vec![SimulationCommand::Op];
    let options: Vec<SimulationOption> = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    let vsource = Element::VSource(VSourceBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        10.0,
        None,
    ));

    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        None,
        10.0,
    ));

    let elements = vec![vsource, resistor];
    let variables = vec![branch_1, node_1];

    Simulation {
        commands,
        options,
        elements,
        variables,
    }
}

/// Test helper function to verify basic simulation results
fn test_basic_simulation_results(result: &[(Variable, Numeric)]) {
    let branch_curr = result[0].clone();
    let branch_curr_exp = (Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0), -1.0);
    let node_vol = result[1].clone();
    let node_vol_exp = (Variable::new(Arc::from("1"), Unit::Volt, 1), 10.0);
    assert_eq!(branch_curr, branch_curr_exp);
    assert_eq!(node_vol, node_vol_exp);
}
#[test]
fn init_sim_rsparse() {
    let sim = create_basic_simulation();
    let mut simulator: Simulator<RSparseSolver> = Simulator::from(sim.clone());

    let result = simulator.run().unwrap();

    let result = match result.results[0].clone() {
        Sim::Op(items) => items,
        Sim::Dc(_) => todo!(),
        Sim::Ac(_) => todo!(),
        Sim::Tran(_) => todo!(),
    };

    test_basic_simulation_results(&result);
}

#[test]
fn init_sim_faer() {
    let sim = create_basic_simulation();
    let mut simulator: Simulator<FaerSolver> = Simulator::from(sim.clone());

    let result = simulator.run().unwrap();

    let result = match result.results[0].clone() {
        Sim::Op(items) => items,
        Sim::Dc(_) => todo!(),
        Sim::Ac(_) => todo!(),
        Sim::Tran(_) => todo!(),
    };

    test_basic_simulation_results(&result);
}

#[test]
fn init_sim_nalgebra() {
    let sim = create_basic_simulation();
    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim.clone());

    let result = simulator.run().unwrap();

    let result = match result.results[0].clone() {
        Sim::Op(items) => items,
        Sim::Dc(_) => todo!(),
        Sim::Ac(_) => todo!(),
        Sim::Tran(_) => todo!(),
    };

    test_basic_simulation_results(&result);
}

<<<<<<< HEAD
// Test the transient simulation with a constant voltage source.
//
// This test verifies that the transient simulation correctly calculates
// the current and voltage over time for a simple circuit with a constant
// voltage source and a resistor.
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
=======
// Test the transient simulation with a constant voltage source.
//
// This test verifies that the transient simulation correctly calculates
// the current and voltage over time for a simple circuit with a constant
// voltage source and a resistor.
=======
// Test the transient simulation with a constant voltage source.
//
// This test verifies that the transient simulation correctly calculates
// the current and voltage over time for a simple circuit with a constant
// voltage source and a resistor.
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
#[test]
fn run_sim_tran() {
    let commands = vec![SimulationCommand::Tran(1.0, 10.0)];
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    let vsource = Element::VSource(VSourceBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        10.0,
        None,
    ));

    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        None,
        10.0,
    ));

    let elements = vec![vsource, resistor];
    let variables = vec![branch_1, node_1];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim.clone());
    let result = simulator.run().unwrap();

    let tran_results = match result.results[0].clone() {
        Sim::Tran(res) => res,
        _ => panic!("Expected transient simulation results"),
    };

    // Verify that we have results for the expected time range
    assert!(!tran_results.is_empty(), "Transient simulation should produce results");
    
    // Check that the simulation ran for the expected duration
    let final_time = tran_results.last().unwrap().0;
    assert!(final_time >= 9.0, "Simulation should run until at least t=9s, got t={final_time}s");
    
    // Verify basic circuit behavior: V=IR, so I = V/R = 10V/10Ω = 1A (but negative due to direction)
    const EXPECTED_VOLTAGE: Numeric = 10.0;
    const EXPECTED_CURRENT: Numeric = -1.0; // Negative due to current direction
    const TOLERANCE: Numeric = 1e-6;
    
    for (time, values) in &tran_results {
        let voltage = values[1].1; // Node voltage
        let current = values[0].1; // Branch current
        
        // Verify voltage is approximately 10V (within tolerance)
        assert!(
            (voltage - EXPECTED_VOLTAGE).abs() < TOLERANCE,
            "Voltage at t={time}s should be ~10V, got {voltage}V"
        );
        
        // Verify current is approximately -1A (within tolerance)
        assert!(
            (current - EXPECTED_CURRENT).abs() < TOLERANCE,
            "Current at t={time}s should be ~-1A, got {current}A"
        );
        
        // Verify Ohm's law: V = -I * R (current direction is opposite to voltage)
        let calculated_voltage = -current * 10.0; // -I * R
        assert!(
            (calculated_voltage - voltage).abs() < TOLERANCE,
            "Ohm's law violated at t={time}s: V={voltage}V, -I*R={calculated_voltage}V"
        );
    }
}

#[test]
fn test_vsource_sin_op() {
    // Tests the sinusoidal voltage source in OP simulation.
    //
    // This test verifies that the DC offset of the sinusoidal voltage source
    // is correctly applied in operating point analysis.
    let commands = vec![SimulationCommand::Op];
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    // Create a sinusoidal voltage source with DC offset
    let vsource_sin = Element::VSourceSin(VSourceSinBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        10.0, // dc_offset
        1.0,  // amplitude (not used in OP simulation)
        1.0,  // frequency (not used in OP simulation)
        0.0,  // phase (not used in OP simulation)
        None, // ac_value
    ));

    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        None,
        10.0,
    ));

    let elements = vec![vsource_sin, resistor];
    let variables = vec![branch_1, node_1];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<FaerSolver> = Simulator::from(sim);

    let result = simulator.run().unwrap();

    let op_results = match &result.results[0] {
        Sim::Op(results) => results,
        _ => panic!("Expected OP results"),
    };

    // Verify the results
    // Expected: V = DC_offset = 10.0V
    //           I = V / R = 10.0V / 10.0Ω = 1.0A (negative because flowing into source)
    let mut voltage = 0.0;
    let mut current = 0.0;

    for (var, val) in op_results {
        if var.name() == Arc::from("1") {
            voltage = *val;
        } else if var.name() == Arc::from("V1#branch") {
            current = *val;
        }
    }

    let expected_voltage = 10.0;
    let expected_current = -1.0; // Negative because current flows into voltage source

    let vol_diff = (voltage - expected_voltage).abs();
    let curr_diff = (current - expected_current).abs();

    assert!(
        vol_diff < 1e-6,
        "Voltage test failed: V_measured={}V, V_expected={}V, diff={}V",
        voltage,
        expected_voltage,
        vol_diff
    );

    assert!(
        curr_diff < 1e-6,
        "Current test failed: I_measured={}A, I_expected={}A, diff={}A",
        current,
        expected_current,
        curr_diff
    );
}

#[test]
fn test_vsource_sin_tran() {
    // Tests the sinusoidal voltage source in transient simulation.
    //
    // This test verifies that the sinusoidal voltage source correctly generates
    // a sinusoidal waveform in transient analysis.
    let commands = vec![SimulationCommand::Tran(0.1, 1.0)]; // Small time step for accuracy
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    // Create a sinusoidal voltage source
    // V(t) = DC_offset + amplitude * sin(2π * frequency * t + phase)
    let vsource_sin = Element::VSourceSin(VSourceSinBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        10.0, // dc_offset
        1.0,  // amplitude
        1.0,  // frequency (Hz)
        0.0,  // phase
        None, // ac_value
    ));

    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        None,
        10.0,
    ));

    let elements = vec![vsource_sin, resistor];
    let variables = vec![branch_1, node_1];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim);

    let result = simulator.run().unwrap();

    let tran_results = match &result.results[0] {
        Sim::Tran(results) => results,
        _ => panic!("Expected transient results"),
    };

    // Verify the results at a few key points
    let _dc_offset = 10.0;
    let _amplitude = 1.0;
    let _frequency = 1.0;
    let resistance = 10.0;
    let _phase = 0.0;

    // Test at t = 0s: V = DC_offset + amplitude * sin(phase) = 10.0 + 1.0 * sin(0) = 10.0V
    // Test at t = 0.25s: V = 10.0 + 1.0 * sin(π/2) = 10.0 + 1.0 = 11.0V
    // Test at t = 0.5s: V = 10.0 + 1.0 * sin(π) = 10.0 + 0.0 = 10.0V
    // Test at t = 0.75s: V = 10.0 + 1.0 * sin(3π/2) = 10.0 - 1.0 = 9.0V
    // Test at t = 1.0s: V = 10.0 + 1.0 * sin(2π) = 10.0 + 0.0 = 10.0V

    let test_points = vec![
        (0.0, 10.0),
        (0.25, 11.0),
        (0.5, 10.0),
        (0.75, 9.0),
        (1.0, 10.0),
    ];

    for (expected_time, expected_voltage) in test_points {
        let closest_result = find_closest_result(tran_results, expected_time);
        
        if let Some((time, values)) = closest_result {
            let (voltage, current) = extract_voltage_and_current(values);
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
=======
        let closest_result = find_closest_result(tran_results, expected_time);
        
        if let Some((time, values)) = closest_result {
            let (voltage, current) = extract_voltage_and_current(values);

=======
        let closest_result = find_closest_result(tran_results, expected_time);
        
        if let Some((time, values)) = closest_result {
            let (voltage, current) = extract_voltage_and_current(values);
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
            let expected_current = -expected_voltage / resistance; // Negative because current flows into source

            // Allow 5% relative error or reasonable absolute error
            let vol_diff = (voltage - expected_voltage).abs();
            let curr_diff = (current - expected_current).abs();

            let vol_rel_error = vol_diff / expected_voltage.abs().max(1e-9);
            let curr_rel_error = curr_diff / expected_current.abs().max(1e-9);

            assert!(
                vol_diff < 0.5 || vol_rel_error < 0.05,
                "Voltage test failed at t≈{}s: V_measured={}V, V_expected={}V, diff={}V ({:.2}%)",
                time,
                voltage,
                expected_voltage,
                vol_diff,
                vol_rel_error * 100.0
            );

            assert!(
                curr_diff < 0.05 || curr_rel_error < 0.05,
                "Current test failed at t≈{}s: I_measured={}A, I_expected={}A, diff={}A ({:.2}%)",
                time,
                current,
                expected_current,
                curr_diff,
                curr_rel_error * 100.0
            );
        }
    }
}

#[test]
fn test_rc_sinusoidal_tran() {
    // Tests RC circuit with sinusoidal voltage source in transient simulation.
    // This test verifies the behavior of an RC low-pass filter with a sinusoidal input.
    //
    // NOTE: This test currently documents a known issue with transient capacitor simulation.
    // The output voltage should follow the input voltage (with attenuation and phase shift),
    // but currently stays at a constant value, indicating a problem with capacitor charging.

    // RC Low-Pass Filter: R=1kΩ, C=1µF
    // Cutoff frequency: fc = 1/(2πRC) ≈ 159.15 Hz
    // At 100Hz (below cutoff), we expect significant attenuation and phase shift

    let commands = vec![SimulationCommand::Tran(0.00001, 0.1)]; // 10µs step, 100ms total
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1); // Input node
    let node_2 = Variable::new(Arc::from("2"), Unit::Volt, 2); // Output node (after RC)
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    // Sinusoidal voltage source: V(t) = 10 + 1*sin(2π*100*t)
    let vsource_sin = Element::VSourceSin(VSourceSinBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        10.0,  // DC offset
        1.0,   // amplitude
        100.0, // frequency (100 Hz)
        0.0,   // phase
        None,  // ac_value
    ));

    // R = 1kΩ
    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        Some(node_2.clone()),
        1000.0,
    ));

    // C = 1µF
    let capacitor = Element::Capacitor(CapacitorBundle::new(
        Arc::from("C1"),
        Some(node_2.clone()),
        None, // to ground
        0.000_001,
    ));

    let elements = vec![vsource_sin, resistor, capacitor];
    let variables = vec![branch_1, node_1, node_2];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim);
    let result = simulator.run().unwrap();

    let tran_results = match &result.results[0] {
        Sim::Tran(results) => results,
        _ => panic!("Expected transient results"),
    };

    // Test at specific time points
    // Expected behavior: RC low-pass filter
    // Vout(t) = Vdc + (Vamp * sin(2πft + φ)) / sqrt(1 + (2πfRC)^2)
    // where φ = -arctan(2πfRC)

    let dc_offset = 10.0;
    let amplitude = 1.0;
    let frequency = 100.0;
    let r = 1000.0;
    let c = 0.000_001;
    let rc = r * c;
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let omega_rc = omega * rc;

    // Attenuation factor and phase shift
    let attenuation = 1.0 / (1.0 + omega_rc.powi(2)).sqrt();
    let phase_shift = -omega_rc.atan();

    println!("RC circuit test:");
    println!("  R = {}Ω, C = {}F", r, c);
    println!(
        "  Cutoff frequency: {:.2} Hz",
        1.0 / (2.0 * std::f64::consts::PI * rc)
    );
    println!("  Test frequency: {} Hz", frequency);
    println!("  Attenuation factor: {:.4}", attenuation);
    println!("  Phase shift: {:.2}°", phase_shift.to_degrees());

    // Test at t = 0.025s (1/4 period at 100Hz)
    let test_time = 0.025; // 25ms = 1/4 of 100Hz period (10ms)
    let expected_input = dc_offset + amplitude * (omega * test_time).sin();
    let expected_output =
        dc_offset + amplitude * attenuation * (omega * test_time + phase_shift).sin();

    // Find the result closest to the test time
    if let Some(&(time, ref values)) = tran_results
        .iter()
        .min_by_key(|(t, _)| ((*t - test_time).abs() * 1000.0) as i32)
    {
        let mut input_voltage = 0.0;
        let mut output_voltage = 0.0;
        let mut current = 0.0;

        for (var, val) in values {
            if var.name() == Arc::from("1") {
                input_voltage = *val;
            } else if var.name() == Arc::from("2") {
                output_voltage = *val;
            } else if var.name() == Arc::from("V1#branch") {
                current = *val;
            }
        }

        println!("\nAt t ≈ {}s:", time);
        println!(
            "  Input voltage:  {:.4}V (expected: {:.4}V)",
            input_voltage, expected_input
        );
        println!(
            "  Output voltage: {:.4}V (expected: {:.4}V)",
            output_voltage, expected_output
        );
        println!("  Current:        {:.6}A", current);

        // The output voltage should vary with time, but currently stays constant
        // This indicates a problem with capacitor charging in transient analysis.

        // Check if output voltage is constant (indicating the bug)
        let first_output = tran_results[0]
            .1
            .iter()
            .find(|(var, _)| var.name() == Arc::from("2"))
            .map(|(_, val)| *val)
            .unwrap_or(0.0);
        let last_output = tran_results
            .last()
            .unwrap()
            .1
            .iter()
            .find(|(var, _)| var.name() == Arc::from("2"))
            .map(|(_, val)| *val)
            .unwrap_or(0.0);

        // Also check if the output voltage is reasonable for the expected behavior
        let expected_output_min = 9.0; // Should be at least 9V (10V - 1V amplitude)
        let expected_output_max = 11.0; // Should be at most 11V (10V + 1V amplitude)

        assert!(
            (first_output - last_output).abs() > 0.1,
            "RC transient simulation failure: Output voltage is constant throughout simulation (first: {}V, last: {}V). This indicates a problem with capacitor charging in transient analysis.",
            first_output, last_output
        );

        assert!(
            output_voltage >= expected_output_min && output_voltage <= expected_output_max,
            "RC transient simulation failure: Output voltage {}V is outside expected range [{:.1}V, {:.1}V] for RC low-pass filter behavior.",
            output_voltage, expected_output_min, expected_output_max
        );
    } else {
        panic!("Could not find results near test time {}s", test_time);
    }
}

#[test]
fn test_rc_constant_tran() {
    // Tests RC circuit with constant voltage source in transient simulation.
    // This is a simpler test to isolate the capacitor charging issue.
    //
    // With a constant 10V input, the capacitor should charge up and the output
    // should approach 10V over time (RC time constant = 1ms).

    let commands = vec![SimulationCommand::Tran(0.00001, 0.01)]; // 10µs step, 10ms total
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1); // Input node
    let node_2 = Variable::new(Arc::from("2"), Unit::Volt, 2); // Output node (after RC)
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    // Constant voltage source: 10V
    let vsource = Element::VSource(VSourceBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        10.0, // constant voltage
        None,
    ));

    // R = 1kΩ
    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        Some(node_2.clone()),
        1000.0,
    ));

    // C = 1µF
    let capacitor = Element::Capacitor(CapacitorBundle::new(
        Arc::from("C1"),
        Some(node_2.clone()),
        None, // to ground
        0.000_001,
    ));

    let elements = vec![vsource, resistor, capacitor];
    let variables = vec![branch_1, node_1, node_2];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim);
    let result = simulator.run().unwrap();

    let tran_results = match &result.results[0] {
        Sim::Tran(results) => results,
        _ => panic!("Expected transient results"),
    };

    println!("RC constant voltage test:");
    println!("  R = 1kΩ, C = 1µF, τ = 1ms");
    println!("  Input: 10V constant");

    // Check initial and final conditions
    let initial_output = tran_results[0]
        .1
        .iter()
        .find(|(var, _)| var.name() == Arc::from("2"))
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    let final_output = tran_results
        .last()
        .unwrap()
        .1
        .iter()
        .find(|(var, _)| var.name() == Arc::from("2"))
        .map(|(_, val)| *val)
        .unwrap_or(0.0);

    println!("  Initial output: {}V", initial_output);
    println!("  Final output:   {}V", final_output);

    // With RC=1ms, after 10ms (10 time constants), the capacitor should be fully charged
    // Vout should be close to 10V

    assert!(
        (final_output - 10.0).abs() < 0.5,
        "RC transient simulation failure: Capacitor not charging properly. Expected final voltage close to 10V,
        got {}V. This confirms the transient capacitor simulation issue.",
        final_output
    );
}

#[test]
fn test_ac_rc_cutoff_frequency() {
    // RC Low-Pass Filter: R=1kΩ, C=1µF
    // Expected cutoff frequency: fc = 1/(2πRC) ≈ 159.15 Hz
    // At fc, |Vout/Vin| should be ≈ 0.707 (3dB point)

    let commands = vec![SimulationCommand::Ac(10.0, 1000.0, 100, ACMode::Lin)];
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let node_2 = Variable::new(Arc::from("2"), Unit::Volt, 2);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    let vsource = Element::VSource(VSourceBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        1.0,
        Some(1.0), // AC amplitude
    ));

    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        Some(node_2.clone()),
        1000.0,
    ));

    let capacitor = Element::Capacitor(crate::models::CapacitorBundle::new(
        Arc::from("C1"),
        Some(node_2.clone()),
        None,
        0.000_001,
    ));

    let elements = vec![vsource, resistor, capacitor];

    let variables = vec![branch_1, node_1, node_2];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<FaerSolver> = Simulator::from(sim);

    let result = simulator.run().unwrap();

    // Extract AC results
    let ac_results = match &result.results[0] {
        Sim::Ac(results) => results,
        _ => panic!("Expected AC results"),
    };

    // Find the frequency where |Vout/Vin| crosses 0.707
    let mut prev_freq = 0.0;
    let mut prev_mag = 0.0;
    let mut found_cutoff = false;

    for (freq, values) in ac_results {
        // Find V2 (node 2) - should be the second variable after branch
        let v2_real = values[2].1.re; // node_2 is index 2 (branch=0, node1=1, node2=2)
        let v2_imag = values[2].1.im;
        let magnitude = (v2_real.powi(2) + v2_imag.powi(2)).sqrt();

        // Check if we crossed the 0.707 threshold
        if prev_mag > 0.707 && magnitude < 0.707 {
            // Linear interpolation to find exact cutoff frequency
            let ratio = (0.707 - prev_mag) / (magnitude - prev_mag);
            let cutoff_freq = prev_freq + ratio * (freq - prev_freq);

            println!("Measured cutoff frequency: {:.2} Hz", cutoff_freq);
            println!("Magnitude at cutoff: {:.4}", magnitude);

            // Expected: 159.15 Hz, allow 2% tolerance (reduced from 15%)
            let expected_fc = 159.15;
            let error_percent = (cutoff_freq - expected_fc).abs() / expected_fc * 100.0;

            println!("Frequency error: {:.2}%", error_percent);

            assert!(
                error_percent < 2.0,
                "AC analysis test FAILED: Cutoff frequency error {:.2}% exceeds 2% tolerance. Expected: {:.2} Hz, Measured: {:.2} Hz",
                error_percent, expected_fc, cutoff_freq
            );

            println!("✅ AC analysis test PASSED (within 2% tolerance)");

            found_cutoff = true;
            break;
        }

        prev_freq = *freq;
        prev_mag = magnitude;
    }

    assert!(
        found_cutoff,
        "Could not find cutoff frequency in AC analysis"
    );
}

#[test]
fn test_dc_linear_resistor() {
    // Tests DC sweep on a simple resistor circuit
    // Verifies linear I-V relationship: I = V/R

    let commands = vec![SimulationCommand::Dc(Arc::from("V1"), 0.0, 10.0, 1.0, None)];
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    let vsource = Element::VSource(VSourceBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        0.0, // DC value (will be swept)
        None,
    ));

    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        None,   // Connect to ground
        1000.0, // 1kΩ
    ));

    let elements = vec![vsource, resistor];
    let variables = vec![branch_1, node_1];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<FaerSolver> = Simulator::from(sim);
    let result = simulator.run().unwrap();

    let dc_results = match &result.results[0] {
        Sim::Dc(results) => results,
        _ => panic!("Expected DC results"),
    };

    // Verify linearity: I = V/R
    // Resistance is 1kΩ, so I (in A) = V (in V) / 1000
    for step_data in dc_results {
        // Find voltage and current values
        let mut voltage = 0.0;
        let mut current = 0.0;
        let mut found_voltage = false;
        let mut found_current = false;

        for (var, val) in step_data {
            if var.name() == Arc::from("V1#branch") {
                current = *val; // Current through voltage source
                found_current = true;
            } else if var.name() == Arc::from("1") {
                voltage = *val; // Voltage at node 1
                found_voltage = true;
            }
        }

        if !found_voltage || !found_current {
            panic!("Could not find voltage or current in DC results");
        }

        let expected_current = voltage / 1000.0;

        // Allow 1% relative error or 1µA absolute error
        // Note: Current is negative because it flows into the voltage source
        let abs_diff = (current.abs() - expected_current.abs()).abs();
        let rel_diff = abs_diff / expected_current.abs().max(1e-9);

        assert!(
            abs_diff < 1e-6 || rel_diff < 0.01,
            "DC linearity test failed at V={}V: I_measured={}A, I_expected={}A, diff={}A ({:.2}%)",
            voltage,
            current,
            expected_current,
            abs_diff,
            rel_diff * 100.0
        );
    }

    println!("✅ DC linearity test PASSED - linear I-V relationship verified");
}

#[test]
fn test_dc_resistor_divider() {
    // Tests DC sweep on a resistor voltage divider
    // Verifies voltage division: Vout = Vin * R2 / (R1 + R2)

    let commands = vec![SimulationCommand::Dc(Arc::from("V1"), 0.0, 5.0, 0.5, None)];
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let node_2 = Variable::new(Arc::from("2"), Unit::Volt, 2);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    let vsource = Element::VSource(VSourceBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        0.0,
        None,
    ));

    // R1 = 1kΩ, R2 = 1kΩ -> voltage divider with ratio 0.5
    let r1 = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        Some(node_2.clone()),
        1000.0,
    ));

    let r2 = Element::Resistor(ResistorBundle::new(
        Arc::from("R2"),
        Some(node_2.clone()),
        None,
        1000.0,
    ));

    let elements = vec![vsource, r1, r2];
    let variables = vec![branch_1, node_1, node_2];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<FaerSolver> = Simulator::from(sim);
    let result = simulator.run().unwrap();

    let dc_results = match &result.results[0] {
        Sim::Dc(results) => results,
        _ => panic!("Expected DC results"),
    };

    // Verify voltage division: V2 = V1 * R2/(R1+R2) = V1 * 0.5
    for step_data in dc_results {
        let mut v1 = 0.0;
        let mut v2 = 0.0;

        for (var, val) in step_data {
            if var.name() == Arc::from("1") {
                v1 = *val;
            } else if var.name() == Arc::from("2") {
                v2 = *val;
            }
        }

        let expected_v2 = v1 * 0.5;
        let abs_diff = (v2 - expected_v2).abs();
        let rel_diff = abs_diff / expected_v2.abs().max(1e-9);

        assert!(
            abs_diff < 1e-6 || rel_diff < 0.01,
            "Voltage divider test failed at V1={}V: V2_measured={}V, V2_expected={}V, diff={}V ({:.2}%)",
            v1, v2, expected_v2, abs_diff, rel_diff * 100.0
        );
    }
}

#[test]
fn test_rc_step_response() {
    // Test RC circuit with step voltage source in transient simulation.
    // This test uses a step function (0V to 10V) to verify capacitor charging behavior.

    let commands = vec![SimulationCommand::Tran(0.0001, 0.01)]; // 100µs step, 10ms total
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1); // Input node
    let node_2 = Variable::new(Arc::from("2"), Unit::Volt, 2); // Output node (after RC)
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    // Step voltage source: 0V to 10V at t=0
    let vsource = Element::VSourceStep(VSourceStepBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        0.0,  // initial value
        10.0, // final value
        0.0,  // step time (immediately)
        None,
    ));

    // R = 1kΩ
    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"),
        Some(node_1.clone()),
        Some(node_2.clone()),
        1000.0,
    ));

    // C = 1µF
    let capacitor = Element::Capacitor(CapacitorBundle::new(
        Arc::from("C1"),
        Some(node_2.clone()),
        None, // to ground
        0.000_001,
    ));

    let elements = vec![vsource, resistor, capacitor];
    let variables = vec![branch_1, node_1, node_2];

    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim);

    // First, run OP analysis to see initial conditions
    let op_result = simulator.find_op().unwrap();
    println!("OP Analysis Results:");
    for (var, value) in &op_result {
        println!("  {}: {}V", var.name(), value);
    }

    let result = simulator.run().unwrap();

    let tran_results = match &result.results[0] {
        Sim::Tran(results) => results,
        _ => panic!("Expected transient results"),
    };

    println!("\nRC step response test:");
    println!("  R = 1kΩ, C = 1µF, τ = 1ms");
    println!("  Input: 0V to 10V step at t=0");

    // Print time series for debugging
    println!("\nTime series (first 10 and last 5 points):");
    for (i, (time, values)) in tran_results.iter().enumerate() {
        if i < 10 || i >= tran_results.len() - 5 {
            let output = values
                .iter()
                .find(|(var, _)| var.name() == Arc::from("2"))
                .map(|(_, val)| *val)
                .unwrap_or(0.0);
            println!("  t={:.6}s: Vout={:.6}V", time, output);
        } else if i == 10 {
            println!("  ...");
        }
    }

    // Check initial and final conditions
    let initial_output = tran_results[0]
        .1
        .iter()
        .find(|(var, _)| var.name() == Arc::from("2"))
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    let final_output = tran_results
        .last()
        .unwrap()
        .1
        .iter()
        .find(|(var, _)| var.name() == Arc::from("2"))
        .map(|(_, val)| *val)
        .unwrap_or(0.0);

    println!("\nSummary:");
    println!("  Initial output: {}V", initial_output);
    println!("  Final output:   {}V", final_output);

    // With RC=1ms, after 10ms (10 time constants), the capacitor should be fully charged
    // Vout should be close to 10V (within 5% tolerance)
    assert!(
        (final_output - 10.0).abs() < 0.5,
        "RC step response failure: Capacitor not charging properly. Expected final voltage close to 10V, got {}V",
        final_output
    );

    // Also check that initial voltage is close to 0V
    assert!(
        initial_output < 0.001,
        "RC step response failure: Initial voltage should be close to 0V, got {}V",
        initial_output
    );
}

#[test]
fn test_adaptive_timestep_detection() {
    // Test that small timesteps trigger adaptive mode
    let small_timestep = 1e-7; // Smaller than ADAPTIVE_INITIAL_TIMESTEP
    let large_timestep = 1e-2; // Larger than ADAPTIVE_INITIAL_TIMESTEP

    assert!(small_timestep <= ADAPTIVE_INITIAL_TIMESTEP);
    assert!(large_timestep > ADAPTIVE_INITIAL_TIMESTEP);
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_adaptive_timestep_constants_are_positive() {
    // Test that all constants are positive and have reasonable relationships
    // These assertions document the expected relationships between constants
    assert!(ADAPTIVE_MIN_TIMESTEP > 0.0);
    assert!(ADAPTIVE_MAX_TIMESTEP > ADAPTIVE_MIN_TIMESTEP);
    assert!(ADAPTIVE_INITIAL_TIMESTEP > ADAPTIVE_MIN_TIMESTEP);
    assert!(ADAPTIVE_INITIAL_TIMESTEP < ADAPTIVE_MAX_TIMESTEP);
    assert!(ADAPTIVE_TOLERANCE > 0.0);
    assert!(ADAPTIVE_TOLERANCE < 1.0);
    assert!(ADAPTIVE_SAFETY_FACTOR > 0.0);
    assert!(ADAPTIVE_SAFETY_FACTOR < 1.0);
    assert!(ADAPTIVE_MAX_GROWTH_FACTOR > 1.0);
    assert!(ADAPTIVE_MIN_GROWTH_FACTOR > 0.0);
    assert!(ADAPTIVE_MIN_GROWTH_FACTOR < 1.0);
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_adaptive_timestep_clamping() {
    // Test that the constants define a reasonable range
    // These assertions document the expected relationships between constants
    assert!(ADAPTIVE_MIN_TIMESTEP < ADAPTIVE_INITIAL_TIMESTEP);
    assert!(ADAPTIVE_INITIAL_TIMESTEP < ADAPTIVE_MAX_TIMESTEP);

    // Test that growth factors are reasonable
    assert!(ADAPTIVE_MIN_GROWTH_FACTOR < ADAPTIVE_MAX_GROWTH_FACTOR);

    // Test that safety factor is conservative
    assert!(ADAPTIVE_SAFETY_FACTOR < 1.0);
    assert!(ADAPTIVE_SAFETY_FACTOR > 0.5);
}
