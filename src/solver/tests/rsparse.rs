use crate::{
    solver::{RSparseSolver, Solver},
    spot::ComplexNumeric,
};

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

    let mut solver = RSparseSolver::new(3).unwrap();

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
fn init_solver_cplx() {
    let mut a_matrix = Vec::new();

    a_matrix.push((0, 0, ComplexNumeric { re: 1.0, im: 0.0 }));
    a_matrix.push((0, 1, ComplexNumeric { re: 2.0, im: 0.0 }));
    a_matrix.push((0, 2, ComplexNumeric { re: 3.0, im: 0.0 }));

    a_matrix.push((1, 0, ComplexNumeric { re: 4.0, im: 0.0 }));
    a_matrix.push((1, 1, ComplexNumeric { re: 5.0, im: 0.0 }));
    a_matrix.push((1, 2, ComplexNumeric { re: 6.0, im: 0.0 }));

    a_matrix.push((2, 0, ComplexNumeric { re: 7.0, im: 0.0 }));
    a_matrix.push((2, 1, ComplexNumeric { re: 8.0, im: 0.0 }));
    a_matrix.push((2, 2, ComplexNumeric { re: 9.0, im: 0.0 }));

    let mut b_vector = Vec::new();

    b_vector.push(ComplexNumeric { re: 1.0, im: 0.0 });
    b_vector.push(ComplexNumeric { re: 2.0, im: 0.0 });
    b_vector.push(ComplexNumeric { re: 3.0, im: 0.0 });

    let mut solver = RSparseSolver::new(3).unwrap();

    a_matrix.iter().for_each(|trpl| solver.insert_cplx_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_cplx_b(&(idx, *val)));

    assert_eq!(solver.cplx_cols(), 6);
    assert_eq!(solver.cplx_rows(), 6);
    assert_eq!(solver.cplx_b_vec_len(), 6);
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

    let mut solver = RSparseSolver::new(3).unwrap();

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

    let mut solver = RSparseSolver::new(2).unwrap();

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

    let mut solver = RSparseSolver::new(2).unwrap();

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
    let mut solver = RSparseSolver::new(2).unwrap();

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

    let mut solver = RSparseSolver::new(3).unwrap();

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
fn insert_add_a() {
    let mut a_matrix_elements = Vec::new();
    a_matrix_elements.push((0, 0, 1.0));
    a_matrix_elements.push((0, 1, 2.0));
    a_matrix_elements.push((1, 0, 3.0));
    a_matrix_elements.push((1, 1, 4.0));

    let mut solver = RSparseSolver::new(2).unwrap();

    a_matrix_elements
        .iter()
        .for_each(|trpl| solver.insert_a(trpl));

    solver.insert_a(&(0, 0, 4.0));
    solver.insert_a(&(0, 1, 3.0));
    solver.insert_a(&(1, 0, 2.0));
    solver.insert_a(&(1, 1, 1.0));

    solver.a_mat_mut().sum_dupl();

    let mut exp = rsparse::data::Trpl::new();
    exp.append(0, 0, 5.0);
    exp.append(0, 1, 5.0);
    exp.append(1, 0, 5.0);
    exp.append(1, 1, 5.0);

    solver.a_mat_mut().sum_dupl();
    println!("{:?}", solver.a_mat());

    assert_eq!(exp.get(0, 0), solver.a_mat().get(0, 0));
    assert_eq!(exp.get(1, 0), solver.a_mat().get(1, 0));
    assert_eq!(exp.get(0, 1), solver.a_mat().get(0, 1));
    assert_eq!(exp.get(1, 1), solver.a_mat().get(1, 1));
}

#[test]
fn solve_small_electrical() {
    let mut a_matrix = Vec::new();

    a_matrix.push((0, 0, 1.0 / 10.0));
    a_matrix.push((0, 1, 1.0));
    a_matrix.push((1, 0, 1.0));
    a_matrix.push((1, 1, 0.0));

    let mut b_vector = Vec::new();

    b_vector.push(0.0);
    b_vector.push(10.0);

    let mut solver = RSparseSolver::new(3).unwrap();

    a_matrix.iter().for_each(|trpl| solver.insert_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    let solution = match solver.solve() {
        Ok(solution) => solution,
        Err(err) => panic!("{err}"),
    };

    assert_eq!(solution[0], 10.0);
    assert_eq!(solution[1], -1.0);
}
