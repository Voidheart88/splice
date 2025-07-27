use std::hint::black_box;

use criterion::Criterion;
use splice::{solver::FaerSolver, solver::Solver};

pub fn faer_insert_a_benchmark(c: &mut Criterion) {
    let mut solver = FaerSolver::new(3).unwrap();

    let mut values = Vec::new();
    for val in 0..10 {
        let row = val % 3;
        let col = (10 - val) % 3;
        values.push((row, col, val as f64));
    }

    c.bench_function("Faer::insert a", |b| {
        b.iter(|| {
            for value in values.iter() {
                black_box(solver.insert_a(&value));
            }
        });
    });
}

pub fn faer_insert_b_benchmark(c: &mut Criterion) {
    let mut solver = FaerSolver::new(3).unwrap();

    let mut values = Vec::new();
    for val in 0..10 {
        let row = val % 3;
        values.push((row, val as f64));
    }

    c.bench_function("Faer::insert b", |b| {
        b.iter(|| {
            for value in values.iter() {
                black_box(solver.insert_b(&value));
            }
        });
    });
}

pub fn faer_insert_a_1000_benchmark(c: &mut Criterion) {
    let mut solver = FaerSolver::new(1000).unwrap();

    let mut values = Vec::new();
    for val in 0..1000 {
        let row = val % 1000;
        let col = (1000 - val) % 1000;
        values.push((row, col, val as f64));
    }

    c.bench_function("Faer::insert a 10k", |b| {
        b.iter(|| {
            for value in values.iter() {
                black_box(solver.insert_a(&value));
            }
        });
    });
}
