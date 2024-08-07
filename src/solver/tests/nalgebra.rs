use approx::relative_eq;
use std::sync::Arc;

/* -----------------------------------Tests---------------------------------- */
use super::super::*;
use crate::{
    models::{DiodeBundle, Unit, VSourceBundle, Variable},
    solver::NalgebraSolver,
};

#[test]
fn test_new() {
    let solver = NalgebraSolver::new(3).unwrap();
    assert_eq!(solver.rows(), 3);
    assert_eq!(solver.cols(), 3);
    assert_eq!(solver.b_vec_len(), 3);
}

#[test]
fn test_set_a() {
    let mut solver = NalgebraSolver::new(2).unwrap();
    let triples = Triples::Vec(vec![(0, 0, 1.0), (1, 1, 2.0)]);
    solver.set_a(&triples);

    assert_eq!(solver.a_mat()[(0, 0)], 1.0);
    assert_eq!(solver.a_mat()[(1, 1)], 2.0);
}

#[test]
fn test_set_b() {
    let mut solver = NalgebraSolver::new(2).unwrap();
    let pairs = Pairs::Double([(0, 3.0), (1, 4.0)]);
    solver.set_b(&pairs);

    assert_eq!(solver.b_vec()[0], 3.0);
    assert_eq!(solver.b_vec()[1], 4.0);
}

#[test]
fn test_solve() {
    // Solvable system
    let mut solver = NalgebraSolver::new(2).unwrap();
    let triples = Triples::Vec(vec![(0, 0, 1.0), (1, 1, 2.0)]);
    let pairs = Pairs::Double([(0, 3.0), (1, 4.0)]);
    solver.set_a(&triples);
    solver.set_b(&pairs);

    let solution = solver.solve().unwrap();
    assert_eq!(solution, &vec![3.0, 2.0]);

    // Singular system
    let mut solver = NalgebraSolver::new(2).unwrap();
    let triples = Triples::Vec(vec![(0, 0, 1.0), (0, 1, 1.0)]);
    let pairs = Pairs::Double([(0, 3.0), (1, 4.0)]);
    solver.set_a(&triples);
    solver.set_b(&pairs);

    let result = solver.solve();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), SolverError::MatrixNonInvertible);
}

#[test]
fn test_newton() {
    // Create an instance of the solver with 2 variables
    let mut solver = NalgebraSolver::new(2).unwrap();

    let diode = DiodeBundle::new(
        Arc::from("d1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
        None,
    );
    let vsource = VSourceBundle::new(
        Arc::from("v1"),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 1),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        0.7,
        None,
    );

    let x = vec![0.6, 0.0];
    let a_mat = diode.triples(&x) + vsource.triples();
    let b_vec = diode.pairs(&x) + vsource.pairs();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);

    let x = solver.solve().unwrap();
    let a_mat = diode.triples(&x) + vsource.triples();
    let b_vec = diode.pairs(&x) + vsource.pairs();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);

    let x = solver.solve().unwrap();
    println!("A: {:?}", a_mat);
    println!("b: {:?}", b_vec);
    println!("x: {:?}", x);

    assert_eq!(x[0], 0.7);
    assert!(relative_eq!(x[1], -0.01082060404, epsilon = 1e-6));
}

#[test]
fn test_newton2() {
    // Create an instance of the solver with 2 variables
    let mut solver = NalgebraSolver::new(2).unwrap();

    let diode = DiodeBundle::new(
        Arc::from("d1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
        None,
    );
    let vsource = VSourceBundle::new(
        Arc::from("v1"),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 1),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        0.8,
        None,
    );

    let x = vec![0.5, 0.0];
    let a_mat = diode.triples(&x) + vsource.triples();
    let b_vec = diode.pairs(&x) + vsource.pairs();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);

    let x = solver.solve().unwrap();
    let a_mat = diode.triples(&x) + vsource.triples();
    let b_vec = diode.pairs(&x) + vsource.pairs();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);

    let x = solver.solve().unwrap();
    println!("A: {:?}", a_mat);
    println!("b: {:?}", b_vec);
    println!("x: {:?}", x);

    assert_eq!(x[0], 0.8);
    assert!(relative_eq!(x[1], -0.566820436, epsilon = 1e-8));
}
