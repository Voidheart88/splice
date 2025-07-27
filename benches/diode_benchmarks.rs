use criterion::Criterion;
use std::hint::black_box;
use std::sync::Arc;
use splice::models::diode::{DiodeBundle, DiodeOptions};
use splice::models::{Unit, Variable};


pub fn diode_triples_benchmark(c: &mut Criterion) {
    let anode_var = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let cathode_var = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let diode_bundle = DiodeBundle::new(
        Arc::from("DiodeBundle1"),
        Some(anode_var),
        Some(cathode_var),
        Some(DiodeOptions::default()),
    );

    let x_vec = vec![0.5, 0.2]; // Example voltages

    c.bench_function("DiodeBundle::triples", |b| {
        b.iter(|| {
            // Use black_box to prevent the compiler from optimizing away the computation
            black_box(diode_bundle.triples(black_box(&x_vec)));
        })
    });
}

pub fn diode_pairs_benchmark(c: &mut Criterion) {
    let anode_var = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let cathode_var = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let diode_bundle = DiodeBundle::new(
        Arc::from("DiodeBundle1"),
        Some(anode_var),
        Some(cathode_var),
        Some(DiodeOptions::default()),
    );

    let x_vec = vec![0.5, 0.2]; // Example voltages

    c.bench_function("DiodeBundle::pairs", |b| {
        b.iter(|| {
            black_box(diode_bundle.pairs(black_box(&x_vec)));
        })
    });
}