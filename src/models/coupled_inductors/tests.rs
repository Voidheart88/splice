use crate::models::coupled_inductors::CoupledInductorsBundle;
use crate::spot::Numeric;
use crate::{Element, Variable};
use num::Zero;
use std::sync::Arc;

fn create_test_inductor(
    name: &str,
    node0: Option<usize>,
    node1: Option<usize>,
    value: Numeric,
) -> Element {
    Element::Inductor(crate::models::InductorBundle {
        name: Arc::from(name),
        node0: node0.map(|idx| {
            Variable::new(
                Arc::from(format!("node{}", idx)),
                crate::models::Unit::Volt,
                idx,
            )
        }),
        node1: node1.map(|idx| {
            Variable::new(
                Arc::from(format!("node{}", idx)),
                crate::models::Unit::Volt,
                idx,
            )
        }),
        value,
        previous_current: Numeric::zero(),
        current_history: Vec::new(),
    })
}

fn create_test_coupled(name: &str, inductor1: &str, inductor2: &str, coupling: Numeric) -> Element {
    Element::CoupledInductors(CoupledInductorsBundle {
        name: Arc::from(name),
        inductor1: Arc::from(inductor1),
        inductor2: Arc::from(inductor2),
        coupling_factor: coupling,
        mutual_inductance: 0.0,
        node0_idx1: None,
        node1_idx1: None,
        node0_idx2: None,
        node1_idx2: None,
        previous_current1: Numeric::zero(),
        previous_current2: Numeric::zero(),
    })
}

#[test]
fn test_new_coupled_inductors() {
    let coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    assert_eq!(coupled.name(), Arc::from("K1"));
    assert_eq!(coupled.inductor1(), Arc::from("L1"));
    assert_eq!(coupled.inductor2(), Arc::from("L2"));
    assert_eq!(coupled.coupling_factor(), 0.999);
    assert_eq!(coupled.mutual_inductance(), 0.0);
}

#[test]
fn test_calculate_mutual_inductance() {
    let mut coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    coupled.calculate_mutual_inductance(10e-6, 15e-6); // L1=10µH, L2=15µH

    let product: crate::spot::Numeric = 10e-6 * 15e-6;
    let expected_m = 0.999 * product.sqrt();
    assert!((coupled.mutual_inductance() - expected_m).abs() < 1e-12);
}

#[test]
fn test_set_node_indices() {
    let mut coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    coupled.set_node_indices(Some(0), Some(1), Some(2), Some(3));

    assert_eq!(coupled.node0_idx1, Some(0));
    assert_eq!(coupled.node1_idx1, Some(1));
    assert_eq!(coupled.node0_idx2, Some(2));
    assert_eq!(coupled.node1_idx2, Some(3));
}

#[test]
fn test_update_previous_currents() {
    let mut coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    coupled.update_previous_currents(0.001, 0.002);

    let (current1, current2) = coupled.previous_currents();
    assert!((current1 - 0.001).abs() < 1e-12);
    assert!((current2 - 0.002).abs() < 1e-12);
}

#[test]
fn test_get_triple_indices_no_indices() {
    let coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    assert!(coupled.get_triple_indices().is_none());
}

#[test]
fn test_get_triple_indices_with_indices() {
    let mut coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    coupled.set_node_indices(Some(0), Some(1), Some(2), Some(3));

    let indices = coupled.get_triple_indices();
    assert!(indices.is_some());
    let idx = indices.unwrap();
    assert_eq!(idx.data().len(), 4); // 4 combinations of nodes (limited by TripleIdx<4>)
}

#[test]
fn test_get_time_variant_triples_no_indices() {
    let coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    let triples = coupled.get_time_variant_triples(&1e-6);
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_get_ac_triples_no_indices() {
    let coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    let triples = coupled.get_ac_triples(1000.0);
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_setup_coupled_inductors_validation() {
    // Test invalid coupling factor
    let mut elements = vec![
        create_test_inductor("L1", Some(0), Some(1), 10e-6),
        create_test_inductor("L2", Some(2), Some(3), 15e-6),
        create_test_coupled("K1", "L1", "L2", 1.2), // Invalid: > 1.0
    ];

    let errors = Element::setup_coupled_inductors(&mut elements);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("Invalid coupling factor"));
}

#[test]
fn test_setup_coupled_inductors_missing_inductors() {
    let mut elements = vec![
        create_test_inductor("L1", Some(0), Some(1), 10e-6),
        create_test_coupled("K1", "L1", "L3", 0.999), // L3 doesn't exist
    ];

    let errors = Element::setup_coupled_inductors(&mut elements);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("not found"));
}

#[test]
fn test_setup_coupled_inductors_self_coupling() {
    let mut elements = vec![
        create_test_inductor("L1", Some(0), Some(1), 10e-6),
        create_test_coupled("K1", "L1", "L1", 0.999), // Self-coupling
    ];

    let errors = Element::setup_coupled_inductors(&mut elements);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("cannot be coupled to itself"));
}

#[test]
fn test_setup_coupled_inductors_successful() {
    let mut elements = vec![
        create_test_inductor("L1", Some(0), Some(1), 10e-6),
        create_test_inductor("L2", Some(2), Some(3), 15e-6),
        create_test_coupled("K1", "L1", "L2", 0.999),
    ];

    let errors = Element::setup_coupled_inductors(&mut elements);
    assert_eq!(errors.len(), 0);

    // Check that node indices were set
    if let Element::CoupledInductors(coupled) = &elements[2] {
        assert_eq!(coupled.node0_idx1, Some(0));
        assert_eq!(coupled.node1_idx1, Some(1));
        assert_eq!(coupled.node0_idx2, Some(2));
        assert_eq!(coupled.node1_idx2, Some(3));
    } else {
        panic!("Expected coupled inductors element");
    }
}

#[test]
fn test_coupled_inductors_mna_contributions() {
    let mut coupled =
        CoupledInductorsBundle::new(Arc::from("K1"), Arc::from("L1"), Arc::from("L2"), 0.999);

    // Set up mutual inductance
    coupled.calculate_mutual_inductance(10e-6, 15e-6);
    coupled.set_node_indices(Some(0), Some(1), Some(2), Some(3));

    // Test transient triples
    let transient_triples = coupled.get_time_variant_triples(&1e-6);
    assert_eq!(transient_triples.len(), 4); // Should have 4 entries (limited by Triples<Numeric, 4>)

    // Test AC triples
    let ac_triples = coupled.get_ac_triples(1000.0);
    assert_eq!(ac_triples.len(), 4); // Should have 4 entries (limited by Triples<ComplexNumeric, 4>)

    // Test triple indices
    let indices = coupled.get_triple_indices();
    assert!(indices.is_some());
    assert_eq!(indices.unwrap().len(), 4); // Should have 4 entries (limited by TripleIdx<4>)
}
