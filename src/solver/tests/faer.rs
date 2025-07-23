use crate::solver::{FaerSolver, Solver};

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

    let mut solver = FaerSolver::new(3).unwrap();

    a_matrix.iter().for_each(|trpl| solver.set_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.set_b(&(idx, *val)));

    assert_eq!(solver.rows(), 3);
    assert_eq!(solver.cols(), 3);
    assert_eq!(solver.b_vec_len(), 3);
}
