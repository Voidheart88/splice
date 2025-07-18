use super::*;

fn create_variable(name: &str, unit: Unit, idx: usize) -> Variable {
    Variable(Arc::from(name.to_string()), unit, idx)
}

#[test]
fn test_new_isource_bundle() {
    let isource_bundle = ISourceBundle::new(
        Arc::from("ISourceBundle1"),
        Some(create_variable("Node0", Unit::Volt, 0)),
        Some(create_variable("Node1", Unit::Volt, 1)),
        5.0,
    );

    assert_eq!(*isource_bundle.name(), *"ISourceBundle1");
    assert_eq!(isource_bundle.pairs().len(), 2);
    assert_eq!(isource_bundle.value, 5.0);
}

#[test]
fn test_name() {
    let isource_bundle = ISourceBundle::new(
        Arc::from("ISourceBundle2"),
        Some(create_variable("Node0", Unit::Volt, 0)),
        Some(create_variable("Node1", Unit::Volt, 1)),
        5.0,
    );

    assert_eq!(*isource_bundle.name(), *"ISourceBundle2");
}

#[test]
fn test_pairs_with_both_nodes() {
    let isource_bundle = ISourceBundle::new(
        Arc::from("ISourceBundle"),
        Some(create_variable("Node0", Unit::Volt, 0)),
        Some(create_variable("Node1", Unit::Volt, 1)),
        5.0,
    );

    let pairs = isource_bundle.pairs();
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0], (0, -5.0));
    assert_eq!(pairs[1], (1, 5.0));
}

#[test]
fn test_pairs_with_one_node() {
    let isource_bundle = ISourceBundle::new(
        Arc::from("ISourceBundle"),
        Some(create_variable("Node1", Unit::Volt, 0)),
        None,
        5.0,
    );

    let pairs = isource_bundle.pairs();
    assert_eq!(pairs.len(), 1);
}

#[test]
fn test_pairs_with_no_nodes() {
    let isource_bundle = ISourceBundle::new(Arc::from("ISourceBundle"), None, None, 5.0);

    let pairs = isource_bundle.pairs();
    assert_eq!(pairs.len(), 0);
}
