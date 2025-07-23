use criterion::Criterion;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

const MAP_SIZE: usize = 10_000;
const LARGE_MAP_SIZE: usize = 1_000_000;

pub fn bench_hashmap_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap_Insert");

    group.bench_function(format!("insert_{}", MAP_SIZE), |b| {
        let mut map = HashMap::with_capacity(MAP_SIZE);
        let mut keys_to_get: Vec<(u64, u64)> = Vec::with_capacity(MAP_SIZE);

        for _ in 0..MAP_SIZE {
            let key_row = rand::rng().random();
            let key_col = rand::rng().random();
            map.insert((key_row, key_col), key_row);
            keys_to_get.push((key_row, key_col));
        }

        keys_to_get.shuffle(&mut rand::rng());

        b.iter(|| {
            for &key in keys_to_get.iter() {
                let val = map[&key];
                map.insert(key, val + 1);
            }
        });
    });

    group.bench_function(format!("insert_{}", LARGE_MAP_SIZE), |b| {
        let mut map = HashMap::with_capacity(MAP_SIZE);
        let mut keys_to_get: Vec<(u64, u64)> = Vec::with_capacity(MAP_SIZE);

        for _ in 0..MAP_SIZE {
            let key_row = rand::rng().random();
            let key_col = rand::rng().random();
            map.insert((key_row, key_col), key_row);
            keys_to_get.push((key_row, key_col));
        }

        keys_to_get.shuffle(&mut rand::rng());

        b.iter(|| {
            for &key in keys_to_get.iter() {
                let val = map[&key];
                map.insert(key, val + 1);
            }
        });
    });

    group.finish();
}

pub fn bench_hashmap_get_mut(c: &mut Criterion) {
    let mut group = c.benchmark_group("HashMap_GetMut");

    group.bench_function(format!("get_mut_{}", MAP_SIZE), |b| {
        let mut map = HashMap::with_capacity(MAP_SIZE);
        let mut keys_to_get: Vec<(u64, u64)> = Vec::with_capacity(MAP_SIZE);

        for _ in 0..MAP_SIZE {
            let key_row = rand::rng().random();
            let key_col = rand::rng().random();
            map.insert((key_row, key_col), key_row);
            keys_to_get.push((key_row, key_col));
        }

        keys_to_get.shuffle(&mut rand::rng());

        b.iter(|| {
            for &key in keys_to_get.iter() {
                let val = map.get_mut(&key).unwrap();
                *val += 1;
            }
        });
    });

    group.bench_function(format!("get_mut_{}", LARGE_MAP_SIZE), |b| {
        let mut map = HashMap::with_capacity(MAP_SIZE);
        let mut keys_to_get: Vec<(u64, u64)> = Vec::with_capacity(MAP_SIZE);

        for _ in 0..MAP_SIZE {
            let key_row = rand::rng().random();
            let key_col = rand::rng().random();
            map.insert((key_row, key_col), key_row);
            keys_to_get.push((key_row, key_col));
        }

        keys_to_get.shuffle(&mut rand::rng());

        b.iter(|| {
            for &key in keys_to_get.iter() {
                let val = map.get_mut(&key).unwrap();
                *val += 1;
            }
        });
    });

    group.finish();
}
