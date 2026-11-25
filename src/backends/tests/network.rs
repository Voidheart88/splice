use crate::backends::NetworkBackend;
use crate::models::Unit;
use crate::sim::simulation_result::{Sim, SimulationResults};
use num::Complex;
use std::sync::Arc;

#[test]
fn test_convert_sim_result_op() {
    let var = Variable::new(Arc::from("v1"), Unit::Volt, 0);
    let sim = Sim::Op(vec![(var, 3.3)]);
    let results = NetworkBackend::convert_sim_result(sim);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].r#type, "op");
    assert_eq!(results[0].variables.len(), 1);
    assert_eq!(results[0].variables[0].name, "v1");
    assert_eq!(results[0].variables[0].unit, "V");
    assert_eq!(results[0].variables[0].value, 3.3);
}

#[test]
fn test_convert_sim_result_dc() {
    let var = Variable::new(Arc::from("i1"), Unit::Ampere, 0);
    let sim = Sim::Dc(vec![vec![(var.clone(), 0.1)], vec![(var, 0.2)]]);
    let results = NetworkBackend::convert_sim_result(sim);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].r#type, "dc_step_0");
    assert_eq!(results[0].variables[0].value, 0.1);
    assert_eq!(results[1].r#type, "dc_step_1");
    assert_eq!(results[1].variables[0].value, 0.2);
}

#[test]
fn test_convert_sim_result_ac() {
    let var = Variable::new(Arc::from("out"), Unit::Volt, 0);
    let sim = Sim::Ac(vec![(1000.0, vec![(var, Complex::new(2.5, 1.0))])]);
    let results = NetworkBackend::convert_sim_result(sim);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].r#type, "ac_1000");
    assert_eq!(results[0].variables[0].value, 2.5);
}

#[test]
fn test_convert_sim_result_tran() {
    let var = Variable::new(Arc::from("node1"), Unit::Volt, 0);
    let sim = Sim::Tran(vec![(0.01, vec![(var, 4.2)])]);
    let results = NetworkBackend::convert_sim_result(sim);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].r#type, "tran_0.01");
    assert_eq!(results[0].variables[0].value, 4.2);
}
