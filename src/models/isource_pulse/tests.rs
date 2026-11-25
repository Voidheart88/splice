/// Tests for ISourcePulseBundle
use super::*;

#[test]
fn test_isource_pulse_creation() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        0.0,
        1.0,
        1.0,
        1e-9,
        1e-9,
        1e-6,
        1e-5,
    );
    assert_eq!(isource.name(), "I1".into());
}

#[test]
fn test_isource_pulse_pairs() {
    use crate::models::Unit;

    let node0 = Variable::new("1".into(), Unit::Ampere, 0);
    let node1 = Variable::new("2".into(), Unit::Ampere, 1);

    let isource = ISourcePulseBundle::new(
        "I1".into(),
        Some(node0.clone()),
        Some(node1.clone()),
        2.0,  // initial_value
        5.0,  // pulsed_value
        1.0,  // delay_time
        0.1,  // rise_time
        0.1,  // fall_time
        2.0,  // pulse_width
        10.0, // period
    );

    let pairs = isource.pairs();
    assert_eq!(pairs.len(), 2);

    let data = pairs.data();
    assert_eq!(data[0], (0, -2.0)); // node0 gets -initial_value
    assert_eq!(data[1], (1, 2.0)); // node1 gets +initial_value
}

#[test]
fn test_isource_pulse_before_delay() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        1.0,  // initial_value
        5.0,  // pulsed_value
        10.0, // delay_time
        0.1,  // rise_time
        0.1,  // fall_time
        2.0,  // pulse_width
        20.0, // period
    );

    // Before delay time, should be at initial value
    assert_eq!(isource.current_at_time(0.0), 1.0);
    assert_eq!(isource.current_at_time(5.0), 1.0);
    assert_eq!(isource.current_at_time(9.999), 1.0);
}

#[test]
fn test_isource_pulse_during_rise() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        0.0,  // initial_value
        10.0, // pulsed_value
        1.0,  // delay_time
        1.0,  // rise_time
        1.0,  // fall_time
        2.0,  // pulse_width
        20.0, // period
    );

    // During rise (at delay + 0.5 * rise_time)
    assert_eq!(isource.current_at_time(1.5), 5.0);

    // At end of rise
    assert_eq!(isource.current_at_time(2.0), 10.0);
}

#[test]
fn test_isource_pulse_during_pulse() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        0.0,  // initial_value
        10.0, // pulsed_value
        1.0,  // delay_time
        1.0,  // rise_time
        1.0,  // fall_time
        2.0,  // pulse_width
        20.0, // period
    );

    // During pulse (after rise, before fall)
    // Rise ends at t=2, pulse ends at t=4
    assert_eq!(isource.current_at_time(2.0), 10.0);
    assert_eq!(isource.current_at_time(3.0), 10.0);
    assert_eq!(isource.current_at_time(3.999), 10.0);
}

#[test]
fn test_isource_pulse_during_fall() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        0.0,  // initial_value
        10.0, // pulsed_value
        1.0,  // delay_time
        1.0,  // rise_time
        1.0,  // fall_time
        2.0,  // pulse_width
        20.0, // period
    );

    // During fall (at delay + rise + pulse + 0.5 * fall)
    // Fall starts at t=4, ends at t=5
    assert_eq!(isource.current_at_time(4.5), 5.0);

    // At end of fall
    assert_eq!(isource.current_at_time(5.0), 0.0);
}

#[test]
fn test_isource_pulse_after_pulse() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        1.0,  // initial_value
        5.0,  // pulsed_value
        1.0,  // delay_time
        0.1,  // rise_time
        0.1,  // fall_time
        2.0,  // pulse_width
        20.0, // period
    );

    // After pulse, should return to initial value
    assert_eq!(isource.current_at_time(10.0), 1.0);
    assert_eq!(isource.current_at_time(15.0), 1.0);
}

#[test]
fn test_isource_pulse_periodic() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        0.0,  // initial_value
        10.0, // pulsed_value
        1.0,  // delay_time
        1.0,  // rise_time
        1.0,  // fall_time
        2.0,  // pulse_width
        10.0, // period
    );

    // First pulse at t=1-5
    assert_eq!(isource.current_at_time(3.0), 10.0);

    // Between pulses (period is 10, so next pulse starts at t=11)
    assert_eq!(isource.current_at_time(7.0), 0.0);

    // Second pulse (t=11-15)
    assert_eq!(isource.current_at_time(13.0), 10.0);

    // Third pulse (t=21-25)
    assert_eq!(isource.current_at_time(23.0), 10.0);
}

#[test]
fn test_isource_pulse_zero_rise_fall() {
    let isource = ISourcePulseBundle::new(
        "I1".into(),
        None,
        None,
        0.0,  // initial_value
        10.0, // pulsed_value
        1.0,  // delay_time
        0.0,  // rise_time (instant)
        0.0,  // fall_time (instant)
        2.0,  // pulse_width
        20.0, // period
    );

    // Before delay
    assert_eq!(isource.current_at_time(0.5), 0.0);

    // During pulse (instant rise at t=1)
    assert_eq!(isource.current_at_time(1.0), 10.0);
    assert_eq!(isource.current_at_time(2.0), 10.0);

    // After pulse (instant fall at t=3)
    assert_eq!(isource.current_at_time(3.0), 0.0);
}
