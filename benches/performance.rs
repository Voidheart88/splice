mod diode;
mod faer;
mod hash_map;
mod resistor;

use criterion::{criterion_group, criterion_main};

use diode::diode_pairs_benchmark;
use diode::diode_triples_benchmark;
use faer::{faer_insert_a_benchmark, faer_insert_b_benchmark,faer_insert_a_1000_benchmark};
use hash_map::{bench_hashmap_get_mut, bench_hashmap_insert};
use resistor::resistor_triples_benchmark;

criterion_group!(hashmap_benches, bench_hashmap_insert, bench_hashmap_get_mut,);

criterion_group!(
    model_benches,
    diode_triples_benchmark,
    diode_pairs_benchmark,
    resistor_triples_benchmark,
);

criterion_group!(
    backend_benches,
    faer_insert_a_benchmark,
    faer_insert_b_benchmark,
    faer_insert_a_1000_benchmark
);
criterion_main!(backend_benches);
