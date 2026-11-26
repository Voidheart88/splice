//! Tests for integration methods

use crate::sim::integration::{
    BDF, BackwardEuler, ForwardEuler, IntegrationMethod, MultiStepMethod, Trapezoidal,
    default_integration_method,
};

#[test]
fn test_backward_euler() {
    let method = BackwardEuler::new(1e-6);
    assert_eq!(method.name(), "Backward Euler");
    assert_eq!(method.delta_t(), 1e-6);
    assert_eq!(method.order(), 1);
    assert!(!method.is_multistep());
}

#[test]
fn test_forward_euler() {
    let method = ForwardEuler::new(1e-6);
    assert_eq!(method.name(), "Forward Euler");
    assert_eq!(method.delta_t(), 1e-6);
    assert_eq!(method.order(), 1);
    assert!(!method.is_multistep());
}

#[test]
fn test_trapezoidal() {
    let method = Trapezoidal::new(1e-6);
    assert_eq!(method.name(), "Trapezoidal");
    assert_eq!(method.delta_t(), 1e-6);
    assert_eq!(method.order(), 2);
    assert!(!method.is_multistep());
}

#[test]
fn test_bdf1() {
    let method = BDF::new(1e-6, 1);
    assert_eq!(method.name(), "BDF1");
    assert_eq!(method.delta_t(), 1e-6);
    assert_eq!(method.order(), 1);
    assert!(method.is_multistep());
}

#[test]
fn test_bdf2() {
    let method = BDF::new(1e-6, 2);
    assert_eq!(method.name(), "BDF2");
    assert_eq!(method.order(), 2);
    assert!(method.is_multistep());
}

#[test]
fn test_bdf3() {
    let method = BDF::new(1e-6, 3);
    assert_eq!(method.name(), "BDF3");
    assert_eq!(method.order(), 3);
}

#[test]
#[should_panic(expected = "BDF order must be between 1 and 6")]
fn test_bdf_invalid_order_zero() {
    BDF::new(1e-6, 0);
}

#[test]
#[should_panic(expected = "BDF order must be between 1 and 6")]
fn test_bdf_invalid_order_too_high() {
    BDF::new(1e-6, 7);
}

#[test]
fn test_bdf_coefficients() {
    // BDF1: y(t+dt) - y(t) = dt*f
    let coeffs1 = BDF::bdf_coefficients(1);
    assert!((coeffs1[0] - 1.0).abs() < 1e-10);
    assert!((coeffs1[1] - (-1.0)).abs() < 1e-10);

    // BDF2: (4/3)*y(t+dt) - (1/3)*y(t) = (2/3)*dt*f
    let coeffs2 = BDF::bdf_coefficients(2);
    assert!((coeffs2[0] - 4.0 / 3.0).abs() < 1e-10);
    assert!((coeffs2[1] - (-1.0 / 3.0)).abs() < 1e-10);
    assert!((coeffs2[2] - (-1.0)).abs() < 1e-10);
}

#[test]
fn test_capacitor_history() {
    let mut method = BDF::new(1e-6, 3);

    // Push history values
    // Note: push_capacitor_history inserts at position 0 (most recent first)
    // After truncation to 3 values, we get [most_recent, ..., oldest]
    method.push_capacitor_history(1.0);
    method.push_capacitor_history(0.9);
    method.push_capacitor_history(0.8);
    method.push_capacitor_history(0.7); // Should be truncated, keeping [0.7, 0.8, 0.9]

    let history = method.capacitor_history();
    assert_eq!(history.len(), 3); // Should only keep 3 values
    // history[0] = 0.7 (most recent), history[1] = 0.8, history[2] = 0.9 (oldest)
    assert!(
        (history[0] - 0.7).abs() < 1e-9,
        "history[0] = {}, expected 0.7",
        history[0]
    );
    assert!(
        (history[1] - 0.8).abs() < 1e-9,
        "history[1] = {}, expected 0.8",
        history[1]
    );
    assert!(
        (history[2] - 0.9).abs() < 1e-9,
        "history[2] = {}, expected 0.9",
        history[2]
    );
}

#[test]
fn test_history_clearing() {
    let mut method = BDF::new(1e-6, 2);
    method.push_capacitor_history(1.0);
    method.push_capacitor_history(0.9);
    assert_eq!(method.capacitor_history().len(), 2);

    method.clear_history();
    assert!(method.capacitor_history().is_empty());
}

#[test]
fn test_capacitor_history_contribution() {
    let method = BDF::new(1e-6, 2);
    let c = 1e-9; // 1 nF

    // With BDF2: (4/3)*V_current - (1/3)*V_prev = (2/3)*dt * I
    // History contribution should account for the -(-1/3)/(4/3) = 1/4 factor

    let contribution = method.capacitor_history_contribution(c);
    // Since history is empty, contribution should be 0
    assert!((contribution - 0.0).abs() < 1e-10);
}

#[test]
fn test_integration_method_default() {
    let method = default_integration_method();
    assert_eq!(method.name(), "Backward Euler");
}
