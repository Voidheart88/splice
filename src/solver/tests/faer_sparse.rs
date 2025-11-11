use crate::solver::{tests::generate_solvable_system, FaerSparseSolver, Solver};
use crate::spot::*;

#[test]
fn init_solver() {
    let a_matrix = [
        (0, 0, 1.0),
        (0, 1, 2.0),
        (0, 2, 3.0),
        (1, 0, 4.0),
        (1, 1, 5.0),
        (1, 2, 6.0),
        (2, 0, 7.0),
        (2, 1, 8.0),
        (2, 2, 9.0),
    ];

    let b_vector = [1.0, 2.0, 3.0];

    let mut solver = FaerSparseSolver::new(3).unwrap();

    a_matrix.iter().for_each(|trpl| solver.insert_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    assert_eq!(solver.rows(), 3);
    assert_eq!(solver.cols(), 3);
    assert_eq!(solver.b_vec_len(), 3);
}

#[test]
fn solve_small() {
    let a_matrix = [(0, 0, 5.0), (0, 1, 2.0), (1, 0, 5.0), (1, 1, -2.0)];

    let b_vector = [7.0, 3.0];

    let mut solver = FaerSparseSolver::new(2).unwrap();

    a_matrix.iter().for_each(|trpl| solver.insert_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    let solution = match solver.solve() {
        Ok(solution) => solution,
        Err(err) => panic!("{err}"),
    };
    assert_eq!(solution[0], 1.0);
    assert_eq!(solution[1], 1.0);
}

#[test]
fn solve_small_2() {
    let a_matrix_elements = [(0, 0, 1.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, -1.0)];

    let b_vector_elements = [3.0, 1.0];

    let mut solver = FaerSparseSolver::new(2).unwrap();

    a_matrix_elements
        .iter()
        .for_each(|trpl| solver.insert_a(trpl));
    b_vector_elements
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    let solution = match solver.solve() {
        Ok(solution) => solution,
        Err(err) => panic!("Error: {err}"),
    };

    let epsilon = 1e-9;

    assert!(
        (solution[0] - 2.0).abs() < epsilon,
        "Expected x=2.0, but is {}",
        solution[0]
    );
    assert!(
        (solution[1] - 1.0).abs() < epsilon,
        "Expected y=1.0, but is {}",
        solution[1]
    );
}

#[test]
#[should_panic]
fn solve_no_solution() {
    let a_matrix_elements = [(0, 0, 1.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 1.0)];

    let b_vector_elements = [3.0, 5.0];

    let mut solver = FaerSparseSolver::new(2).unwrap();

    a_matrix_elements
        .iter()
        .for_each(|trpl| solver.insert_a(trpl));
    b_vector_elements
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    let result = solver.solve();
    if result.is_err() {
        panic!("expected")
    }
}

#[test]
#[should_panic]
fn solve_infinite_solutions_dependent() {
    let a_matrix_elements = [(0, 0, 1.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 2.0)];

    let b_vector_elements = [3.0, 6.0];
    let mut solver = FaerSparseSolver::new(2).unwrap();

    a_matrix_elements
        .iter()
        .for_each(|trpl| solver.insert_a(trpl));
    b_vector_elements
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    let result = solver.solve();
    if result.is_err() {
        panic!("expected")
    }
}

#[test]
fn solve_complex_small_wo_imag() {
    let a_matrix = [
        (0, 0, ComplexNumeric { re: 5.0, im: 0.0 }),
        (0, 1, ComplexNumeric { re: 2.0, im: 0.0 }),
        (1, 0, ComplexNumeric { re: 5.0, im: 0.0 }),
        (1, 1, ComplexNumeric { re: -2.0, im: 0.0 }),
    ];

    let b_vector = [
        ComplexNumeric { re: 7.0, im: 0.0 },
        ComplexNumeric { re: 3.0, im: 0.0 },
    ];

    let mut solver = FaerSparseSolver::new(3).unwrap();

    a_matrix.iter().for_each(|trpl| solver.insert_cplx_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_cplx_b(&(idx, *val)));

    let solution = match solver.solve() {
        Ok(solution) => solution,
        Err(err) => panic!("{err}"),
    };
    assert_eq!(solution[0], 1.0);
    assert_eq!(solution[1], 1.0);
}

#[test]
fn insert_after_solve() {
    let mut solver = FaerSparseSolver::new(2).unwrap();
    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    let solution_0 = solver.solve().unwrap().clone();

    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    let solution_1 = solver.solve().unwrap().clone();

    assert_eq!(solution_0, solution_1)
}

#[test]
fn insert_after_solve2() {
    let mut solver = FaerSparseSolver::new(2).unwrap();

    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    let solution_0 = solver.solve().unwrap().clone();

    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    let solution_1 = solver.solve().unwrap().clone();

    assert!(solution_0 != solution_1)
}

#[test]
fn insert_after_solve3() {
    let mut solver = FaerSparseSolver::new(2).unwrap();

    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    let solution_0 = solver.solve().unwrap().clone();

    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    let _ = solver.solve().unwrap().clone();

    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    let solution_2 = solver.solve().unwrap().clone();
    assert_eq!(solution_0, solution_2)
}

#[test]
pub fn faer_solve() {
    const SIZE: usize = 10;
    let (a_mat, b_vec, x_vec) = generate_solvable_system(SIZE, 0.5);
    let mut solver = FaerSparseSolver::new(SIZE).unwrap();

    for (idx, row) in a_mat.iter().enumerate() {
        for (idy, val) in row.iter().enumerate() {
            solver.insert_a(&(idx, idy, *val));
        }
    }

    for (idx, entry) in b_vec.iter().enumerate() {
        solver.insert_b(&(idx, *entry));
    }

    let solution = solver.solve().unwrap();

    for (idx, _) in solution.iter().enumerate() {
        assert!((solution[idx] - x_vec[idx]) < 100.0 * Numeric::EPSILON);
    }
}
