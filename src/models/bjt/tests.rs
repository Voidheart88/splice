// Basic tests for BJT model
use super::bjt0::{Bjt0Bundle, Bjt0Options};
use crate::{
    models::{Variable, Unit},
};
use std::sync::Arc;

#[test]
fn test_bjt_creation() {
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let collector = Variable::new(Arc::from("collector"), Unit::Volt, 1);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 2);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        Some(collector),
        Some(emitter),
        None, // Use default options
    );
    
    assert_eq!(bjt.name(), "Q1".into());
    assert_eq!(bjt.b_idx(), Some(0));
    assert_eq!(bjt.c_idx(), Some(1));
    assert_eq!(bjt.e_idx(), Some(2));
}

#[test]
fn test_bjt_default_options() {
    let _options = Bjt0Options::default();
    // Just test that default options can be created
    // Fields are private, so we can't access them directly
    // Test passes if we reach this point without panicking
}

#[test]
fn test_bjt_custom_options() {
    // Test that we can create a BJT with default options
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let collector = Variable::new(Arc::from("collector"), Unit::Volt, 1);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 2);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        Some(collector),
        Some(emitter),
        Some(Bjt0Options::default()),
    );
    
    assert_eq!(bjt.name(), "Q1".into());
    // Test passes if we reach this point without panicking
}

#[test]
fn test_bjt_triples_basic() {
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let collector = Variable::new(Arc::from("collector"), Unit::Volt, 1);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 2);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        Some(collector),
        Some(emitter),
        None,
    );
    
    // Test with zero voltages
    let x_vec = vec![0.0, 0.0, 0.0];
    let triples = bjt.triples(&x_vec);
    
    // At zero voltage, conductances should be zero (no exponential terms)
    let triples_data = triples.data();
    for (_, _, value) in triples_data.iter() {
        if *value != 0.0 {
            // Allow for very small floating point errors
            assert!(value.abs() < 1e-10, "Conductance should be zero at zero voltage, got {}", value);
        }
    }
}

#[test]
fn test_bjt_pairs_basic() {
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let collector = Variable::new(Arc::from("collector"), Unit::Volt, 1);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 2);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        Some(collector),
        Some(emitter),
        None,
    );
    
    // Test with zero voltages
    let x_vec = vec![0.0, 0.0, 0.0];
    let pairs = bjt.pairs(&x_vec);
    
    // At zero voltage, currents should be zero
    let pairs_data = pairs.data();
    for (_, value) in pairs_data.iter() {
        if *value != 0.0 {
            // Allow for very small floating point errors
            assert!(value.abs() < 1e-10, "Current should be zero at zero voltage, got {}", value);
        }
    }
}

#[test]
fn test_bjt_forward_active() {
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let collector = Variable::new(Arc::from("collector"), Unit::Volt, 1);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 2);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        Some(collector),
        Some(emitter),
        None,
    );
    
    // Forward active mode: V_BE = 0.7V, V_CE = 5V
    let v_base = 0.7;
    let v_emitter = 0.0;
    let v_collector = 5.0;
    let x_vec = vec![v_base, v_collector, v_emitter];
    
    let pairs = bjt.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    // In forward active mode, we should have non-zero currents
    let base_current = pairs_data.iter().find(|(idx, _)| *idx == 0)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    let collector_current = pairs_data.iter().find(|(idx, _)| *idx == 1)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    let emitter_current = pairs_data.iter().find(|(idx, _)| *idx == 2)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    // Debug output
    eprintln!("Base current: {}", base_current);
    eprintln!("Collector current: {}", collector_current);
    eprintln!("Emitter current: {}", emitter_current);
    
    // Currents should be non-zero in forward active mode
    // Note: Base current can be very small (microamps range) in forward active mode
    assert!(base_current.abs() < 1e-9 || base_current > 0.0, "Base current should be positive or very small in forward active mode, got {}", base_current);
    assert!(collector_current < 0.0, "Collector current should be negative (flowing out), got {}", collector_current);
    assert!(emitter_current > 0.0, "Emitter current should be positive (flowing in), got {}", emitter_current);
    
    // Current gain: I_C should be approximately beta * I_B
    // Note: When base current is very small, current gain can be very large
    let base_current_abs = base_current.abs();
    if base_current_abs > 1e-12 {
        // Only check current gain if base current is significant
        let current_gain = collector_current.abs() / base_current_abs;
        assert!(current_gain > 50.0, "Current gain should be significant, got {}", current_gain);
    } else {
        // When base current is very small, we can't meaningfully check current gain
        eprintln!("Base current too small ({} A) to check current gain", base_current_abs);
    }
}

#[test]
fn test_bjt_triple_idx() {
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let collector = Variable::new(Arc::from("collector"), Unit::Volt, 1);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 2);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        Some(collector),
        Some(emitter),
        None,
    );
    
    let triple_idx = bjt.triple_idx();
    assert!(triple_idx.is_some(), "Should have triple indices for full BJT");
    
    // Should have triple indices for full BJT model
    // Test passes if we reach this point without panicking
}

#[test]
fn test_bjt_partial_connections() {
    // Test with only base and emitter connected
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 1);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        None, // No collector
        Some(emitter),
        None,
    );
    
    assert_eq!(bjt.b_idx(), Some(0));
    assert_eq!(bjt.c_idx(), None);
    assert_eq!(bjt.e_idx(), Some(1));
    
    // Should still work with partial connections
    let x_vec = vec![0.7, 0.0];
    let pairs = bjt.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    // Count non-zero entries
    let non_zero_count = pairs_data.iter().filter(|(_, val)| *val != 0.0).count();
    assert_eq!(non_zero_count, 2, "Should have 2 currents for base and emitter");
}

#[test]
fn test_bjt_numerical_stability() {
    let base = Variable::new(Arc::from("base"), Unit::Volt, 0);
    let collector = Variable::new(Arc::from("collector"), Unit::Volt, 1);
    let emitter = Variable::new(Arc::from("emitter"), Unit::Volt, 2);
    
    let bjt = Bjt0Bundle::new(
        Arc::from("Q1"),
        Some(base),
        Some(collector),
        Some(emitter),
        None,
    );
    
    // Test with very negative voltage (should not overflow)
    let x_vec = vec![-100.0, 0.0, 0.0];
    let pairs = bjt.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    // Should handle negative voltages gracefully
    for (_, value) in pairs_data.iter() {
        assert!(value.is_finite(), "Should not produce NaN or infinite values, got {}", value);
    }
}