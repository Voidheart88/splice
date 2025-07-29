use std::collections::HashMap;
use std::hint::black_box;

use criterion::Criterion;
use num::Zero;
use num::One;
use rand::rng;
use rand::prelude::*;

use splice::solver::{NalgebraSolver, Solver};
use splice::spot::Numeric;

pub fn generate_solvable_system(n: usize, density: Numeric) -> (Vec<Vec<Numeric>>, Vec<Numeric>, Vec<Numeric>) {
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
                black_box(solver.insert_a(&value));
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
                black_box(solver.insert_b(&value));
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
                black_box(solver.insert_a(&value));
            }
        });
    });
}

pub fn nalgebra_solve(c: &mut Criterion) {
    c.bench_function("Nalgebra::solve a 10x10 linalg system", |b| {
        const SIZE: usize = 10;
        let (a_mat, b_vec, _x_vec) = generate_solvable_system(SIZE, 0.5);
        let mut solver = NalgebraSolver::new(SIZE).unwrap();

        for (idx, row) in a_mat.iter().enumerate() {
            for (idy, val) in row.iter().enumerate() {
                solver.insert_a(&(idx, idy, *val));
            }
        }

        for (idx, entry) in b_vec.iter().enumerate() {
            solver.insert_b(&(idx, *entry));
        }

        b.iter(|| {
            black_box(solver.solve().unwrap());
        });
    });
}

