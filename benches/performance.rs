mod diode;
mod faer;
mod faer_sparse;
mod hash_map;
mod nalgebra;
mod resistor;
mod rsparse;

use criterion::{criterion_group, criterion_main};

use crate::diode::*;
use crate::faer::*;
use crate::hash_map::*;
use crate::nalgebra::*;
use crate::resistor::*;
use crate::rsparse::*;





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

criterion_group!(
    backend_solve,
    nalgebra_solve,
    faer_solve,
);

criterion_main!(backend_solve);
