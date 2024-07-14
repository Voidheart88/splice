use super::*;

fn create_variable(name: &str, unit: Unit, idx: usize) -> Variable {
    Variable(Arc::new(name.to_string()), unit, idx)
}

#[test]
fn test_new_resistor_bundle() {
    let resistor_bundle = ResistorBundle::new(
        Arc::new("ResistorBundle1".to_string()),
        Some(create_variable("Node0", Unit::Volt, 0)),
        Some(create_variable("Node1", Unit::Volt, 1)),
        5.0,
    );

    assert_eq!(*resistor_bundle.name(), "ResistorBundle1");
    assert_eq!(resistor_bundle.triples().len(), 4);
    assert_eq!(resistor_bundle.value, Value(5.0));
}

#[test]
fn test_name() {
    let resistor_bundle = ResistorBundle::new(
        Arc::new("ResistorBundle2".to_string()),
        Some(create_variable("Node0", Unit::Volt, 0)),
        Some(create_variable("Node1", Unit::Volt, 1)),
        0.0,
    );

    assert_eq!(*resistor_bundle.name(), "ResistorBundle2");
}

#[test]
fn test_triples() {
    let resistor_bundle = ResistorBundle::new(
        Arc::new("ResistorBundle3".to_string()),
        Some(create_variable("1", Unit::Volt, 0)),
        Some(create_variable("2", Unit::Volt, 1)),
        10.0,
    );

    assert_eq!(resistor_bundle.triples().len(), 4);
    assert_eq!(
        resistor_bundle.triples(),
        Triples::Quad([(0, 0, 0.1), (1, 1, 0.1), (0, 1, -0.1), (1, 0, -0.1),])
    );
}
