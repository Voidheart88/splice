use std::sync::Arc;

use crate::frontends::Simulation;
use crate::sim::commands::SimulationCommand;
use crate::models::{Element, ResistorBundle, Unit, VSourceBundle, Variable};
use crate::solver::RSparseSolver;
use crate::Simulator;


#[test]
fn init_sim() {
    let commands = vec!(SimulationCommand::Op);
    let options = vec!();
    
    let node_1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let branch_1 = Variable::new(Arc::from("V1#Branch"), Unit::Ampere, 0);
    
    let vsource = Element::VSource(VSourceBundle::new(
        Arc::from("V1"), 
        branch_1.clone(), 
        Some(node_1.clone()), 
        None, 
        10.0, 
        None,
    ));
    
    let resistor = Element::Resistor(ResistorBundle::new(
        Arc::from("R1"), 
        Some(node_1.clone()), 
        None, 
        10.0
    ));
    
    let elements = vec!(vsource,resistor);
    
    let variables = vec!(node_1,branch_1);
    
    let sim = Simulation {
        commands,
        options,
        elements,
        variables,
    };
    
    let mut simulator: Simulator<RSparseSolver> = Simulator::from(sim);    
    let results = simulator.run().unwrap().results;
    
    println!("{:?}", results);
}