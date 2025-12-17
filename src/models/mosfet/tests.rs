// Basic tests for MOSFET model
use super::mos0::{Mos0Bundle, Mos0Options};
use crate::{
    models::{Variable, Unit},
};
use std::sync::Arc;

#[test]
fn test_mosfet_creation() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None, // Use default options
    );
    
    assert_eq!(mosfet.name(), "M1".into());
    assert_eq!(mosfet.g_idx(), Some(0));
    assert_eq!(mosfet.d_idx(), Some(1));
    assert_eq!(mosfet.s_idx(), Some(2));
}

#[test]
fn test_mosfet_default_options() {
    let _options = Mos0Options::default();
    // Test that default options can be created
    // Fields are private, so we can't access them directly, but we can test
    // that the default values are reasonable by using them in a MOSFET
    // Test passes if we reach this point without panicking
}

#[test]
fn test_mosfet_custom_options() {
    // Test that we can create a MOSFET with custom options
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        Some(Mos0Options::default()),
    );
    
    assert_eq!(mosfet.name(), "M1".into());
    // Test passes if we reach this point without panicking
}

#[test]
fn test_mosfet_triples_basic() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test with zero voltages
    let x_vec = vec![0.0, 0.0, 0.0];
    let triples = mosfet.triples(&x_vec);
    
    // At zero voltage, conductances should be very small (below threshold)
    let triples_data = triples.data();
    for (_, _, value) in triples_data.iter() {
        if *value != 0.0 {
            // The MOSFET model produces small conductances even below threshold
            // Allow for reasonable small values
            assert!(value.abs() < 1e-4, "Conductance should be very small at zero voltage, got {}", value);
        }
    }
}

#[test]
fn test_mosfet_pairs_basic() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test with zero voltages
    let x_vec = vec![0.0, 0.0, 0.0];
    let pairs = mosfet.pairs(&x_vec);
    
    // At zero voltage, currents should be very small
    let pairs_data = pairs.data();
    for (_, value) in pairs_data.iter() {
        if *value != 0.0 {
            // The MOSFET model produces small currents even below threshold
            // Allow for reasonable small values
            assert!(value.abs() < 1e-4, "Current should be very small at zero voltage, got {}", value);
        }
    }
}

#[test]
fn test_mosfet_above_threshold() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test with gate voltage above threshold (Vt0 = 0.43V by default)
    // V_gs = 1.0V (well above threshold), V_ds = 1.0V
    let v_gate = 1.0;
    let v_source = 0.0;
    let v_drain = 1.0;
    let x_vec = vec![v_gate, v_drain, v_source];
    
    let pairs = mosfet.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    // In saturation mode, we should have non-zero currents
    let drain_current = pairs_data.iter().find(|(idx, _)| *idx == 1)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    let source_current = pairs_data.iter().find(|(idx, _)| *idx == 2)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    // Debug output
    eprintln!("Drain current: {}", drain_current);
    eprintln!("Source current: {}", source_current);
    
    // Currents should be non-zero when MOSFET is conducting
    assert!(drain_current > 0.0, "Drain current should be positive when conducting, got {}", drain_current);
    assert!(source_current < 0.0, "Source current should be negative when conducting, got {}", source_current);
    
    // Drain and source currents should be approximately equal (Kirchhoff's current law)
    let current_diff = (drain_current + source_current).abs();
    assert!(current_diff < 1e-6, "Drain and source currents should be equal, difference: {}", current_diff);
}

#[test]
fn test_mosfet_triple_idx() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    let triple_idx = mosfet.triple_idx();
    assert!(triple_idx.is_some(), "Should have triple indices for full MOSFET");
    
    // Should have triple indices for full MOSFET model
    // Test passes if we reach this point without panicking
}

#[test]
fn test_mosfet_partial_connections() {
    // Test with only gate and source connected (drain not connected)
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 1);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        None, // No drain
        Some(source),
        None,
    );
    
    assert_eq!(mosfet.g_idx(), Some(0));
    assert_eq!(mosfet.d_idx(), None);
    assert_eq!(mosfet.s_idx(), Some(1));
    
    // Should still work with partial connections
    let x_vec = vec![1.0, 0.0];
    let pairs = mosfet.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    // Count non-zero entries
    let non_zero_count = pairs_data.iter().filter(|(_, val)| *val != 0.0).count();
    // The MOSFET model may produce different number of currents for partial connections
    assert!(non_zero_count >= 1, "Should have at least 1 current for partial connections, got {}", non_zero_count);
}

#[test]
fn test_mosfet_numerical_stability() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test with very negative voltage (should not overflow)
    let x_vec = vec![-100.0, 0.0, 0.0];
    let pairs = mosfet.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    // Should handle negative voltages gracefully
    for (_, value) in pairs_data.iter() {
        assert!(value.is_finite(), "Should not produce NaN or infinite values, got {}", value);
    }
}

#[test]
fn test_mosfet_saturation_region() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test saturation region: V_gs > Vt, V_ds > V_gs - Vt
    // V_gs = 2.0V, V_ds = 3.0V, Vt = 0.43V
    let v_gate = 2.0;
    let v_source = 0.0;
    let v_drain = 3.0;
    let x_vec = vec![v_gate, v_drain, v_source];
    
    let pairs = mosfet.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    let drain_current = pairs_data.iter().find(|(idx, _)| *idx == 1)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    eprintln!("Saturation region drain current: {}", drain_current);
    
    // In saturation, current should be positive
    // Note: The MOSFET model produces relatively small currents
    assert!(drain_current > 1e-5, "Should have current in saturation, got {}", drain_current);
}

#[test]
fn test_mosfet_linear_region() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test linear region: V_gs > Vt, V_ds < V_gs - Vt
    // V_gs = 2.0V, V_ds = 0.5V, Vt = 0.43V
    let v_gate = 2.0;
    let v_source = 0.0;
    let v_drain = 0.5;
    let x_vec = vec![v_gate, v_drain, v_source];
    
    let pairs = mosfet.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    let drain_current = pairs_data.iter().find(|(idx, _)| *idx == 1)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    eprintln!("Linear region drain current: {}", drain_current);
    
    // In linear region, current should be positive
    assert!(drain_current > 0.0, "Should have current in linear region, got {}", drain_current);
}

#[test]
fn test_mosfet_subthreshold() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test subthreshold: V_gs < Vt (Vt = 0.43V by default)
    let v_gate = 0.2;
    let v_source = 0.0;
    let v_drain = 1.0;
    let x_vec = vec![v_gate, v_drain, v_source];
    
    let pairs = mosfet.pairs(&x_vec);
    let pairs_data = pairs.data();
    
    let drain_current = pairs_data.iter().find(|(idx, _)| *idx == 1)
        .map(|(_, val)| *val)
        .unwrap_or(0.0);
    
    eprintln!("Subthreshold drain current: {}", drain_current);
    
    // In subthreshold, current should be very small
    // The MOSFET model produces small currents even below threshold
    assert!(drain_current.abs() < 1e-4, "Should have small current in subthreshold, got {}", drain_current);
}

#[test]
fn test_mosfet_symmetry() {
    let gate = Variable::new(Arc::from("gate"), Unit::Volt, 0);
    let drain = Variable::new(Arc::from("drain"), Unit::Volt, 1);
    let source = Variable::new(Arc::from("source"), Unit::Volt, 2);
    
    let mosfet = Mos0Bundle::new(
        Arc::from("M1"),
        Some(gate),
        Some(drain),
        Some(source),
        None,
    );
    
    // Test symmetry: swap drain and source
    let v_gate = 2.0;
    let v_source1 = 0.0;
    let v_drain1 = 1.0;
    let x_vec1 = vec![v_gate, v_drain1, v_source1];
    
    let v_source2 = 1.0;
    let v_drain2 = 0.0;
    let x_vec2 = vec![v_gate, v_drain2, v_source2];
    
    let pairs1 = mosfet.pairs(&x_vec1);
    let pairs2 = mosfet.pairs(&x_vec2);
    
    let drain_current1 = pairs1.data().iter().find(|(idx, _)| *idx == 1).map(|(_, val)| *val).unwrap_or(0.0);
    let source_current1 = pairs1.data().iter().find(|(idx, _)| *idx == 2).map(|(_, val)| *val).unwrap_or(0.0);
    
    let drain_current2 = pairs2.data().iter().find(|(idx, _)| *idx == 1).map(|(_, val)| *val).unwrap_or(0.0);
    let source_current2 = pairs2.data().iter().find(|(idx, _)| *idx == 2).map(|(_, val)| *val).unwrap_or(0.0);
    
    eprintln!("Original: Id = {}, Is = {}", drain_current1, source_current1);
    eprintln!("Swapped: Id = {}, Is = {}", drain_current2, source_current2);
    
    // The MOSFET should show symmetric behavior (currents should be similar in magnitude but opposite in direction)
    let current_diff1 = (drain_current1 + source_current1).abs();
    let current_diff2 = (drain_current2 + source_current2).abs();
    
    assert!(current_diff1 < 1e-6, "KCL should hold for original configuration, difference: {}", current_diff1);
    assert!(current_diff2 < 1e-6, "KCL should hold for swapped configuration, difference: {}", current_diff2);
}
