mod faer;
mod nalgebra;
mod rsparse;

use std::collections::HashMap;

use num::{One, Zero};
use rand::prelude::*;
use rand::rng;

use crate::spot::*;

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

fn calculate_f(x: &Vec<Numeric>) -> Vec<Numeric> {
    let x0 = x[0];
    let x1 = x[1];

    let f0 = x0.powi(2) + x1.powi(2) - 4.0;
    let f1 = x0 - x1.powi(2) - 1.0;

    vec![f0, f1]
}

fn calculate_jacobian(x: &Vec<Numeric>) -> HashMap<(usize, usize), Numeric> {
    let x0 = x[0];
    let x1 = x[1];

    let mut jacobian = HashMap::new();
    jacobian.insert((0, 0), 2.0 * x0);
    jacobian.insert((0, 1), 2.0 * x1);
    jacobian.insert((1, 0), 1.0);
    jacobian.insert((1, 1), -2.0 * x1);

    jacobian
}

fn norm(vec: &Vec<Numeric>) -> Numeric {
    Numeric::sqrt(vec.iter().map(|val| val * val).sum())
}
