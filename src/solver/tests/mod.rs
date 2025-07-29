mod faer;
mod nalgebra;
mod rsparse;

use std::collections::HashMap;

use rand::prelude::*;
use rand::rng;
use num::{Zero,One};

use crate::spot::*;


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
