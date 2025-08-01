use crate::solver::tests::*;
use crate::solver::{NalgebraSolver, Solver};
use crate::spot::*;

#[test]
fn init_solver() {
    let mut a_matrix = Vec::new();

    a_matrix.push((0, 0, 1.0));
    a_matrix.push((0, 1, 2.0));
    a_matrix.push((0, 2, 3.0));

    a_matrix.push((1, 0, 4.0));
    a_matrix.push((1, 1, 5.0));
    a_matrix.push((1, 2, 6.0));

    a_matrix.push((2, 0, 7.0));
    a_matrix.push((2, 1, 8.0));
    a_matrix.push((2, 2, 9.0));

    let mut b_vector = Vec::new();

    b_vector.push(1.0);
    b_vector.push(2.0);
    b_vector.push(3.0);

    let mut solver = NalgebraSolver::new(3).unwrap();

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
    let mut a_matrix = Vec::new();

    a_matrix.push((0, 0, 5.0));
    a_matrix.push((0, 1, 2.0));
    a_matrix.push((1, 0, 5.0));
    a_matrix.push((1, 1, -2.0));

    let mut b_vector = Vec::new();

    b_vector.push(7.0);
    b_vector.push(3.0);

    let mut solver = NalgebraSolver::new(2).unwrap();

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
    let mut a_matrix_elements = Vec::new();
    a_matrix_elements.push((0, 0, 1.0));
    a_matrix_elements.push((0, 1, 1.0));
    a_matrix_elements.push((1, 0, 1.0));
    a_matrix_elements.push((1, 1, -1.0));

    let mut b_vector_elements = Vec::new();
    b_vector_elements.push(3.0);
    b_vector_elements.push(1.0);

    let mut solver = NalgebraSolver::new(2).unwrap();

    a_matrix_elements
        .iter()
        .for_each(|trpl| solver.insert_a(trpl));
    b_vector_elements
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    let solution = match solver.solve() {
        Ok(solution) => solution,
        Err(err) => panic!("Error: {}", err),
    };

    let epsilon = 1e-9;

    assert!(
        (solution[0] - 2.0).abs() < epsilon,
        "Erwartet x=2.0, aber ist {}",
        solution[0]
    );
    assert!(
        (solution[1] - 1.0).abs() < epsilon,
        "Erwartet y=1.0, aber ist {}",
        solution[1]
    );
}

#[test]
#[should_panic]
fn solve_no_solution() {
    let mut a_matrix_elements = Vec::new();
    a_matrix_elements.push((0, 0, 1.0));
    a_matrix_elements.push((0, 1, 1.0));
    a_matrix_elements.push((1, 0, 1.0));
    a_matrix_elements.push((1, 1, 1.0));

    let mut b_vector_elements = Vec::new();
    b_vector_elements.push(3.0);
    b_vector_elements.push(5.0);

    let mut solver = NalgebraSolver::new(2).unwrap();

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
    let mut a_matrix_elements = Vec::new();
    a_matrix_elements.push((0, 0, 1.0));
    a_matrix_elements.push((0, 1, 1.0));
    a_matrix_elements.push((1, 0, 2.0));
    a_matrix_elements.push((1, 1, 2.0));

    let mut b_vector_elements = Vec::new();
    b_vector_elements.push(3.0);
    b_vector_elements.push(6.0);
    let mut solver = NalgebraSolver::new(2).unwrap();

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
    let mut a_matrix = Vec::new();

    a_matrix.push((0, 0, ComplexNumeric { re: 5.0, im: 0.0 }));
    a_matrix.push((0, 1, ComplexNumeric { re: 2.0, im: 0.0 }));
    a_matrix.push((1, 0, ComplexNumeric { re: 5.0, im: 0.0 }));
    a_matrix.push((1, 1, ComplexNumeric { re: -2.0, im: 0.0 }));

    let mut b_vector = Vec::new();

    b_vector.push(ComplexNumeric { re: 7.0, im: 0.0 });
    b_vector.push(ComplexNumeric { re: 3.0, im: 0.0 });

    let mut solver = NalgebraSolver::new(3).unwrap();

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
fn insert_add_a_mat() {
    let a_matrix = vec![(0, 0, 1.0), (0, 0, 1.0)];

    let mut solver = NalgebraSolver::new(3).unwrap();
    a_matrix.iter().for_each(|trpl| solver.insert_a(trpl));

    let a_mat = solver.a_mat();

    println!("{}", a_mat);
}

#[test]
pub fn nalgebra_solve() {
    const SIZE: usize = 10;
    let (a_mat, b_vec, x_vec) = generate_solvable_system(SIZE, 0.5);
    let mut solver = NalgebraSolver::new(SIZE).unwrap();

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

#[test]
pub fn newton_raphson_test() {
    const SIZE: usize = 2;

    let mut x_current = vec![1.5, 1.5];

    let max_iterations = 100;
    let tolerance = 1.0e-6;

    println!("Startschätzung: {:?}", x_current);

    for iter in 0..max_iterations {
        let f_val = calculate_f(&x_current);
        let jacobian_mat = calculate_jacobian(&x_current);

        if norm(&f_val) < tolerance {
            println!("Converged after {} iterations.", iter);
            break;
        }

        let rhs: Vec<Numeric> = f_val.iter().map(|val| -val).collect();
        let mut solver = NalgebraSolver::new(SIZE).unwrap();

        for r in 0..SIZE {
            for c in 0..SIZE {
                solver.insert_a(&(r, c, jacobian_mat[&(r, c)]));
            }
        }

        for r in 0..SIZE {
            solver.insert_b(&(r, rhs[r]));
        }

        let dx_vec = match solver.solve() {
            Ok(sol) => sol,
            Err(e) => {
                panic!("Solver Error: {e}");
            }
        };

        x_current
            .iter_mut()
            .enumerate()
            .for_each(|(idx, val)| *val += dx_vec[idx]);

        println!(
            "Iteration {}: x = {:?}, |F(x)| = {}",
            iter,
            x_current,
            norm(&f_val)
        );

        if iter == max_iterations - 1 {
            println!("Maximale Iterationen erreicht ohne Konvergenz.");
        }
    }

    let expected_x0 = 1.7912878;
    let expected_x1 = 0.8895436;

    assert!((x_current[0] - expected_x0).abs() < tolerance);
    assert!((x_current[1] - expected_x1).abs() < tolerance);

    println!("Solution: {:?}", x_current);
}
