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
    assert_eq!(capacitor_bundle.triples().len(), 1);
    assert_eq!(capacitor_bundle.value, Value(5.0));
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

    assert_eq!(capacitor_bundle.triples().len(), 4);
}
