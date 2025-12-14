mod faer;
mod faer_sparse;
mod nalgebra;
mod rsparse;

use std::collections::HashMap;

use num::{One, Zero};
use rand::prelude::*;
use rand::rng;

use crate::solver::FaerSolver;
use crate::solver::NalgebraSolver;
use crate::solver::RSparseSolver;
use crate::solver::Solver;
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
    (0..n).for_each(|i| {
        b[i] = (0..n).map(|j| a[i][j] * x_true[j]).sum();
    });

    (a, b, x_true)
}

fn calculate_f(x: &[Numeric]) -> Vec<Numeric> {
    let x0 = x[0];
    let x1 = x[1];

    let f0 = x0.powi(2) + x1.powi(2) - 4.0;
    let f1 = x0 - x1.powi(2) - 1.0;

    vec![f0, f1]
}

fn calculate_jacobian(x: &[Numeric]) -> HashMap<(usize, usize), Numeric> {
    let x0 = x[0];
    let x1 = x[1];

    let mut jacobian = HashMap::new();
    jacobian.insert((0, 0), 2.0 * x0);
    jacobian.insert((0, 1), 2.0 * x1);
    jacobian.insert((1, 0), 1.0);
    jacobian.insert((1, 1), -2.0 * x1);

    jacobian
}

fn norm(vec: &[Numeric]) -> Numeric {
    Numeric::sqrt(vec.iter().map(|val| val * val).sum())
}

fn test_solver_reset<SolverT>() -> Result<(), String>
where
    SolverT: Solver + std::fmt::Debug,
{
    /// Tests that the solver can be reset and reused for multiple solves.
    ///
    /// This test verifies that the solver correctly resets its internal state
    /// between solves, ensuring that old values do not affect new calculations.
    ///
    /// # Steps
    ///
    /// 1. Insert values into the solver and solve the system.
    /// 2. Insert new values into the solver and solve the system again.
    /// 3. Verify that the results are correct for both solves.
    let mut solver = SolverT::new(2).map_err(|e| e.to_string())?;
    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(1, 1, 1.0));
    solver.insert_b(&(0, 1.0));
    solver.insert_b(&(1, 1.0));

    let res1 = solver.solve().map_err(|e| e.to_string())?;
    assert_eq!(res1[0], 1.0);
    assert_eq!(res1[1], 1.0);

    solver.insert_a(&(0, 0, 3.0));
    solver.insert_a(&(1, 1, 3.0));
    solver.insert_b(&(0, 6.0));
    solver.insert_b(&(1, 6.0));

    let res2 = solver.solve().map_err(|e| e.to_string())?;
    assert_eq!(res2[0], 2.0);
    assert_eq!(res2[1], 2.0);

    Ok(())
}

#[test]
fn faer_solver_resets() {
    /// Tests that the FaerSolver can be reset and reused for multiple solves.
    ///
    /// This test verifies that the FaerSolver correctly resets its internal state
    /// between solves, ensuring that old values do not affect new calculations.
    test_solver_reset::<FaerSolver>().unwrap();
}

#[test]
fn nalgebra_solver_resets() {
    /// Tests that the NalgebraSolver can be reset and reused for multiple solves.
    ///
    /// This test verifies that the NalgebraSolver correctly resets its internal state
    /// between solves, ensuring that old values do not affect new calculations.
    test_solver_reset::<NalgebraSolver>().unwrap();
}

#[test]
fn rsparse_solver_resets() {
    /// Tests that the RSparseSolver can be reset and reused for multiple solves.
    ///
    /// This test verifies that the RSparseSolver correctly resets its internal state
    /// between solves, ensuring that old values do not affect new calculations.
    test_solver_reset::<RSparseSolver>().unwrap();
}
