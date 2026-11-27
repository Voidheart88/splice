#![cfg(feature = "bench-internals")]

use std::hint::black_box;

use criterion::BatchSize;
use criterion::Criterion;

use splice::bench_prelude::*;

pub fn nalgebra_insert_a_benchmark(c: &mut Criterion) {
    let mut solver = NalgebraSolver::new(3).unwrap();

    let mut values = Vec::new();
    for val in 0..10 {
        let row = val % 3;
        let col = (10 - val) % 3;
        values.push((row, col, val as f64));
    }

    c.bench_function("Nalgebra::insert a", |b| {
        b.iter(|| {
            for value in values.iter() {
                solver.insert_a(value);
                black_box(());
            }
        });
    });
}

pub fn nalgebra_insert_b_benchmark(c: &mut Criterion) {
    let mut solver = NalgebraSolver::new(3).unwrap();

    let mut values = Vec::new();
    for val in 0..10 {
        let row = val % 3;
        values.push((row, val as f64));
    }

    c.bench_function("Nalgebra::insert b", |b| {
        b.iter(|| {
            for value in values.iter() {
                solver.insert_b(value);
                black_box(());
            }
        });
    });
}

pub fn nalgebra_insert_a_1000_benchmark(c: &mut Criterion) {
    let mut solver = NalgebraSolver::new(1000).unwrap();

    let mut values = Vec::new();
    for val in 0..1000 {
        let row = val % 1000;
        let col = (1000 - val) % 1000;
        values.push((row, col, val as f64));
    }

    c.bench_function("Nalgebra::insert a 10k", |b| {
        b.iter(|| {
            for value in values.iter() {
                solver.insert_a(value);
                black_box(());
            }
        });
    });
}

pub fn nalgebra_solve(c: &mut Criterion) {
    let mut group = c.benchmark_group("Nalgebra::solve");

    group.bench_function("Nalgebra::solve random n=10,s=0.5 system", |b| {
        const SIZE: usize = 10;
        const SPARSITY: Numeric = 0.5;

        b.iter_batched(
            || {
                let (a_mat, b_vec, _x_vec) = generate_solvable_system(SIZE, SPARSITY);
                let mut solver = NalgebraSolver::new(SIZE).unwrap();

                for (idx, row) in a_mat.iter().enumerate() {
                    for (idy, val) in row.iter().enumerate() {
                        solver.insert_a(&(idx, idy, *val));
                    }
                }

                for (idx, entry) in b_vec.iter().enumerate() {
                    solver.insert_b(&(idx, *entry));
                }

                solver
            },
            |mut solver| {
                black_box(solver.solve().unwrap());
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("Nalgebra::solve random n=100,s=0.1 system", |b| {
        const SIZE: usize = 100;
        const SPARSITY: Numeric = 0.1;

        b.iter_batched(
            || {
                let (a_mat, b_vec, _x_vec) = generate_solvable_system(SIZE, SPARSITY);
                let mut solver = NalgebraSolver::new(SIZE).unwrap();

                for (idx, row) in a_mat.iter().enumerate() {
                    for (idy, val) in row.iter().enumerate() {
                        solver.insert_a(&(idx, idy, *val));
                    }
                }

                for (idx, entry) in b_vec.iter().enumerate() {
                    solver.insert_b(&(idx, *entry));
                }

                solver
            },
            |mut solver| {
                black_box(solver.solve().unwrap());
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}
