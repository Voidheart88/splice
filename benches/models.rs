use criterion::Criterion;
use splice::models::diode::{DiodeBundle, DiodeOptions};
use splice::models::resistor::ResistorBundle;
use splice::models::{Unit, Variable};
use std::hint::black_box;
use std::sync::Arc;

/// Benchmark for diode model triples computation
pub fn diode_triples_benchmark(c: &mut Criterion) {
    let anode_var = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let cathode_var = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let diode_bundle = DiodeBundle::new(
        Arc::from("DiodeBundle"),
        Some(anode_var),
        Some(cathode_var),
        Some(DiodeOptions::default()),
    );

    let x_vec = vec![0.5, 0.2];

    c.bench_function("DiodeBundle::triples", |b| {
        b.iter(|| {
            black_box(diode_bundle.triples(black_box(&x_vec)));
        })
    });
}

/// Benchmark for diode model pairs computation
pub fn diode_pairs_benchmark(c: &mut Criterion) {
    let anode_var = Variable::new(Arc::from("Node0"), Unit::Volt, 0);
    let cathode_var = Variable::new(Arc::from("Node1"), Unit::Volt, 1);
    let diode_bundle = DiodeBundle::new(
        Arc::from("DiodeBundle"),
        Some(anode_var),
        Some(cathode_var),
        Some(DiodeOptions::default()),
    );

    let x_vec = vec![0.5, 0.2];

    c.bench_function("DiodeBundle::pairs", |b| {
        b.iter(|| {
            black_box(diode_bundle.pairs(black_box(&x_vec)));
        })
    });
}

/// Benchmark for resistor model triples computation
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

/// Combined benchmark group for all basic models
pub fn models_benchmark_group(c: &mut Criterion) {
    // Diode benchmarks
    diode_triples_benchmark(c);
    diode_pairs_benchmark(c);
    
    // Resistor benchmarks
    resistor_triples_benchmark(c);
}