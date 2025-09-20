use super::*;

#[test]
fn test_new_capacitor_bundle() {
    let capacitor_bundle = CapacitorBundle::new(
        Arc::from("CapacitorBundle1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        5.0,
    );

    assert_eq!(*capacitor_bundle.name(), *"CapacitorBundle1");
    assert_eq!(capacitor_bundle.triples(None).len(), 1);
    assert_eq!(capacitor_bundle.value, 5.0);
}

#[test]
fn test_name() {
    let capacitor_bundle = CapacitorBundle::new(
        Arc::from("CapacitorBundle2"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        0.0,
    );

    assert_eq!(*capacitor_bundle.name(), *"CapacitorBundle2");
}

#[test]
fn test_triples() {
    let capacitor_bundle = CapacitorBundle::new(
        Arc::from("CapacitorBundle3"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        Some(Variable::new(Arc::from("2"), Unit::Volt, 2)),
        0.0,
    );

    assert_eq!(capacitor_bundle.triples(None).len(), 4);
}

fn make_var(index: usize) -> Option<Variable> {
    Some(Variable::new(Arc::from("test"), Unit::None, index))
}

#[test]
fn test_new() {
    let name: Arc<str> = Arc::from("C1");
    let node0 = make_var(0);
    let node1 = make_var(1);
    let value = 1.0;
    let cap = CapacitorBundle::new(name.clone(), node0.clone(), node1.clone(), value);
    assert_eq!(cap.name(), name);
    assert_eq!(cap.node0_idx(), Some(0));
    assert_eq!(cap.node1_idx(), Some(1));
}

#[test]
fn test_node_indices() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), make_var(1), 1.0);
    assert_eq!(cap.node0_idx(), Some(0));
    assert_eq!(cap.node1_idx(), Some(1));
}

#[test]
fn test_node_indices_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), None, make_var(1), 1.0);
    assert_eq!(cap.node0_idx(), None);
    assert_eq!(cap.node1_idx(), Some(1));

    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), None, 1.0);
    assert_eq!(cap.node0_idx(), Some(0));
    assert_eq!(cap.node1_idx(), None);
}

#[test]
fn test_triples_both_nodes() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), make_var(1), 1.0);
    let triples = cap.triples(None);
    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0], (0, 0, Numeric::zero()));
    assert_eq!(triples[1], (1, 1, Numeric::zero()));
    assert_eq!(triples[2], (0, 1, Numeric::zero()));
    assert_eq!(triples[3], (1, 0, Numeric::zero()));
}

#[test]
fn test_triples_node0_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), None, make_var(1), 1.0);
    let triples = cap.triples(None);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0], (1, 1, Numeric::zero()));
}

#[test]
fn test_triples_node1_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), None, 1.0);
    let triples = cap.triples(None);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0], (0, 0, Numeric::zero()));
}

#[test]
fn test_ac_triples_both_nodes() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), make_var(1), 2.0);
    let freq = 50.0;
    let triples = cap.ac_triples(freq);
    let expected_im = -(2.0 * 2.0 * std::f64::consts::PI * freq);
    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].0, 0);
    assert_eq!(triples[0].1, 0);
    assert_eq!(triples[0].2.re, Numeric::zero());
    assert_eq!(triples[0].2.im, expected_im);
    assert_eq!(triples[1].0, 1);
    assert_eq!(triples[1].1, 1);
    assert_eq!(triples[1].2.re, Numeric::zero());
    assert_eq!(triples[1].2.im, expected_im);
    assert_eq!(triples[2].0, 0);
    assert_eq!(triples[2].1, 1);
    assert_eq!(triples[2].2.re, Numeric::zero());
    assert_eq!(triples[2].2.im, -expected_im);
    assert_eq!(triples[3].0, 1);
    assert_eq!(triples[3].1, 0);
    assert_eq!(triples[3].2.re, Numeric::zero());
    assert_eq!(triples[3].2.im, -expected_im);
}

#[test]
fn test_ac_triples_node0_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), None, make_var(1), 2.0);
    let freq = 50.0;
    let triples = cap.ac_triples(freq);
    let expected_im = -(2.0 * 2.0 * std::f64::consts::PI * freq);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].0, 1);
    assert_eq!(triples[0].1, 1);
    assert_eq!(triples[0].2.re, Numeric::zero());
    assert_eq!(triples[0].2.im, expected_im);
}

#[test]
fn test_ac_triples_node1_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), None, 2.0);
    let freq = 50.0;
    let triples = cap.ac_triples(freq);
    let expected_im = -(2.0 * 2.0 * std::f64::consts::PI * freq);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].0, 0);
    assert_eq!(triples[0].1, 0);
    assert_eq!(triples[0].2.re, Numeric::zero());
    assert_eq!(triples[0].2.im, expected_im);
}

#[test]
fn test_triple_idx_both_nodes() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), make_var(1), 1.0);
    let idx = cap.triple_idx().unwrap();
    assert_eq!(idx.data().len(), 4);
    assert_eq!(idx.data()[0], (0, 0));
    assert_eq!(idx.data()[1], (1, 1));
    assert_eq!(idx.data()[2], (0, 1));
    assert_eq!(idx.data()[3], (1, 0));
}

#[test]
fn test_triple_idx_node0_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), None, make_var(1), 1.0);
    let idx = cap.triple_idx().unwrap();
    assert_eq!(idx.len(), 1);
    assert_eq!(idx.data()[0], (1, 1));
}

#[test]
fn test_triple_idx_node1_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), None, 1.0);
    let idx = cap.triple_idx().unwrap();
    assert_eq!(idx.len(), 1);
    assert_eq!(idx.data()[0], (0, 0));
}

#[test]
fn test_triple_idx_both_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), None, None, 1.0);
    assert!(cap.triple_idx().is_none());
}

#[test]
fn test_transient_triples_both_nodes() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), make_var(1), 2.0);
    let delta_t = 0.01;
    let triples = cap.triples(Some(&delta_t));

    let expected_conductance = 200.0;

    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].0, 0);
    assert_eq!(triples[0].1, 0);
    assert_eq!(triples[0].2, expected_conductance);
    assert_eq!(triples[1].0, 1);
    assert_eq!(triples[1].1, 1);
    assert_eq!(triples[1].2, expected_conductance);
    assert_eq!(triples[2].0, 0);
    assert_eq!(triples[2].1, 1);
    assert_eq!(triples[2].2, -expected_conductance);
    assert_eq!(triples[3].0, 1);
    assert_eq!(triples[3].1, 0);
    assert_eq!(triples[3].2, -expected_conductance);
}

#[test]
fn test_transient_triples_node0_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), None, make_var(1), 2.0);
    let delta_t = 0.01;
    let triples = cap.triples(Some(&delta_t));

    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].0, 1);
    assert_eq!(triples[0].1, 1);
    assert_eq!(triples[0].2, 200.0);
}

#[test]
fn test_transient_triples_node1_none() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), None, 2.0);
    let delta_t = 0.01;
    let triples = cap.triples(Some(&delta_t));

    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].0, 0);
    assert_eq!(triples[0].1, 0);
    assert_eq!(triples[0].2, 200.0);
}

#[test]
fn test_transient_triples_zero_capacitance() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), make_var(1), 0.0);
    let delta_t = 0.01;
    let triples = cap.triples(Some(&delta_t));

    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].2, Numeric::zero());
    assert_eq!(triples[1].2, Numeric::zero());
    assert_eq!(triples[2].2, Numeric::zero());
    assert_eq!(triples[3].2, Numeric::zero());
}

#[test]
fn test_transient_triples_large_delta_t() {
    let cap = CapacitorBundle::new(Arc::from("C1"), make_var(0), make_var(1), 1.0);
    let delta_t = 100.0;
    let triples = cap.triples(Some(&delta_t));

    let expected_conductance = 0.01;

    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].2, expected_conductance);
    assert_eq!(triples[1].2, expected_conductance);
    assert_eq!(triples[2].2, -expected_conductance);
    assert_eq!(triples[3].2, -expected_conductance);
}