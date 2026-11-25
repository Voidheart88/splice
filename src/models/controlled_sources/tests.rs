// Tests for controlled sources

// CCCS tests
#[test]
fn test_cccs_creation() {
    use crate::models::controlled_sources::CCCSBundle;
    
    let cccs = CCCSBundle::new("F1".into(), None, None, None, None);
    assert_eq!(cccs.name(), "F1".into());
}

// CCVS tests
#[test]
fn test_ccvs_creation() {
    use crate::models::controlled_sources::CCVSBundle;
    
    let ccvs = CCVSBundle::new("H1".into(), None, None, None, None);
    assert_eq!(ccvs.name(), "H1".into());
}

// VCCS tests
#[test]
fn test_vccs_creation() {
    use crate::models::controlled_sources::VCCSBundle;
    
    let vccs = VCCSBundle::new("G1".into(), None, None, None, None, None);
    assert_eq!(vccs.name(), "G1".into());
    // options is private, so we just test the name
}

#[test]
fn test_vccs_with_options() {
    use crate::models::controlled_sources::VCCSBundle;
    use crate::models::controlled_sources::vccs::VCCSOptions;
    
    let options = VCCSOptions {
        transconductance: 0.1,
    };
    let vccs = VCCSBundle::new("G1".into(), None, None, None, None, Some(options));
    assert_eq!(vccs.name(), "G1".into());
    // options is private, so we just test the name
}

#[test]
fn test_vccs_triples() {
    use crate::models::controlled_sources::VCCSBundle;
    use crate::models::controlled_sources::vccs::VCCSOptions;
    use crate::models::Unit;
    use crate::models::Variable;
    
    let v1 = Variable::new("1".into(), Unit::Volt, 0);
    let v2 = Variable::new("2".into(), Unit::Volt, 1);
    let v3 = Variable::new("3".into(), Unit::Volt, 2);
    let v4 = Variable::new("4".into(), Unit::Volt, 3);

    let vccs = VCCSBundle::new(
        "G1".into(),
        Some(v3),
        Some(v4),
        Some(v1),
        Some(v2),
        Some(VCCSOptions {
            transconductance: 0.05,
        }),
    );

    let triples = vccs.triples();
    assert_eq!(triples.len(), 4);

    // Check specific triple values
    let data = triples.data();
    assert_eq!(data[0], (2, 0, 0.05)); // pos-ctrl_pos with transconductance
    assert_eq!(data[1], (2, 1, -0.05)); // pos-ctrl_neg with -transconductance
    assert_eq!(data[2], (3, 0, -0.05)); // neg-ctrl_pos with -transconductance
    assert_eq!(data[3], (3, 1, 0.05)); // neg-ctrl_neg with transconductance
}

// VCVS tests
#[test]
fn test_vcvs_creation() {
    use crate::models::controlled_sources::VCVSBundle;
    
    let vcvs = VCVSBundle::new("E1".into(), None, None, None, None, None);
    assert_eq!(vcvs.name(), "E1".into());
    // options is private, so we just test the name
}

#[test]
fn test_vcvs_with_options() {
    use crate::models::controlled_sources::VCVSBundle;
    use crate::models::controlled_sources::vcvrs::VCVSOptions;
    
    let options = VCVSOptions { gain: 10.0 };
    let vcvs = VCVSBundle::new("E1".into(), None, None, None, None, Some(options));
    assert_eq!(vcvs.name(), "E1".into());
    // options is private, so we just test the name
}

#[test]
fn test_vcvs_triples() {
    use crate::models::controlled_sources::VCVSBundle;
    use crate::models::controlled_sources::vcvrs::VCVSOptions;
    use crate::models::Unit;
    use crate::models::Variable;
    
    let v1 = Variable::new("1".into(), Unit::Volt, 0);
    let v2 = Variable::new("2".into(), Unit::Volt, 1);
    let v3 = Variable::new("3".into(), Unit::Volt, 2);
    let v4 = Variable::new("4".into(), Unit::Volt, 3);

    let vcvs = VCVSBundle::new(
        "E1".into(),
        Some(v3),
        Some(v4),
        Some(v1),
        Some(v2),
        Some(VCVSOptions { gain: 2.0 }),
    );

    let triples = vcvs.triples();
    assert_eq!(triples.len(), 4);

    // Check specific triple values
    let data = triples.data();
    assert_eq!(data[0], (2, 0, 2.0)); // pos-ctrl_pos with gain
    assert_eq!(data[1], (2, 1, -2.0)); // pos-ctrl_neg with -gain
    assert_eq!(data[2], (3, 0, -2.0)); // neg-ctrl_pos with -gain
    assert_eq!(data[3], (3, 1, 2.0)); // neg-ctrl_neg with gain
}

#[test]
fn test_vcvs_triple_indices() {
    use crate::models::controlled_sources::VCVSBundle;
    use crate::models::Unit;
    use crate::models::Variable;
    
    let v1 = Variable::new("1".into(), Unit::Volt, 0);
    let v2 = Variable::new("2".into(), Unit::Volt, 1);
    let v3 = Variable::new("3".into(), Unit::Volt, 2);
    let v4 = Variable::new("4".into(), Unit::Volt, 3);

    let vcvs = VCVSBundle::new(
        "E1".into(),
        Some(v3),
        Some(v4),
        Some(v1),
        Some(v2),
        None,
    );

    let indices = vcvs.triple_idx().unwrap();
    let data = indices.data();
    assert_eq!(data[0], (2, 0));
    assert_eq!(data[1], (2, 1));
    assert_eq!(data[2], (3, 0));
    assert_eq!(data[3], (3, 1));
}
