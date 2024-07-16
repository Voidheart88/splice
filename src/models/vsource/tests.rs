use super::*;

fn create_variable(name: &str, unit: Unit, idx: usize) -> Variable {
    Variable(Arc::from(name.to_string()), unit, idx)
}

#[test]
fn test_new_vsource_bundle() {
    let vsource_bundle = VSourceBundle::new(
        Arc::from("VSourceBundle1".to_string()),
        create_variable("Branch1", Unit::Ampere, 0),
        Some(create_variable("Node0", Unit::Volt, 1)),
        Some(create_variable("Node1", Unit::Volt, 2)),
        5.0,
    );

    assert_eq!(*vsource_bundle.name(), *"VSourceBundle1");
    assert_eq!(vsource_bundle.triples().len(), 4);
    assert_eq!(vsource_bundle.pairs().len(), 1);
    assert_eq!(vsource_bundle.value(), 5.0);
}

#[test]
fn test_name() {
    let vsource_bundle = VSourceBundle::new(
        Arc::from("VSourceBundle2".to_string()),
        create_variable("Branch2", Unit::Ampere, 0),
        Some(create_variable("Node0", Unit::Volt, 1)),
        Some(create_variable("Node1", Unit::Volt, 2)),
        0.0,
    );

    assert_eq!(*vsource_bundle.name(), *"VSourceBundle2");
}

#[test]
fn test_triples() {
    let vsource_bundle = VSourceBundle::new(
        Arc::from("VSourceBundle3".to_string()),
        create_variable("Branch3", Unit::Ampere, 0),
        Some(create_variable("Node0", Unit::Volt, 1)),
        Some(create_variable("Node1", Unit::Volt, 2)),
        10.0,
    );

    assert_eq!(vsource_bundle.triples().len(), 4);
}

#[test]
fn test_pairs() {
    let vsource_bundle = VSourceBundle::new(
        Arc::from("VSourceBundle4".to_string()),
        create_variable("Branch4", Unit::Ampere, 0),
        Some(create_variable("Node0", Unit::Volt, 1)),
        Some(create_variable("Node1", Unit::Volt, 2)),
        0.0,
    );

    assert_eq!(vsource_bundle.pairs().len(), 1);
}
