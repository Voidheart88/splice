mod hash_map;
use criterion::{criterion_group, criterion_main};
use hash_map::{bench_hashmap_get_mut, bench_hashmap_insert};

criterion_group!(benches, bench_hashmap_insert, bench_hashmap_get_mut);
criterion_main!(benches);
