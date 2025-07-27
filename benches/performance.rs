mod hash_map;
mod diode_benchmarks;

use criterion::{criterion_group, criterion_main};

use hash_map::{bench_hashmap_get_mut, bench_hashmap_insert};
use diode_benchmarks::diode_triples_benchmark;
use diode_benchmarks::diode_pairs_benchmark;

criterion_group!(hashmap_benches,
    bench_hashmap_insert, 
    bench_hashmap_get_mut,
);

criterion_group!(diode_benches,
    diode_triples_benchmark,
    diode_pairs_benchmark,
);

criterion_main!(diode_benches);
