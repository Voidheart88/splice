use super::*;

#[test]
fn test_new_inductor_bundle() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle =
        InductorBundle::new(Arc::from("InductorBundle1"), Some(node0), Some(node1), 5.0);

    assert_eq!(*inductor_bundle.name(), *"InductorBundle1");
    if let Triples::Quad(triples) = inductor_bundle.triples() {
        assert_eq!(triples.len(), 4);
    } else {
        panic!("Expected Quad tuples");
    }
    assert_eq!(inductor_bundle.value, Value(5.0));
}

#[test]
fn test_name() {
    let node0 = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let inductor_bundle =
        InductorBundle::new(Arc::from("InductorBundle2"), Some(node0), Some(node1), 0.0);

    assert_eq!(*inductor_bundle.name(), *"InductorBundle2");
}

#[test]
fn test_triples() {
    let node1 = Variable::new(Arc::from("1"), Unit::Volt, 1);
    let inductor_bundle = InductorBundle::new(Arc::from("InductorBundle3"), None, Some(node1), 0.0);
    assert_eq!(inductor_bundle.triples().len(), 1);
}
