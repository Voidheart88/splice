use std::{collections::HashMap, hint::black_box};

use criterion::Criterion;
use nalgebra::{DMatrix, DVector};
use rand::{rng, Rng};

use splice::{
    solver::{NalgebraSolver, Solver},
    spot::Numeric,
};

pub fn generate_solvable_system(
    n: usize,
    density: f64,
) -> (DMatrix<f64>, DVector<f64>, DVector<f64>) {
    let mut rng = rng();

    let mut l_entries = HashMap::new();
    for i in 0..n {
        let diag_val = rng.random_range(0.5..2.0);
        l_entries.insert((i, i), diag_val);

        for j in 0..i {
            // Fill lower triangle
            if rng.random::<f64>() < density {
                let val = rng.random_range(-1.0..1.0);
                l_entries.insert((i, j), val);
            }
        }
    }

    // Convert L from HashMap to CsMat for efficient multiplication
    let mut a_mat: DMatrix<Numeric> = DMatrix::zeros(n, n);

    for ((row, col), val) in l_entries.iter() {
        a_mat[(*row, *col)] = *val;
    }

    let a_mat = a_mat.cross(&a_mat.transpose());
    let mut x_true_dense: DVector<f64> = DVector::zeros(n);
    for idx in 0..n {
        x_true_dense[idx] = rng.random_range(-5.0..5.0);
    }

    // Compute b = A * x_true
    // Convert x_true_dense to a sparse vector for multiplication with CsMat
    let b_sparse = a_mat.clone() * &x_true_dense;

    (a_mat, b_sparse, x_true_dense)
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

        for (idx, row) in a_mat.row_iter().enumerate() {
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
