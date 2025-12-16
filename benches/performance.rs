mod faer;
mod faer_sparse;
mod hash_map;
mod models;
mod nalgebra;
mod network;
mod rsparse;
mod real_world;

use criterion::{criterion_group, criterion_main};

use crate::faer::*;
use crate::faer_sparse::*;
use crate::hash_map::*;
use crate::models::*;
use crate::nalgebra::*;
use crate::network::*;
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
    models_benchmark_group,
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

criterion_group!(
    network_benches_quick,
    bench_msgpack_serialization_quick,
    bench_msgpack_deserialization_quick,
    bench_msgpack_roundtrip,
    bench_payload_scaling_quick,
);

criterion_group!(
    network_benches_long,
    bench_msgpack_serialization_long,
    bench_msgpack_deserialization_long,
    bench_payload_scaling_long,
);

criterion_main!(real_world_benches, network_benches_quick);
