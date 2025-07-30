mod diode;
mod faer;
mod faer_sparse;
mod hash_map;
mod nalgebra;
mod resistor;
mod rsparse;

use criterion::{criterion_group, criterion_main};

use diode::diode_pairs_benchmark;
use diode::diode_triples_benchmark;
use faer::{faer_insert_a_1000_benchmark, faer_insert_a_benchmark, faer_insert_b_benchmark};
use hash_map::{bench_hashmap_get_mut, bench_hashmap_insert};
use nalgebra::{
    nalgebra_insert_a_1000_benchmark, nalgebra_insert_a_benchmark, nalgebra_insert_b_benchmark,
};
use resistor::resistor_triples_benchmark;

use crate::faer_sparse::*;
use crate::hash_map::bench_fxhash_get_mut;
use crate::hash_map::bench_fxhash_insert;
use crate::hash_map::bench_nohash_get_mut;
use crate::hash_map::bench_nohash_insert;
use rsparse::{
    rsparse_insert_a_1000_benchmark, rsparse_insert_a_benchmark, rsparse_insert_b_benchmark,
};

criterion_group!(
    hashmap_benches,
    bench_hashmap_insert,
    bench_hashmap_get_mut,
    bench_nohash_insert,
    bench_nohash_get_mut,
    bench_fxhash_insert,
    bench_fxhash_get_mut,
);

criterion_group!(
    model_benches,
    diode_triples_benchmark,
    diode_pairs_benchmark,
    resistor_triples_benchmark,
);

criterion_group!(
    backend_benches,
    faer_insert_a_benchmark,
    faer_sparse_insert_a_benchmark,
    rsparse_insert_a_benchmark,
    nalgebra_insert_a_benchmark,
    faer_insert_b_benchmark,
    faer_sparse_insert_b_benchmark,
    rsparse_insert_b_benchmark,
    nalgebra_insert_b_benchmark,
    faer_insert_a_1000_benchmark,
    faer_sparse_insert_a_1000_benchmark,
    rsparse_insert_a_1000_benchmark,
    nalgebra_insert_a_1000_benchmark,
);
criterion_main!(backend_benches);
