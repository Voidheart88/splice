/// Tests for ISourceSinBundle
use super::*;

#[test]
fn test_isource_sin_creation() {
    let isource = ISourceSinBundle::new("I1".into(), None, None, 0.0, 1.0, 1000.0, 0.0, None);
    assert_eq!(isource.name(), "I1".into());
}

#[test]
fn test_isource_sin_pairs() {
    use crate::models::Unit;

    let node0 = Variable::new("1".into(), Unit::Ampere, 0);
    let node1 = Variable::new("2".into(), Unit::Ampere, 1);

    let isource = ISourceSinBundle::new(
        "I1".into(),
        Some(node0.clone()),
        Some(node1.clone()),
        5.0, // DC offset
        1.0,
        1000.0,
        0.0,
        None,
    );

    let pairs = isource.pairs();
    assert_eq!(pairs.len(), 2);

    let data = pairs.data();
    assert_eq!(data[0], (0, -5.0)); // node0 gets -dc_offset
    assert_eq!(data[1], (1, 5.0)); // node1 gets +dc_offset
}

#[test]
fn test_isource_sin_ac_pairs() {
    use crate::models::Unit;

    let node0 = Variable::new("1".into(), Unit::Ampere, 0);
    let node1 = Variable::new("2".into(), Unit::Ampere, 1);

    let isource = ISourceSinBundle::new(
        "I1".into(),
        Some(node0.clone()),
        Some(node1.clone()),
        0.0,
        2.0, // amplitude
        1000.0,
        0.0,       // phase
        Some(3.0), // AC value
    );

    let ac_pairs = isource.ac_pairs();
    assert_eq!(ac_pairs.len(), 2);

    let data = ac_pairs.data();
    assert_eq!(data[0], (0, Complex::new(-3.0, 0.0)));
    assert_eq!(data[1], (1, Complex::new(3.0, 0.0)));
}
