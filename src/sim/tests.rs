use std::sync::Arc;

use crate::{
    models::{ISourceBundle, ResistorBundle, Unit, VSourceBundle, Variable},
    solver::NalgebraSolver,
};

use approx::relative_eq;

use super::*;

// Mock Backend für Testzwecke
struct MockBackend;

impl Solver for MockBackend {
    fn new(_: usize) -> Result<Self, SolverError>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn set_a(&mut self, _: &Triples) {}

    fn set_b(&mut self, _: &Pairs) {}

    fn solve(&mut self) -> Result<&Vec<f64>, SolverError> {
        Err(SolverError::MatrixNonInvertible)
    }

    //fn insert_a(&mut self, _: &Triples) {}

    //fn insert_b(&mut self, _: &Pairs) {}
}

fn create_mock_elements(vars: &Vec<Variable>) -> Vec<Element> {
    let res = Element::Resistor(ResistorBundle::new(
        Arc::from("r1"),
        None,
        Some(vars[1].clone()),
        10.0,
    ));

    let vol = Element::VSource(VSourceBundle::new(
        Arc::from("v1"),
        vars[0].clone(),
        None,
        Some(vars[1].clone()),
        10.0,
        None,
    ));

    vec![res, vol]
}

fn create_mock_elements2(vars: &Vec<Variable>) -> Vec<Element> {
    let vol = Element::VSource(VSourceBundle::new(
        Arc::from("v1"),
        vars[0].clone(),
        None,
        Some(vars[1].clone()),
        10.0,
        None,
    ));

    let res1 = Element::Resistor(ResistorBundle::new(
        Arc::from("r1"),
        Some(vars[1].clone()),
        Some(vars[2].clone()),
        10.0,
    ));

    let res2 = Element::Resistor(ResistorBundle::new(
        Arc::from("r2"),
        Some(vars[2].clone()),
        None,
        10.0,
    ));

    vec![vol, res1, res2]
}
fn create_mock_elements3() -> Vec<Element> {
    let vol1 = Element::VSource(VSourceBundle::new(
        Arc::from("v1"),
        Variable::new(Arc::from("v1#branch"), Unit::Volt, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        None,
    ));

    let vol2 = Element::VSource(VSourceBundle::new(
        Arc::from("v2"),
        Variable::new(Arc::from("v2#branch"), Unit::Volt, 2),
        None,
        Some(Variable::new(Arc::from("3"), Unit::Volt, 3)),
        10.0,
        None,
    ));

    let res1 = Element::Resistor(ResistorBundle::new(
        Arc::from("r1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        Some(Variable::new(Arc::from("2"), Unit::Volt, 4)),
        10.0,
    ));

    let res2 = Element::Resistor(ResistorBundle::new(
        Arc::from("r2"),
        Some(Variable::new(Arc::from("2"), Unit::Volt, 4)),
        Some(Variable::new(Arc::from("3"), Unit::Volt, 3)),
        10.0,
    ));

    let res3 = Element::Resistor(ResistorBundle::new(
        Arc::from("r2"),
        Some(Variable::new(Arc::from("2"), Unit::Volt, 4)),
        None,
        10.0,
    ));
    vec![vol1, vol2, res1, res2, res3]
}

fn create_mock_elements4(vars: &Vec<Variable>) -> Vec<Element> {
    let res = Element::Resistor(ResistorBundle::new(
        Arc::from("r1"),
        None,
        Some(vars[0].clone()),
        10.0,
    ));

    let vol = Element::ISource(ISourceBundle::new(
        Arc::from("i1"),
        None,
        Some(vars[0].clone()),
        1.0,
    ));

    vec![res, vol]
}

#[test]
fn test_from_simulation() {
    let variables = vec![
        Variable::new(Arc::from("1"), Unit::Volt, 0),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0),
    ];
    let elements = create_mock_elements(&variables);
    let commands = vec![SimulationCommand::Op];
    let sim = Simulation {
        variables,
        elements,
        commands,
    };

    let _: Simulator<NalgebraSolver> = Simulator::from(sim);
}

/// Test the simulation of a simple circuit with a voltage source (V1) and a resistor (R1).
///
/// This test verifies that the simulation correctly computes the voltage at node 1
/// and the current through the voltage source branch.
#[test]
fn test_run_simulation() {
    let variables = vec![
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0),
        Variable::new(Arc::from("1"), Unit::Volt, 1),
    ];
    let elements = create_mock_elements(&variables);
    let commands = vec![SimulationCommand::Op];
    let sim = Simulation {
        variables,
        elements,
        commands,
    };

    let mut sim: Simulator<NalgebraSolver> = Simulator::from(sim);

    let res = sim.run().unwrap().0[0].clone();
    let res = match res {
        Sim::Op(res) => res,
        Sim::Dc(_) => unimplemented!(),
        Sim::Ac(_) => unimplemented!(),
    };
    let node1_volt = res[0].clone();
    let exp_node1_volt = Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0);
    assert_eq!(node1_volt.0, exp_node1_volt);

    let branch_curr = res[1].clone();
    let exp_branch_curr = Variable::new(Arc::from("1"), Unit::Volt, 1);
    assert_eq!(branch_curr.0, exp_branch_curr);
}

/// Test the simulation of a circuit with a voltage source (V1), resistor (R1), and another node (2).
///
/// This test verifies that the simulation correctly computes the current through the voltage source branch,
/// the voltage at node 1, and the voltage at node 2.
#[test]
fn test_run_simulation2() {
    let variables = vec![
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0),
        Variable::new(Arc::from("1"), Unit::Volt, 1),
        Variable::new(Arc::from("2"), Unit::Volt, 2),
    ];
    let elements = create_mock_elements2(&variables);
    let commands = vec![SimulationCommand::Op];
    let sim = Simulation {
        variables,
        elements,
        commands,
    };

    let mut sim: Simulator<NalgebraSolver> = Simulator::from(sim);

    let res = sim.run().unwrap().0[0].clone();
    let res = match res {
        Sim::Op(res) => res,
        Sim::Dc(_) => unimplemented!(),
        Sim::Ac(_) => unimplemented!(),
    };
    let branch_curr = res[0].clone();
    let exp_branch_curr = -0.5;
    assert_eq!(branch_curr.1, exp_branch_curr);

    let node1_volt = res[1].clone();
    let exp_node1_volt = 10.0;
    assert_eq!(node1_volt.1, exp_node1_volt);

    let node1_volt = res[2].clone();
    let exp_node1_volt = 5.0;
    assert_eq!(node1_volt.1, exp_node1_volt);
}

/// Test the simulation of a more complex circuit with multiple voltage sources and nodes.
///
/// This test verifies that the simulation correctly computes the currents through the voltage source branches
/// and the voltages at the respective nodes, ensuring the results are consistent with the expected values.
#[test]
fn test_run_simulation3() {
    let variables = vec![
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0),
        Variable::new(Arc::from("1"), Unit::Volt, 1),
        Variable::new(Arc::from("v2#branch"), Unit::Ampere, 2),
        Variable::new(Arc::from("2"), Unit::Volt, 3),
        Variable::new(Arc::from("3"), Unit::Volt, 4),
    ];
    let elements = create_mock_elements3();
    let commands = vec![SimulationCommand::Op];
    let sim = Simulation {
        variables: variables.clone(),
        elements,
        commands,
    };

    let mut sim: Simulator<NalgebraSolver> = Simulator::from(sim);
    println!("Vars: {:?}", sim.vars);

    let res = sim.run().unwrap().0[0].clone();
    let res = match res {
        Sim::Op(res) => res,
        Sim::Dc(_) => unimplemented!(),
        Sim::Ac(_) => unimplemented!(),
    };

    let var = res[0].clone();
    let exp = (variables[0].clone(), -0.33333333333333337);
    assert_eq!(var, exp);

    let var = res[1].clone();
    let exp = (variables[1].clone(), 10.0);
    assert_eq!(var, exp);

    let var = res[2].clone();
    let exp = (variables[2].clone(), -0.33333333333333337);
    assert_eq!(var, exp);

    let var = res[3].clone();
    let exp = (variables[3].clone(), 10.0);
    assert_eq!(var, exp);

    let var = res[4].clone();
    let exp: (&str, f64) = ("3", 6.6666666666666666);
    assert_eq!(*var.0.name(), *exp.0);
    assert!(relative_eq!(var.1, exp.1, epsilon = f64::EPSILON));
}

/// Test to ensure the consistency of results in witch current sources.
#[test]
fn test_run_simulation4() {
    let variables = vec![Variable::new(Arc::from("1"), Unit::Volt, 0)];
    let elements = create_mock_elements4(&variables);
    let commands = vec![SimulationCommand::Op];
    let sim = Simulation {
        variables,
        elements,
        commands,
    };

    let mut sim: Simulator<NalgebraSolver> = Simulator::from(sim);

    let res = sim.run().unwrap().0[0].clone();
    let res = match res {
        Sim::Op(res) => res,
        Sim::Dc(_) => unimplemented!(),
        Sim::Ac(_) => unimplemented!(),
    };
    let node1_volt = res[0].clone();
    let exp_node1_volt = 10.0;
    assert_eq!(node1_volt.1, exp_node1_volt);
}

#[test]
fn test_build_constant_a_mat() {
    let variables = vec![
        Variable::new(Arc::from("1"), Unit::Volt, 1),
        Variable::new(Arc::from("1"), Unit::Volt, 1),
    ];
    let elements = create_mock_elements(&variables);
    let simulator = Simulator {
        elements,
        commands: vec![],
        vars: variables,
        backend: MockBackend,
    };
    let result = simulator.build_constant_a_mat();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_build_constant_a_mat_empty() {
    let simulator = Simulator {
        elements: vec![],
        commands: vec![],
        vars: vec![],
        backend: MockBackend,
    };
    let result = simulator.build_constant_a_mat();
    result.unwrap();
}

#[test]
fn test_build_constant_b_vec() {
    let variables = vec![
        Variable::new(Arc::from("1"), Unit::Volt, 1),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 1),
    ];
    let elements = create_mock_elements(&variables);
    let simulator = Simulator {
        elements,
        commands: vec![],
        vars: variables,
        backend: MockBackend,
    };
    let result = simulator.build_constant_b_vec();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_build_constant_b_vec_empty() {
    let simulator = Simulator {
        elements: vec![],
        commands: vec![],
        vars: vec![],
        backend: MockBackend,
    };
    let result = simulator.build_constant_b_vec();
    result.unwrap();
}

#[test]
fn test_build_time_variant_b_vec() {
    let variables = vec![
        Variable::new(Arc::from("1"), Unit::Volt, 1),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 1),
    ];
    let elements = create_mock_elements(&variables);
    let simulator = Simulator {
        elements,
        commands: vec![],
        vars: variables,
        backend: MockBackend,
    };
    let result = simulator.build_time_variant_b_vec();
    assert!(result.is_empty());
}

#[test]
fn test_build_time_variant_b_vec_empty() {
    let simulator = Simulator {
        elements: vec![],
        commands: vec![],
        vars: vec![],
        backend: MockBackend,
    };
    let result = simulator.build_time_variant_b_vec();
    assert!(result.is_empty());
}

#[test]
fn test_build_nonlinear_b_vec() {
    let variables = vec![
        Variable::new(Arc::from("1"), Unit::Volt, 1),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 1),
    ];
    let x_vec = vec![1.0, 2.0];
    let elements = create_mock_elements(&variables);
    let simulator = Simulator {
        elements,
        commands: vec![],
        vars: variables,
        backend: MockBackend,
    };
    let result = simulator.build_nonlinear_b_vec(&x_vec);
    assert!(result.is_empty());
}
