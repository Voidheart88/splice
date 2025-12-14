use std::sync::Arc;

use crate::frontends::Simulation;
use crate::models::{Element, ResistorBundle, Unit, VSourceBundle, VSourceSinBundle, Variable};
use crate::sim::commands::SimulationCommand;
use crate::sim::simulation_result::Sim;
use crate::sim::Simulator;
use crate::solver::{FaerSolver, NalgebraSolver, RSparseSolver};

#[test]
fn init_sim_rsparse() {
    let commands = vec![SimulationCommand::Op];
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

    let mut simulator: Simulator<RSparseSolver> = Simulator::from(sim.clone());

    let result = simulator.run().unwrap();

    let result = match result.results[0].clone() {
        Sim::Op(items) => items,
        Sim::Dc(_) => todo!(),
        Sim::Ac(_) => todo!(),
        Sim::Tran(_) => todo!(),
    };

    let branch_curr = result[0].clone();
    let branch_curr_exp = (Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0), -1.0);
    let node_vol = result[1].clone();
    let node_vol_exp = (Variable::new(Arc::from("1"), Unit::Volt, 1), 10.0);
    assert_eq!(branch_curr, branch_curr_exp);
    assert_eq!(node_vol, node_vol_exp);
}

#[test]
fn init_sim_faer() {
    let commands = vec![SimulationCommand::Op];
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

    let mut simulator: Simulator<FaerSolver> = Simulator::from(sim.clone());

    let result = simulator.run().unwrap();

    let result = match result.results[0].clone() {
        Sim::Op(items) => items,
        Sim::Dc(_) => todo!(),
        Sim::Ac(_) => todo!(),
        Sim::Tran(_) => todo!(),
    };

    let branch_curr = result[0].clone();
    let branch_curr_exp = (Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0), -1.0);
    let node_vol = result[1].clone();
    let node_vol_exp = (Variable::new(Arc::from("1"), Unit::Volt, 1), 10.0);
    assert_eq!(branch_curr, branch_curr_exp);
    assert_eq!(node_vol, node_vol_exp);
}

#[test]
fn init_sim_nalgebra() {
    let commands = vec![SimulationCommand::Op];
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

    let result = match result.results[0].clone() {
        Sim::Op(items) => items,
        Sim::Dc(_) => todo!(),
        Sim::Ac(_) => todo!(),
        Sim::Tran(_) => todo!(),
    };

    let branch_curr = result[0].clone();
    let branch_curr_exp = (Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0), -1.0);
    let node_vol = result[1].clone();
    let node_vol_exp = (Variable::new(Arc::from("1"), Unit::Volt, 1), 10.0);
    assert_eq!(branch_curr, branch_curr_exp);
    assert_eq!(node_vol, node_vol_exp);
}

#[test]
fn run_sim_tran() {
    /// Tests the transient simulation with a constant voltage source.
    ///
    /// This test verifies that the transient simulation correctly calculates
    /// the current and voltage over time for a simple circuit with a constant
    /// voltage source and a resistor.
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

    let result = match result.results[0].clone() {
        Sim::Tran(res) => res,
        _ => todo!(),
    };

    for res in result {
        let time = res.0;
        let curr = res.1[0].1;
        let vol = res.1[1].1;
        println!("{time}, {curr:?}, {vol:?}")
    }
}

#[test]
fn run_sim_tran_sin() {
    /// Tests the transient simulation with a sinusoidal voltage source.
    ///
    /// This test verifies that the transient simulation correctly calculates
    /// the current and voltage over time for a simple circuit with a sinusoidal
    /// voltage source and a resistor.
    let commands = vec![SimulationCommand::Tran(0.1, 10.0)];
    let options = vec![];

    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let branch_1 = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);

    // Create a sinusoidal voltage source
    let vsource_sin = Element::VSourceSin(VSourceSinBundle::new(
        Arc::from("V1"),
        branch_1.clone(),
        None,
        Some(node_1.clone()),
        10.0, // dc_offset
        1.0,  // amplitude
        1.0,  // frequency (Hz)
        0.0,  // phase
        None, // ac_value - this is the missing parameter
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

    let mut simulator: Simulator<NalgebraSolver> = Simulator::from(sim.clone());

    let result = simulator.run().unwrap();

    let result = match result.results[0].clone() {
        Sim::Tran(res) => res,
        _ => todo!(),
    };

    // Verify the results
    for res in result {
        let time = res.0;
        let curr = res.1[0].1;
        let vol = res.1[1].1;
        println!("{time}, {curr:?}, {vol:?}")
    }
}
