/// Tests for ISourceStepBundle
use super::*;

#[test]
fn test_isource_step_creation() {
    let isource = ISourceStepBundle::new("I1".into(), None, None, 0.0, 1.0, 5.0);
    assert_eq!(isource.name(), "I1".into());
}

#[test]
fn test_isource_step_pairs() {
    use crate::models::Unit;

    let node0 = Variable::new("1".into(), Unit::Ampere, 0);
    let node1 = Variable::new("2".into(), Unit::Ampere, 1);

    let isource = ISourceStepBundle::new(
        "I1".into(),
        Some(node0.clone()),
        Some(node1.clone()),
        2.0, // initial_value
        5.0, // final_value
        1.0, // delay_time
    );

    let pairs = isource.pairs();
    assert_eq!(pairs.len(), 2);

    let data = pairs.data();
    assert_eq!(data[0], (0, -2.0)); // node0 gets -initial_value
    assert_eq!(data[1], (1, 2.0)); // node1 gets +initial_value
}

#[test]
fn test_isource_step_current_at_time() {
    let isource = ISourceStepBundle::new(
        "I1".into(),
        None,
        None,
        1.0,  // initial_value
        5.0,  // final_value
        10.0, // delay_time
    );

    // Before delay time
    assert_eq!(isource.current_at_time(0.0), 1.0);
    assert_eq!(isource.current_at_time(5.0), 1.0);
    assert_eq!(isource.current_at_time(9.999), 1.0);

    // At and after delay time
    assert_eq!(isource.current_at_time(10.0), 5.0);
    assert_eq!(isource.current_at_time(15.0), 5.0);
    assert_eq!(isource.current_at_time(100.0), 5.0);
}
