//! Tests for Newton-Armijo method

use crate::sim::newton_armijo::{ArmijoParameters, NewtonArmijo};

#[test]
fn test_armijo_parameters_default() {
    let params = ArmijoParameters::default();
    assert_eq!(params.beta, 0.2);
    assert_eq!(params.rho, 0.6);
    assert_eq!(params.alpha_min, 1e-8);
    assert_eq!(params.max_backtrack, 20);
}

#[test]
fn test_armijo_parameters_custom() {
    let params = ArmijoParameters {
        beta: 0.3,
        rho: 0.5,
        alpha_min: 1e-10,
        max_backtrack: 30,
    };
    let armijo = NewtonArmijo::with_parameters(params);
    assert_eq!(armijo.parameters().beta, 0.3);
    assert_eq!(armijo.parameters().rho, 0.5);
}
