use super::bjt1::Bjt1Options;
use super::*;
use crate::models::{Unit, Variable};

use std::sync::Arc;

fn var(name: &str, idx: usize) -> Variable {
    Variable::new(Arc::from(name.to_string()), Unit::Volt, idx)
}

#[test]
fn test_bjt0_new_and_properties() {
    let bjt = Bjt0Bundle::new(
        Arc::from("BJT0".to_string()),
        Some(var("Base", 0)),
        Some(var("Collector", 1)),
        Some(var("Emitter", 2)),
        None,
    );
    assert_eq!(*bjt.name(), *"BJT0");
    assert_eq!(bjt.base().unwrap().idx(), 0);
    assert_eq!(bjt.collector().unwrap().idx(), 1);
    assert_eq!(bjt.emitter().unwrap().idx(), 2);
    // default options
    let opts = bjt.options();
    assert_eq!(opts.is, 1e-14);
    assert_eq!(opts.n, 1.0);
    assert_eq!(opts.beta, 100.0);
}

#[test]
fn test_bjt1_delegates_to_bjt0() {
    let opts = Bjt1Options {
        is: 2e-14,
        n: 2.0,
        beta: 150.0,
    };
    let bjt1 = Bjt1Bundle::new(
        Arc::from("BJT1".to_string()),
        Some(var("B", 0)),
        Some(var("C", 1)),
        Some(var("E", 2)),
        Some(opts.clone()),
    );
    // inner should have matching options
    let inner_opts = bjt1.inner().options();
    assert_eq!(inner_opts.is, opts.is);
    assert_eq!(inner_opts.n, opts.n);
    assert_eq!(inner_opts.beta, opts.beta);
    // name forwards
    assert_eq!(*bjt1.name(), *"BJT1");
    // indices forward
    assert_eq!(bjt1.b_idx().unwrap(), 0);
    assert_eq!(bjt1.c_idx().unwrap(), 1);
    assert_eq!(bjt1.e_idx().unwrap(), 2);
}

#[test]
fn test_bjt0_triples_structure() {
    let bjt = Bjt0Bundle::new(
        Arc::from("BJT0".to_string()),
        Some(var("B", 0)),
        Some(var("C", 1)),
        Some(var("E", 2)),
        None,
    );
    let triples = bjt.triples(&[0.0, 0.0, 0.0]);
    // Expect 12 entries: 4 (BE) + 4 (BC) + 2 (transistor action) + 2 (CE path)
    assert_eq!(triples.len(), 12);
}

#[test]
fn test_bjt1_triples_matches_bjt0() {
    let opts = Bjt1Options {
        is: 1e-14,
        n: 1.0,
        beta: 100.0,
    };
    let bjt1 = Bjt1Bundle::new(
        Arc::from("BJT1".to_string()),
        Some(var("B", 0)),
        Some(var("C", 1)),
        Some(var("E", 2)),
        Some(opts),
    );
    let triples1 = bjt1.triples(&[0.0, 0.0, 0.0]);
    let bjt0 = bjt1.inner();
    let triples0 = bjt0.triples(&[0.0, 0.0, 0.0]);
    assert_eq!(triples1, triples0);
}
