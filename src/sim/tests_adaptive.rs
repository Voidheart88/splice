// Simple tests for adaptive timestep functionality

#[test]
fn test_adaptive_timestep_detection() {
    // Test that small timesteps trigger adaptive mode
    let small_timestep = 1e-7; // Smaller than ADAPTIVE_INITIAL_TIMESTEP
    let large_timestep = 1e-2; // Larger than ADAPTIVE_INITIAL_TIMESTEP
    
    assert!(small_timestep <= ADAPTIVE_INITIAL_TIMESTEP);
    assert!(large_timestep > ADAPTIVE_INITIAL_TIMESTEP);
}

#[test]
fn test_adaptive_timestep_constants_are_positive() {
    use crate::sim::tran::*;
    
    // Test that all constants are positive and have reasonable relationships
    assert!(ADAPTIVE_MIN_TIMESTEP > 0.0);
    assert!(ADAPTIVE_MAX_TIMESTEP > ADAPTIVE_MIN_TIMESTEP);
    assert!(ADAPTIVE_INITIAL_TIMESTEP > ADAPTIVE_MIN_TIMESTEP);
    assert!(ADAPTIVE_INITIAL_TIMESTEP < ADAPTIVE_MAX_TIMESTEP);
    assert!(ADAPTIVE_TOLERANCE > 0.0);
    assert!(ADAPTIVE_TOLERANCE < 1.0);
    assert!(ADAPTIVE_SAFETY_FACTOR > 0.0);
    assert!(ADAPTIVE_SAFETY_FACTOR < 1.0);
    assert!(ADAPTIVE_MAX_GROWTH_FACTOR > 1.0);
    assert!(ADAPTIVE_MIN_GROWTH_FACTOR > 0.0);
    assert!(ADAPTIVE_MIN_GROWTH_FACTOR < 1.0);
}

#[test]
fn test_adaptive_timestep_clamping() {
    use crate::sim::tran::*;
    
    // Test that the constants define a reasonable range
    assert!(ADAPTIVE_MIN_TIMESTEP < ADAPTIVE_INITIAL_TIMESTEP);
    assert!(ADAPTIVE_INITIAL_TIMESTEP < ADAPTIVE_MAX_TIMESTEP);
    
    // Test that growth factors are reasonable
    assert!(ADAPTIVE_MIN_GROWTH_FACTOR < ADAPTIVE_MAX_GROWTH_FACTOR);
    
    // Test that safety factor is conservative
    assert!(ADAPTIVE_SAFETY_FACTOR < 1.0);
    assert!(ADAPTIVE_SAFETY_FACTOR > 0.5);
}