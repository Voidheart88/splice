use super::*;

#[test]
fn test_new_isource_bundle() {
    let isource_bundle = ISourceBundle::new(
        Rc::new("ISourceBundle1".to_string()),
        Some(Rc::new("Node0".to_string())),
        Some(Rc::new("Node1".to_string())),
        Some(1),
        Some(2),
        5.0,
    );

    assert_eq!(isource_bundle.name(), "ISourceBundle1");
    assert_eq!(isource_bundle.pairs().0.len(), 2); // Es sollten 2 Einträge im pairs Vektor sein
    assert_eq!(isource_bundle.value, Value(5.0));
}

#[test]
fn test_name() {
    let isource_bundle = ISourceBundle::new(
        Rc::new("ISourceBundle2".to_string()),
        Some(Rc::new("Node0".to_string())),
        Some(Rc::new("Node1".to_string())),
        Some(1),
        Some(2),
        0.0,
    );

    assert_eq!(isource_bundle.name(), "ISourceBundle2");
}

#[test]
fn test_pairs_with_both_nodes() {
    let isource_bundle = ISourceBundle::new(
        Rc::new("ISourceBundle3".to_string()),
        Some(Rc::new("Node0".to_string())),
        Some(Rc::new("Node1".to_string())),
        Some(1),
        Some(2),
        10.0,
    );

    let pairs = isource_bundle.pairs();
    assert_eq!(pairs.0.len(), 2);
    assert_eq!(pairs.0[0], (Row(1), Value(-10.0)));
    assert_eq!(pairs.0[1], (Row(2), Value(10.0)));
}

#[test]
fn test_pairs_with_one_node() {
    let isource_bundle = ISourceBundle::new(
        Rc::new("ISourceBundle4".to_string()),
        Some(Rc::new("Node0".to_string())),
        None,
        Some(1),
        None,
        0.0,
    );

    let pairs = isource_bundle.pairs();
    assert_eq!(pairs.0.len(), 1);
    assert_eq!(pairs.0[0], (Row(1), Value(0.0)));
}

#[test]
fn test_pairs_with_no_nodes() {
    let isource_bundle = ISourceBundle::new(
        Rc::new("ISourceBundle5".to_string()),
        None,
        None,
        None,
        None,
        0.0,
    );

    let pairs = isource_bundle.pairs();
    assert_eq!(pairs.0.len(), 0);
}