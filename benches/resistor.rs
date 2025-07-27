use criterion::Criterion;
use splice::models::resistor::ResistorBundle;
use splice::models::{Unit, Variable};
use std::hint::black_box;
use std::sync::Arc;

pub fn resistor_triples_benchmark(c: &mut Criterion) {
    let anode_var = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let cathode_var = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let bundle = ResistorBundle::new(
        Arc::from("Resistor"),
        Some(anode_var),
        Some(cathode_var),
        10.0,
    );

    c.bench_function("Resistor::triples", |b| {
        b.iter(|| {
            black_box(bundle.triples());
        })
    });
}
