use super::*;

#[test]
fn test_new_diode_bundle() {
    let diode_bundle = DiodeBundle::new(
        Arc::new("DiodeBundle1".to_string()),
        Some(Variable::new(Arc::new("Node0".to_string()), Unit::Volt, 0)),
        Some(Variable::new(Arc::new("Node1".to_string()), Unit::Volt, 1)),
        Some(DiodeOptions::default()),
    );

    assert_eq!(*diode_bundle.name(), "DiodeBundle1");
    assert_eq!(diode_bundle.triples(&vec![0.0, 0.0]).len(), 4);
    assert_eq!(diode_bundle.pairs(&vec![0.0, 0.0]).len(), 2);
    assert_eq!(diode_bundle.value, DiodeOptions::default());
}

#[test]
fn test_name() {
    let diode_bundle = DiodeBundle::new(
        Arc::new("DiodeBundle2".to_string()),
        Some(Variable::new(Arc::new("Node0".to_string()), Unit::Volt, 0)),
        Some(Variable::new(Arc::new("Node1".to_string()), Unit::Volt, 1)),
        Some(DiodeOptions::default()),
    );

    assert_eq!(*diode_bundle.name(), "DiodeBundle2");
}

#[test]
fn test_triples() {
    let diode_bundle = DiodeBundle::new(
        Arc::new("DiodeBundle3".to_string()),
        Some(Variable::new(Arc::new("Node0".to_string()), Unit::Volt, 0)),
        Some(Variable::new(Arc::new("Node1".to_string()), Unit::Volt, 1)),
        Some(DiodeOptions::default()),
    );

    assert_eq!(diode_bundle.triples(&vec![0.0, 0.0]).len(), 4);
}

#[test]
fn test_pairs() {
    let diode_bundle = DiodeBundle::new(
        Arc::new("DiodeBundle4".to_string()),
        Some(Variable::new(Arc::new("Node0".to_string()), Unit::Volt, 0)),
        Some(Variable::new(Arc::new("Node1".to_string()), Unit::Volt, 1)),
        Some(DiodeOptions::default()),
    );

    assert_eq!(diode_bundle.pairs(&vec![0.7, 0.0]).len(), 2);
}

#[test]
fn test_pairs2() {
    let diode_bundle = DiodeBundle::new(
        Arc::new("DiodeBundle4".to_string()),
        Some(Variable::new(Arc::new("Node0".to_string()), Unit::Volt, 0)),
        None,
        Some(DiodeOptions::default()),
    );
    assert_eq!(diode_bundle.pairs(&vec![0.7, 0.0]).len(), 1);
}

#[test]
fn test_pairs3() {
    let diode_bundle = DiodeBundle::new(
        Arc::new("DiodeBundle4".to_string()),
        None,
        Some(Variable::new(Arc::new("Node0".to_string()), Unit::Volt, 0)),
        Some(DiodeOptions::default()),
    );

    assert_eq!(diode_bundle.pairs(&vec![0.7, 0.0]).len(), 1);
}
