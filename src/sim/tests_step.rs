use std::sync::Arc;

use crate::sim::commands::SimulationCommand;
use crate::sim::Simulator;
use crate::models::Element;
use crate::models::vsource_step::VSourceStepBundle;
use crate::models::resistor::ResistorBundle;
use crate::models::capacitor::CapacitorBundle;
use crate::models::Variable;
use crate::models::Unit;
use crate::solver::NalgebraSolver;

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
        0.0,    // initial value
        10.0,   // final value
        0.0,    // step time (immediately)
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

    let sim = crate::sim::Simulation {
        commands,
        options,
        elements,
        variables,
    };

    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim);
    let result = simulator.run().unwrap();

    let tran_results = match &result.results[0] {
        crate::sim::simulation_result::Sim::Tran(results) => results,
        _ => panic!("Expected transient results"),
    };

    println!("RC step response test:");
    println!("  R = 1kΩ, C = 1µF, τ = 1ms");
    println!("  Input: 0V to 10V step at t=0");

    // Check initial and final conditions
    let initial_output = tran_results[0].1.iter().find(|(var, _)| var.name() == Arc::from("2")).map(|(_, val)| *val).unwrap_or(0.0);
    let final_output = tran_results.last().unwrap().1.iter().find(|(var, _)| var.name() == Arc::from("2")).map(|(_, val)| *val).unwrap_or(0.0);

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
        initial_output < 0.1,
        "RC step response failure: Initial voltage should be close to 0V, got {}V",
        initial_output
    );

    println!("✅ RC step response test PASSED");
}