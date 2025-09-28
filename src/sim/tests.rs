use std::sync::Arc;

use crate::frontends::Simulation;
use crate::models::{Element, ResistorBundle, Unit, VSourceBundle, Variable};
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
