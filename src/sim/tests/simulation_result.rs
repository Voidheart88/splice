//! Tests for simulation result serialization

use crate::models::Unit;
use crate::models::Variable;
use crate::sim::simulation_result::Sim;
use num::Complex;
use std::sync::Arc;

#[test]
fn test_serialize_op() {
    let var = Variable::new(Arc::from("node1"), Unit::Volt, 0);
    let sim = Sim::Op(vec![(var, 1.23)]);
    let json = serde_json::to_string(&sim).unwrap();
    assert!(json.contains(r#""type":"op""#));
    assert!(json.contains("1.23"));
}

#[test]
fn test_serialize_dc() {
    let var = Variable::new(Arc::from("node1"), Unit::Volt, 0);
    let sim = Sim::Dc(vec![vec![(var, 0.5)]]);
    let json = serde_json::to_string(&sim).unwrap();
    assert!(json.contains(r#""type":"dc""#));
    assert!(json.contains("0.5"));
}

#[test]
fn test_serialize_tran() {
    let var = Variable::new(Arc::from("node1"), Unit::Volt, 0);
    let sim = Sim::Tran(vec![(0.001, vec![(var, 2.5)])]);
    let json = serde_json::to_string(&sim).unwrap();
    assert!(json.contains(r#""type":"tran""#));
    assert!(json.contains("2.5"));
}

#[test]
fn test_serialize_ac() {
    let var = Variable::new(Arc::from("node1"), Unit::Volt, 0);
    let sim = Sim::Ac(vec![(1000.0, vec![(var, Complex::new(1.0, -0.5))])]);
    let json = serde_json::to_string(&sim).unwrap();
    assert!(json.contains(r#""type":"ac""#));
    assert!(json.contains("1000"));
}
