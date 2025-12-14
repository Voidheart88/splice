use std::sync::Arc;

use crate::models::vsource_step::VSourceStepBundle;
use crate::models::{Unit, Variable};

#[test]
fn test_step_source_creation() {
    let branch = Variable::new(Arc::from("branch_V1"), Unit::Ampere, 0);
    let node0 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let node1 = Variable::new(Arc::from("0"), Unit::Volt, 2);
    
    let step_source = VSourceStepBundle::new(
        Arc::from("V1"),
        branch,
        Some(node0),
        Some(node1),
        0.0,    // initial value
        10.0,   // final value
        0.001,  // step time
        None,
    );
    
    assert_eq!(step_source.name(), Arc::from("V1"));
    assert_eq!(step_source.initial_value, 0.0);
    assert_eq!(step_source.final_value, 10.0);
    assert_eq!(step_source.step_time, 0.001);
}

#[test]
fn test_step_source_pairs_before_step() {
    let branch = Variable::new(Arc::from("branch_V1"), Unit::Ampere, 0);
    let node0 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let node1 = Variable::new(Arc::from("0"), Unit::Volt, 2);
    
    let step_source = VSourceStepBundle::new(
        Arc::from("V1"),
        branch,
        Some(node0),
        Some(node1),
        0.0,
        10.0,
        0.001,
        None,
    );
    
    // Before step time
    let pairs = step_source.pairs(Some(&0.0005));
    let pairs_data = pairs.data();
    assert_eq!(pairs_data.len(), 2); // Branch and node
    assert_eq!(pairs_data[0].1, 0.0); // Should be initial value
}

#[test]
fn test_step_source_pairs_after_step() {
    let branch = Variable::new(Arc::from("branch_V1"), Unit::Ampere, 0);
    let node0 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let node1 = Variable::new(Arc::from("0"), Unit::Volt, 2);
    
    let step_source = VSourceStepBundle::new(
        Arc::from("V1"),
        branch,
        Some(node0),
        Some(node1),
        0.0,
        10.0,
        0.001,
        None,
    );
    
    // After step time
    let pairs = step_source.pairs(Some(&0.0015));
    let pairs_data = pairs.data();
    assert_eq!(pairs_data.len(), 2); // Branch and node
    assert_eq!(pairs_data[0].1, 10.0); // Should be final value
}

#[test]
fn test_step_source_pairs_op_analysis() {
    let branch = Variable::new(Arc::from("branch_V1"), Unit::Ampere, 0);
    let node0 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let node1 = Variable::new(Arc::from("0"), Unit::Volt, 2);
    
    let step_source = VSourceStepBundle::new(
        Arc::from("V1"),
        branch,
        Some(node0),
        Some(node1),
        5.0,   // initial value
        10.0,
        0.001,
        None,
    );
    
    // OP analysis (no time)
    let pairs = step_source.pairs(None);
    let pairs_data = pairs.data();
    assert_eq!(pairs_data.len(), 2); // Branch and node
    assert_eq!(pairs_data[0].1, 5.0); // Should be initial value
}