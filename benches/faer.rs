use std::collections::HashMap;
use std::hint::black_box;

use criterion::BatchSize;
use criterion::Criterion;
use num::One;
use num::Zero;
use rand::prelude::*;
use rand::rng;

use splice::solver::{FaerSolver, Solver};
use splice::spot::*;

pub fn generate_solvable_system(
    n: usize,
    density: Numeric,
) -> (Vec<Vec<Numeric>>, Vec<Numeric>, Vec<Numeric>) {
    let mut rng = rng();

    let mut l_entries: HashMap<(usize, usize), f64> = HashMap::new();
    for i in 0..n {
        let diag_val = rng.random_range(0.5..2.0);
        l_entries.insert((i, i), diag_val);

        for j in 0..i {
            if rng.random::<Numeric>() < density {
                let val = rng.random_range(-Numeric::one()..Numeric::one());
                l_entries.insert((i, j), val);
            }
        }
    }

    let mut l = vec![vec![Numeric::zero(); n]; n];
    for (&(i, j), &val) in l_entries.iter() {
        l[i][j] = val;
    }

    let mut a = vec![vec![Numeric::zero(); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                a[i][j] += l[i][k] * l[j][k];
            }
        }
    }

    let x_true: Vec<Numeric> = (0..n).map(|_| rng.random_range(-5.0..5.0)).collect();

    let mut b = vec![Numeric::zero(); n];
    for i in 0..n {
        for j in 0..n {
            b[i] += a[i][j] * x_true[j];
        }
    }

    (a, b, x_true)
}

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

pub fn faer_solve(c: &mut Criterion) {
    let mut group = c.benchmark_group("Faer::solve");

    group.bench_function("Faer::solve random n=10,s=0.5 system", |b| {
        const SIZE: usize = 10;
        const SPARSITY: Numeric = 0.5;

        b.iter_batched(
            || {
                let (a_mat, b_vec, _x_vec) = generate_solvable_system(SIZE, SPARSITY);
                let mut solver = FaerSolver::new(SIZE).unwrap();

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

    group.bench_function("Faer::solve random n=100,s=0.1 system", |b| {
        const SIZE: usize = 100;
        const SPARSITY: Numeric = 0.1;

        b.iter_batched(
            || {
                let (a_mat, b_vec, _x_vec) = generate_solvable_system(SIZE, SPARSITY);
                let mut solver = FaerSolver::new(SIZE).unwrap();

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

    group.measurement_time(std::time::Duration::from_secs(300));
    group.bench_function("Faer::solve random n=1000,s=0.01 system", |b| {
        const SIZE: usize = 1000;
        const SPARSITY: Numeric = 0.01;
        b.iter_batched(
            || {
                let (a_mat, b_vec, _x_vec) = generate_solvable_system(SIZE, SPARSITY);
                let mut solver = FaerSolver::new(SIZE).unwrap();

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
