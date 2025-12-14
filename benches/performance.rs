mod diode;
mod faer;
mod faer_sparse;
mod hash_map;
mod nalgebra;
mod resistor;
mod rsparse;
mod real_world;

use criterion::{criterion_group, criterion_main};

use crate::diode::*;
use crate::faer::*;
use crate::faer_sparse::*;
use crate::hash_map::*;
use crate::nalgebra::*;
use crate::resistor::*;
use crate::rsparse::*;
use crate::real_world::*;

criterion_group!(
    real_world_benches,
    bench_resistor_network,
    bench_resistor_ladder,
);


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
    nalgebra_update,
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
    faer_sparse_solve,
    rsparse_solve
);

criterion_main!(real_world_benches);
