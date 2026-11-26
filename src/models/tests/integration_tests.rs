// Integration tests for different integration methods
use crate::sim::integration::{
    BDF, BackwardEuler, ForwardEuler, IntegrationMethod, MultiStepMethod, Trapezoidal,
};

#[test]
fn test_backward_euler_name() {
    let method = BackwardEuler::new(1e-6);
    assert_eq!(method.name(), "Backward Euler");
}

#[test]
fn test_backward_euler_delta_t() {
    let method = BackwardEuler::new(1e-6);
    assert!((method.delta_t() - 1e-6).abs() < 1e-12);
}

#[test]
fn test_backward_euler_capacitor_conductance() {
    let method = BackwardEuler::new(1e-6);
    let c = 1e-9; // 1 nF
    let g = method.capacitor_equivalent_conductance(c);
    // For Backward Euler: G_eq = C / delta_t = 1e-9 / 1e-6 = 1e-3
    assert!((g - 1e-3).abs() < 1e-10);
}

#[test]
fn test_backward_euler_inductor_conductance() {
    let method = BackwardEuler::new(1e-6);
    let l = 1e-6; // 1 uH
    let g = method.inductor_equivalent_conductance(l);
    // For Backward Euler: G_eq = delta_t / L = 1e-6 / 1e-6 = 1
    assert!((g - 1.0).abs() < 1e-10);
}

#[test]
fn test_backward_euler_capacitor_history() {
    let method = BackwardEuler::new(1e-6);
    let c = 1e-9; // 1 nF
    let v_prev = 5.0;
    let i_hist = method.capacitor_history_current(c, v_prev);
    // For Backward Euler: I_hist = (C / delta_t) * V_prev = 1e-3 * 5 = 5e-3
    assert!((i_hist - 5e-3).abs() < 1e-10);
}

#[test]
fn test_backward_euler_inductor_history() {
    let method = BackwardEuler::new(1e-6);
    let l = 1e-6; // 1 uH
    let i_prev = 2.0;
    let i_hist = method.inductor_history_current(l, i_prev);
    // For Backward Euler: I_hist = (delta_t / L) * I_prev = 1 * 2 = 2
    assert!((i_hist - 2.0).abs() < 1e-10);
}

#[test]
fn test_backward_euler_implicit() {
    let method = BackwardEuler::new(1e-6);
    assert!(method.is_implicit());
}

#[test]
fn test_backward_euler_order() {
    let method = BackwardEuler::new(1e-6);
    assert_eq!(method.order(), 1);
}

#[test]
fn test_backward_euler_multistep() {
    let method = BackwardEuler::new(1e-6);
    assert!(!method.is_multistep());
}

#[test]
fn test_forward_euler() {
    let method = ForwardEuler::new(1e-6);
    assert_eq!(method.name(), "Forward Euler");
    assert!(!method.is_implicit());
    assert_eq!(method.order(), 1);
    assert!(!method.is_multistep());
}

#[test]
fn test_trapezoidal() {
    let method = Trapezoidal::new(1e-6);
    assert_eq!(method.name(), "Trapezoidal");
    assert!(method.is_implicit());
    assert_eq!(method.order(), 2);
    assert!(!method.is_multistep());

    // For Trapezoidal: G_eq = 2C/delta_t
    let c = 1e-9;
    let g = method.capacitor_equivalent_conductance(c);
    assert!((g - 2e-3).abs() < 1e-10);
}

#[test]
fn test_bdf1() {
    let method = BDF::new(1e-6, 1);
    assert_eq!(method.name(), "BDF1");
    assert_eq!(method.order(), 1);
    assert!(method.is_implicit());
    assert!(method.is_multistep());
}

#[test]
fn test_bdf2() {
    let method = BDF::new(1e-6, 2);
    assert_eq!(method.name(), "BDF2");
    assert_eq!(method.order(), 2);
    assert!(method.is_implicit());
    assert!(method.is_multistep());

    // BDF2 capacitor conductance: G_eq = (4/3)*C/delta_t
    let c = 1e-9;
    let g = method.capacitor_equivalent_conductance(c);
    // (4/3) * 1e-9 / 1e-6 = (4/3) * 1e-3 ≈ 1.333e-3
    assert!((g - 4.0 / 3.0 * 1e-3).abs() < 1e-10);
}

#[test]
fn test_bdf3() {
    let method = BDF::new(1e-6, 3);
    assert_eq!(method.name(), "BDF3");
    assert_eq!(method.order(), 3);
    assert!(method.is_implicit());
    assert!(method.is_multistep());
}

#[test]
fn test_bdf_history() {
    let mut method = BDF::new(1e-6, 3);

    // Push history values
    // Note: push_capacitor_history inserts at position 0 (most recent first)
    // Each push adds to the front, so after multiple pushes:
    // - push(1.0): history = [1.0]
    // - push(0.9): history = [0.9, 1.0]
    // - push(0.8): history = [0.8, 0.9, 1.0]
    method.push_capacitor_history(1.0);
    method.push_capacitor_history(0.9);
    method.push_capacitor_history(0.8);

    let history = method.capacitor_history();
    assert_eq!(history.len(), 3);
    // history[0] = 0.8 (most recent), history[1] = 0.9, history[2] = 1.0 (oldest)
    assert!(
        (history[0] - 0.8).abs() < 1e-10,
        "history[0] = {}",
        history[0]
    );
    assert!(
        (history[1] - 0.9).abs() < 1e-10,
        "history[1] = {}",
        history[1]
    );
    assert!(
        (history[2] - 1.0).abs() < 1e-10,
        "history[2] = {}",
        history[2]
    );
}

#[test]
fn test_bdf_truncation() {
    let mut method = BDF::new(1e-6, 2);

    // Push more history values than order
    method.push_capacitor_history(1.0);
    method.push_capacitor_history(0.9);
    method.push_capacitor_history(0.8);
    method.push_capacitor_history(0.7); // Should be truncated

    let history = method.capacitor_history();
    assert_eq!(history.len(), 2); // Should only keep 2 values
    // After truncation: history[0]=0.7, history[1]=0.8
    assert!(
        (history[0] - 0.7).abs() < 1e-10,
        "history[0] = {}",
        history[0]
    );
    assert!(
        (history[1] - 0.8).abs() < 1e-10,
        "history[1] = {}",
        history[1]
    );
}

#[test]
fn test_bdf_clear_history() {
    let mut method = BDF::new(1e-6, 2);
    method.push_capacitor_history(1.0);
    method.push_capacitor_history(0.9);
    assert_eq!(method.capacitor_history().len(), 2);

    method.clear_history();
    assert!(method.capacitor_history().is_empty());
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

    // BDF3: (18/11)*y(t+dt) - (9/11)*y(t) + (2/11)*y(t-dt) = (6/11)*dt*f
    let coeffs3 = BDF::bdf_coefficients(3);
    assert!((coeffs3[0] - 18.0 / 11.0).abs() < 1e-10);
    assert!((coeffs3[1] - (-9.0 / 11.0)).abs() < 1e-10);
    assert!((coeffs3[2] - 2.0 / 11.0).abs() < 1e-10);
    assert!((coeffs3[3] - (-1.0)).abs() < 1e-10);
}

#[test]
fn test_integration_method_default() {
    let method = crate::sim::integration::default_integration_method();
    assert_eq!(method.name(), "Backward Euler");
}

#[test]
fn test_bdf_history_contribution() {
    let method = BDF::new(1e-6, 2);
    let c = 1e-9;

    let contribution = method.capacitor_history_contribution(c);
    // With empty history, contribution should be 0
    assert!((contribution - 0.0).abs() < 1e-10);
}
