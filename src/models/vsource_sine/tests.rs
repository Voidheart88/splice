use super::*;

fn create_variable(name: &str, unit: Unit, idx: usize) -> Variable {
    Variable(Arc::from(name.to_string()), unit, idx)
}

#[test]
fn test_new_vsource_sin_bundle() {
    let vsource_sin_bundle = VSourceSinBundle::new(
        Arc::from("VSourceSinBundle1".to_string()),
        create_variable("Branch1", Unit::Ampere, 0),
        Some(create_variable("Node0", Unit::Volt, 1)),
        Some(create_variable("Node1", Unit::Volt, 2)),
        0.0,
        1.0,
        1.0,
        0.0,
        None,
    );
    assert_eq!(*vsource_sin_bundle.name(), *"VSourceSinBundle1");
    assert_eq!(vsource_sin_bundle.triples().len(), 4);
    assert_eq!(vsource_sin_bundle.pairs(Some(&0.0)).len(), 1);
    assert_eq!(vsource_sin_bundle.amplitude, 1.0);
    assert_eq!(vsource_sin_bundle.frequency, 1.0);
    assert_eq!(vsource_sin_bundle.phase, 0.0);
}

#[test]
fn test_name() {
    let vsource_sin_bundle = VSourceSinBundle::new(
        Arc::from("VSourceSinBundle2".to_string()),
        create_variable("Branch2", Unit::Ampere, 0),
        Some(create_variable("Node0", Unit::Volt, 1)),
        Some(create_variable("Node1", Unit::Volt, 2)),
        0.0,
        1.0,
        1.0,
        0.0,
        None,
    );
    assert_eq!(*vsource_sin_bundle.name(), *"VSourceSinBundle2");
}

#[test]
fn test_triples() {
    let vsource_sin_bundle = VSourceSinBundle::new(
        Arc::from("VSourceSinBundle3".to_string()),
        create_variable("Branch3", Unit::Ampere, 0),
        Some(create_variable("Node0", Unit::Volt, 1)),
        Some(create_variable("Node1", Unit::Volt, 2)),
        0.0,
        1.0,
        1.0,
        0.0,
        None,
    );
    assert_eq!(vsource_sin_bundle.triples().len(), 4);
}

#[test]
fn vsource_sin_pairs_returns_correct_values() {
    let name = Arc::from("V1");
    let branch = Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0);
    let node0 = Some(Variable::new(Arc::from("n1"), Unit::Volt, 1));

    let vsource = VSourceSinBundle::new(
        name,
        branch.clone(),
        node0.clone(),
        None,
        2.0,
        3.0,
        1.0,
        std::f64::consts::FRAC_PI_2,
        None,
    );

    let pairs_t0 = vsource.pairs(Some(&0.0));

    let &(idx, val) = pairs_t0
        .into_iter()
        .next()
        .expect("pairs() should return one entry at time 0");
    assert_eq!(idx, branch.idx());
    assert!((val - 5.0).abs() < 1e-12);

    let pairs_t025 = vsource.pairs(Some(&0.25));
    let &(idx, val) = pairs_t025
        .into_iter()
        .next()
        .expect("pairs() should return one entry at time 0.25");
    assert_eq!(idx, branch.idx());
    assert!((val - 2.0).abs() < 1e-12);
}
