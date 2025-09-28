use super::*;

#[test]
fn test_new_inductor_bundle() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle1"),
        Some(node0),
        Some(node1),
        5.0,
    );
    assert_eq!(*inductor_bundle.name(), *"InductorBundle1");
    assert_eq!(inductor_bundle.value, 5.0);
}

#[test]
fn test_name() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle2"),
        Some(node0),
        Some(node1),
        0.0,
    );
    assert_eq!(*inductor_bundle.name(), *"InductorBundle2");
}

#[test]
fn test_triples_both_nodes() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle3"),
        Some(node0),
        Some(node1),
        1.0,
    );
    // DC: Induktivität wirkt wie Kurzschluss (0 Ohm)
    let triples = inductor_bundle.triples(None);
    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].2, DEFAULT_CONDUCTANCE);
    assert_eq!(triples[1].2, DEFAULT_CONDUCTANCE);
    assert_eq!(triples[2].2, -DEFAULT_CONDUCTANCE);
    assert_eq!(triples[3].2, -DEFAULT_CONDUCTANCE);
}

#[test]
fn test_triples_node0_none() {
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle4"),
        None,
        Some(node1),
        1.0,
    );
    let triples = inductor_bundle.triples(None);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].2, DEFAULT_CONDUCTANCE);
}

#[test]
fn test_triples_node1_none() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle5"),
        Some(node0),
        None,
        1.0,
    );
    let triples = inductor_bundle.triples(None);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].2, DEFAULT_CONDUCTANCE);
}

#[test]
fn test_ac_triples_both_nodes() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle6"),
        Some(node0),
        Some(node1),
        2.0,
    );
    let freq = 50.0;
    let triples = inductor_bundle.ac_triples(freq);
    let expected_im = 1.0 / (2.0 * 2.0 * std::f64::consts::PI * freq); // 1/(jωL)
    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].2.im, expected_im);
    assert_eq!(triples[1].2.im, expected_im);
    assert_eq!(triples[2].2.im, -expected_im);
    assert_eq!(triples[3].2.im, -expected_im);
}

#[test]
fn test_ac_triples_node0_none() {
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle7"),
        None,
        Some(node1),
        2.0,
    );
    let freq = 50.0;
    let triples = inductor_bundle.ac_triples(freq);
    let expected_im = 1.0 / (2.0 * 2.0 * std::f64::consts::PI * freq);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].2.im, expected_im);
}

#[test]
fn test_ac_triples_node1_none() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle8"),
        Some(node0),
        None,
        2.0,
    );
    let freq = 50.0;
    let triples = inductor_bundle.ac_triples(freq);
    let expected_im = 1.0 / (2.0 * 2.0 * std::f64::consts::PI * freq);
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].2.im, expected_im);
}

#[test]
fn test_triple_idx_both_nodes() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle9"),
        Some(node0),
        Some(node1),
        1.0,
    );
    let idx = inductor_bundle.triple_idx().unwrap();
    assert_eq!(idx.data().len(), 4);
    assert_eq!(idx.data()[0], (0, 0));
    assert_eq!(idx.data()[1], (1, 1));
    assert_eq!(idx.data()[2], (0, 1));
    assert_eq!(idx.data()[3], (1, 0));
}

#[test]
fn test_triple_idx_node0_none() {
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle10"),
        None,
        Some(node1),
        1.0,
    );
    let idx = inductor_bundle.triple_idx().unwrap();
    assert_eq!(idx.len(), 1);
    assert_eq!(idx.data()[0], (1, 1));
}

#[test]
fn test_triple_idx_node1_none() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle11"),
        Some(node0),
        None,
        1.0,
    );
    let idx = inductor_bundle.triple_idx().unwrap();
    assert_eq!(idx.len(), 1);
    assert_eq!(idx.data()[0], (0, 0));
}

#[test]
fn test_triple_idx_both_none() {
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle12"),
        None,
        None,
        1.0,
    );
    assert!(inductor_bundle.triple_idx().is_none());
}

#[test]
fn test_transient_triples_both_nodes() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle13"),
        Some(node0),
        Some(node1),
        2.0,
    );
    let delta_t = 0.01;
    let triples = inductor_bundle.triples(Some(&delta_t));
    let expected_conductance = delta_t / 2.0; // delta_t / L
    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].2, expected_conductance);
    assert_eq!(triples[1].2, expected_conductance);
    assert_eq!(triples[2].2, -expected_conductance);
    assert_eq!(triples[3].2, -expected_conductance);
}

#[test]
fn test_transient_triples_node0_none() {
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle14"),
        None,
        Some(node1),
        2.0,
    );
    let delta_t = 0.01;
    let triples = inductor_bundle.triples(Some(&delta_t));
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].2, delta_t / 2.0);
}

#[test]
fn test_transient_triples_node1_none() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle15"),
        Some(node0),
        None,
        2.0,
    );
    let delta_t = 0.01;
    let triples = inductor_bundle.triples(Some(&delta_t));
    assert_eq!(triples.len(), 1);
    assert_eq!(triples[0].2, delta_t / 2.0);
}

#[test]
fn test_transient_triples_zero_inductance() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle16"),
        Some(node0),
        Some(node1),
        0.0,
    );
    let delta_t = 0.01;
    let triples = inductor_bundle.triples(Some(&delta_t));
    // Bei L=0: Leitwert ist unendlich, aber in der Praxis wird hier oft 0 oder ein Default-Wert verwendet
    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].2, Numeric::INFINITY);
    assert_eq!(triples[1].2, Numeric::INFINITY);
    assert_eq!(triples[2].2, -Numeric::INFINITY);
    assert_eq!(triples[3].2, -Numeric::INFINITY);
}

#[test]
fn test_transient_triples_large_delta_t() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(
        Arc::from("InductorBundle17"),
        Some(node0),
        Some(node1),
        1.0,
    );
    let delta_t = 100.0;
    let triples = inductor_bundle.triples(Some(&delta_t));
    let expected_conductance = delta_t / 1.0; // delta_t / L
    assert_eq!(triples.len(), 4);
    assert_eq!(triples[0].2, expected_conductance);
    assert_eq!(triples[1].2, expected_conductance);
    assert_eq!(triples[2].2, -expected_conductance);
    assert_eq!(triples[3].2, -expected_conductance);
}
