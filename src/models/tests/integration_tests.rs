use crate::models::integration::*;

#[test]
fn test_backward_euler_capacitor() {
    let method = BackwardEuler::new(1e-6);
    let conductance = method.capacitor_equivalent_conductance(1e-6); // 1µF
    assert_eq!(conductance, 1.0); // C/delta_t = 1e-6/1e-6 = 1
}

#[test]
fn test_backward_euler_inductor() {
    let method = BackwardEuler::new(1e-6);
    let conductance = method.inductor_equivalent_conductance(1e-3); // 1mH
    assert_eq!(conductance, 1e-3); // delta_t/L = 1e-6/1e-3 = 1e-3
}

#[test]
fn test_backward_euler_history_current() {
    let method = BackwardEuler::new(1e-6);
    let history_current = method.capacitor_history_current(1e-6, 5.0); // 1µF, 5V
    assert_eq!(history_current, 5.0); // (C/delta_t) * V_prev = 1.0 * 5.0 = 5.0
}

#[test]
fn test_trapezoidal_capacitor() {
    let method = Trapezoidal::new(1e-6);
    let conductance = method.capacitor_equivalent_conductance(1e-6); // 1µF
    assert_eq!(conductance, 2.0); // 2C/delta_t = 2*1e-6/1e-6 = 2
}

#[test]
fn test_trapezoidal_history_current() {
    let method = Trapezoidal::new(1e-6);
    let history_current = method.capacitor_history_current(1e-6, 5.0); // 1µF, 5V
    assert_eq!(history_current, 10.0); // (2C/delta_t) * V_prev = 2.0 * 5.0 = 10.0
}

#[test]
fn test_forward_euler_properties() {
    let method = ForwardEuler::new(1e-6);
    assert_eq!(method.name(), "Forward Euler");
    assert!(!method.is_implicit()); // Forward Euler is explicit
}

#[test]
fn test_method_properties() {
    let backward_euler = BackwardEuler::new(1e-6);
    assert_eq!(backward_euler.name(), "Backward Euler");
    assert!(backward_euler.is_implicit());

    let forward_euler = ForwardEuler::new(1e-6);
    assert_eq!(forward_euler.name(), "Forward Euler");
    assert!(!forward_euler.is_implicit());

    let trapezoidal = Trapezoidal::new(1e-6);
    assert_eq!(trapezoidal.name(), "Trapezoidal");
    assert!(trapezoidal.is_implicit());
}

#[test]
fn test_default_integration_method() {
    let method = crate::models::integration::default_integration_method();
    assert_eq!(method.name(), "Backward Euler");
    assert!(method.is_implicit());
}

#[test]
fn test_inductor_integration() {
    let backward_euler = BackwardEuler::new(1e-6);
    let trapezoidal = Trapezoidal::new(1e-6);
    
    // Test inductor equivalent conductance
    let inductance = 1e-3; // 1mH
    let be_conductance = backward_euler.inductor_equivalent_conductance(inductance);
    let trap_conductance = trapezoidal.inductor_equivalent_conductance(inductance);
    
    assert_eq!(be_conductance, 1e-3); // delta_t/L = 1e-6/1e-3 = 1e-3
    assert_eq!(trap_conductance, 2e-3); // 2*delta_t/L = 2e-6/1e-3 = 2e-3
}

#[test]
fn test_nonlinear_conductance_adjustment() {
    let method = BackwardEuler::new(1e-6);
    let base_conductance = 1.0;
    let adjusted = method.nonlinear_conductance_adjustment(base_conductance);
    assert_eq!(adjusted, base_conductance); // Default implementation returns base conductance
}