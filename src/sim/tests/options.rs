use crate::sim::{IntegrationMethodEnum, SimulationOption};
use std::sync::Arc;

#[test]
fn test_serialize_out() {
    let opt = SimulationOption::Out(vec![Arc::from("v1"), Arc::from("v2")]);
    let json = serde_json::to_string(&opt).unwrap();
    assert_eq!(json, r#"{"type":"out","variables":["v1","v2"]}"#);
}

#[test]
fn test_serialize_integration_method() {
    let opt = SimulationOption::IntegrationMethod(IntegrationMethodEnum::BackwardEuler);
    let json = serde_json::to_string(&opt).unwrap();
    assert_eq!(json, r#"{"type":"integration","method":"be"}"#);

    let opt_gear = SimulationOption::IntegrationMethod(IntegrationMethodEnum::GearBdf { order: 2 });
    let json_gear = serde_json::to_string(&opt_gear).unwrap();
    assert_eq!(json_gear, r#"{"type":"integration","method":{"gear":{"order":2}}}"#);
}

#[test]
fn test_integration_method_to_object() {
    let be = IntegrationMethodEnum::BackwardEuler;
    let method = be.to_integration_method(1e-6);
    assert_eq!(method.name(), "Backward Euler");
    assert!(!method.is_multistep());
}

#[test]
fn test_bdf_integration_method_to_object() {
    let bdf = IntegrationMethodEnum::GearBdf { order: 2 };
    let method = bdf.to_integration_method(1e-6);
    assert_eq!(method.name(), "BDF2");
    assert!(method.is_multistep());
}
