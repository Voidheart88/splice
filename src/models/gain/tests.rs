use super::*;
use std::sync::Arc;

#[test]
fn test_new_gain_bundle() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle1"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        2.0,
    );

    assert_eq!(*gain_bundle.name(), *"GainBundle1");
    assert_eq!(gain_bundle.triples().len(), 1);
    assert_eq!(gain_bundle.pairs(&[]).len(), 0);
    assert_eq!(gain_bundle.value, 2.0);
}

#[test]
fn test_name() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle2"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        3.0,
    );

    assert_eq!(*gain_bundle.name(), *"GainBundle2");
}

#[test]
fn test_triples() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle3"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        4.0,
    );

    let triples = gain_bundle.triples();
    assert_eq!(triples.len(), 1);

    let (row, col, value) = triples.data()[0];
    assert_eq!(row, 1);
    assert_eq!(col, 0);
    assert_eq!(value, -4.0);
}

#[test]
fn test_triples_missing_input() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle4"),
        None,
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        5.0,
    );

    let triples = gain_bundle.triples();
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_triples_missing_output() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle5"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        None,
        6.0,
    );

    let triples = gain_bundle.triples();
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_pairs() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle6"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        7.0,
    );

    let pairs = gain_bundle.pairs(&[0.7, 0.0]);
    assert_eq!(pairs.len(), 0);
}

#[test]
fn test_input_idx() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle7"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        8.0,
    );

    assert_eq!(gain_bundle.input_idx(), Some(0));
}

#[test]
fn test_output_idx() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle8"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        9.0,
    );

    assert_eq!(gain_bundle.output_idx(), Some(1));
}

#[test]
fn test_missing_input_idx() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle9"),
        None,
        Some(Variable::new(Arc::from("Output"), Unit::Volt, 1)),
        10.0,
    );

    assert_eq!(gain_bundle.input_idx(), None);
}

#[test]
fn test_missing_output_idx() {
    let gain_bundle = GainBundle::new(
        Arc::from("GainBundle10"),
        Some(Variable::new(Arc::from("Input"), Unit::Volt, 0)),
        None,
        11.0,
    );

    assert_eq!(gain_bundle.output_idx(), None);
}
