use super::*;

#[test]
fn test_new_capacitor_bundle() {
    let capacitor_bundle = CapacitorBundle::new(
        Arc::new("CapacitorBundle1".to_string()),
        None,
        Some(Variable::new(Arc::new("1".to_string()), Unit::Volt, 1)),
        5.0,
    );

    assert_eq!(*capacitor_bundle.name(), "CapacitorBundle1");
    assert_eq!(capacitor_bundle.triples().len(), 1);
    assert_eq!(capacitor_bundle.value, Value(5.0));
}

#[test]
fn test_name() {
    let capacitor_bundle = CapacitorBundle::new(
        Arc::new("CapacitorBundle2".to_string()),
        None,
        Some(Variable::new(Arc::new("1".to_string()), Unit::Volt, 1)),
        0.0,
    );

    assert_eq!(*capacitor_bundle.name(), "CapacitorBundle2");
}

#[test]
fn test_triples() {
    let capacitor_bundle = CapacitorBundle::new(
        Arc::new("CapacitorBundle3".to_string()),
        Some(Variable::new(Arc::new("1".to_string()), Unit::Volt, 1)),
        Some(Variable::new(Arc::new("2".to_string()), Unit::Volt, 2)),
        0.0,
    );

    assert_eq!(capacitor_bundle.triples().len(), 4);
}
