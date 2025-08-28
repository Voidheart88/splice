use crate::solver::tests::*;
use crate::solver::{RSparseSolver, Solver};
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

    let mut solver = RSparseSolver::new(3).unwrap();

    a_matrix.iter().for_each(|trpl| solver.insert_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_b(&(idx, *val)));

    assert_eq!(solver.b_vec_len(), 3);
}

#[test]
fn init_solver_cplx() {
    let a_matrix = [
        (0, 0, ComplexNumeric { re: 1.0, im: 0.0 }),
        (0, 1, ComplexNumeric { re: 2.0, im: 0.0 }),
        (0, 2, ComplexNumeric { re: 3.0, im: 0.0 }),
        (1, 0, ComplexNumeric { re: 4.0, im: 0.0 }),
        (1, 1, ComplexNumeric { re: 5.0, im: 0.0 }),
        (1, 2, ComplexNumeric { re: 6.0, im: 0.0 }),
        (2, 0, ComplexNumeric { re: 7.0, im: 0.0 }),
        (2, 1, ComplexNumeric { re: 8.0, im: 0.0 }),
        (2, 2, ComplexNumeric { re: 9.0, im: 0.0 }),
    ];

    let b_vector = [
        ComplexNumeric { re: 1.0, im: 0.0 },
        ComplexNumeric { re: 2.0, im: 0.0 },
        ComplexNumeric { re: 3.0, im: 0.0 },
    ];

    let mut solver = RSparseSolver::new(3).unwrap();

    a_matrix.iter().for_each(|trpl| solver.insert_cplx_a(trpl));
    b_vector
        .iter()
        .enumerate()
        .for_each(|(idx, val)| solver.insert_cplx_b(&(idx, *val)));

    assert_eq!(solver.cplx_b_vec_len(), 6);
}

#[test]
fn solve_small() {
    let a_matrix = [(0, 0, 5.0), (0, 1, 2.0), (1, 0, 5.0), (1, 1, -2.0)];

    let b_vector = [7.0, 3.0];

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
    let a_matrix_elements = [(0, 0, 1.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, -1.0)];

    let b_vector_elements = [3.0, 1.0];

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
        "Expected x=2.0, put is {}",
        solution[0]
    );
    assert!(
        (solution[1] - 1.0).abs() < epsilon,
        "Expected y=1.0, put is {}",
        solution[1]
    );
}

#[test]
#[should_panic]
fn solve_no_solution() {
    let a_matrix_elements = [(0, 0, 1.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 1.0)];
    let b_vector_elements = [3.0, 5.0];

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
    let a_matrix_elements = [(0, 0, 1.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 2.0)];

    let b_vector_elements = [3.0, 6.0];
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
    let a_matrix_elements = [(0, 0, 1.0), (0, 1, 2.0), (1, 0, 3.0), (1, 1, 4.0)];

    let mut solver = RSparseSolver::new(2).unwrap();

    a_matrix_elements
        .iter()
        .for_each(|trpl| solver.insert_a(trpl));

    solver.insert_a(&(0, 0, 4.0));
    solver.insert_a(&(0, 1, 3.0));
    solver.insert_a(&(1, 0, 2.0));
    solver.insert_a(&(1, 1, 1.0));

    println!("{:?}", solver.a_mat());

    assert_eq!(*solver.a_mat().get(&(0usize, 0usize)).unwrap(), 5.0);
    assert_eq!(*solver.a_mat().get(&(1usize, 0usize)).unwrap(), 5.0);
    assert_eq!(*solver.a_mat().get(&(0usize, 1usize)).unwrap(), 5.0);
    assert_eq!(*solver.a_mat().get(&(1usize, 1usize)).unwrap(), 5.0);
}

#[test]
fn solve_small_electrical() {
    let a_matrix = [(0, 0, 1.0 / 10.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 0.0)];

    let b_vector = [0.0, 10.0];

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

#[test]
fn insert_after_solve() {
    let mut solver = RSparseSolver::new(2).unwrap();

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
    let mut solver = RSparseSolver::new(2).unwrap();

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
    let mut solver = RSparseSolver::new(2).unwrap();

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
fn test_update_from_hashmap_basic() {
    let mut solver = RSparseSolver::new(3).unwrap();
    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(1, 1, 2.0));
    solver.insert_a(&(2, 0, 3.0));
    solver.insert_a(&(0, 2, 4.0));

    solver.update_from_hashmap();

    assert_eq!(solver.sprs().nzmax, 4);
    assert_eq!(solver.sprs().m, 3);
    assert_eq!(solver.sprs().n, 3);
    assert_eq!(solver.sprs().p, vec![0, 2, 3, 4]);
    assert_eq!(solver.sprs().i, vec![0, 2, 1, 0]);
    assert_eq!(solver.sprs().x, vec![1.0, 3.0, 2.0, 4.0]);
}

#[test]
fn test_update_from_hashmap_empty() {
    let mut solver = RSparseSolver::new(0).unwrap();
    solver.update_from_hashmap();

    assert_eq!(solver.sprs().nzmax, 0);
    assert_eq!(solver.sprs().m, 0);
    assert_eq!(solver.sprs().n, 0);
    assert_eq!(solver.sprs().p, vec![0]);
    assert!(solver.sprs().i.is_empty());
    assert!(solver.sprs().x.is_empty());
}

#[test]
fn test_update_from_hashmap_single_element() {
    let mut solver = RSparseSolver::new(7).unwrap();
    solver.insert_a(&(5usize, 7usize, 100.0f64));
    solver.update_from_hashmap();

    assert_eq!(solver.sprs().nzmax, 1);
    assert_eq!(solver.sprs().m, 6);
    assert_eq!(solver.sprs().n, 8);
    assert_eq!(solver.sprs().p, vec![0, 0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(solver.sprs().i, vec![5]);
    assert_eq!(solver.sprs().x, vec![100.0]);
}

#[test]
pub fn rsparse_solve() {
    const SIZE: usize = 10;
    let (a_mat, b_vec, x_vec) = generate_solvable_system(SIZE, 0.5);
    let mut solver = RSparseSolver::new(SIZE).unwrap();

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
        print!("{}", solution[idx]);
        assert!((solution[idx] - x_vec[idx]) < 100.0 * Numeric::EPSILON);
    }
}

#[test]
pub fn rsparse_update_sprs() {
    let mut solver = RSparseSolver::new(3).unwrap();
    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(1, 1, 1.0));
    solver.insert_a(&(2, 2, 1.0));
    solver.update_from_hashmap();

    assert_eq!(solver.sprs().nzmax, 3);
    assert_eq!(solver.sprs().m, 3);
    assert_eq!(solver.sprs().n, 3);
    assert_eq!(solver.sprs().p, Vec::from([0, 1, 2, 3]));
    assert_eq!(solver.sprs().i, Vec::from([0, 1, 2]));
    assert_eq!(solver.sprs().x, Vec::from([1.0, 1.0, 1.0]));
}

#[test]
pub fn rsparse_update_b_vec() {
    let mut solver = RSparseSolver::new(3).unwrap();
    solver.insert_b(&(0, 1.0));
    solver.insert_b(&(1, 2.0));
    solver.insert_b(&(2, 3.0));
    solver.update_from_hashmap();

    assert_eq!(solver.b_vec().len(), 3);
    assert_eq!(solver.b_vec(), &Vec::from([1.0, 2.0, 3.0]));
}

#[test]
fn insert_after_solve_test() {
    let mut solver = RSparseSolver::new(2).unwrap();

    solver.insert_a(&(0, 0, 1.0));
    solver.insert_a(&(0, 1, 1.0));
    solver.insert_a(&(1, 0, 1.0));
    solver.insert_a(&(1, 1, -1.0));
    solver.insert_b(&(0, 3.0));
    solver.insert_b(&(1, 1.0));

    println!("a_mat: {:?}", solver.a_mat());
    println!("b_vec: {:?}", solver.b_vec());
    println!("x_vec: {:?}", solver.x_vec());
    let solution_0 = solver.solve().unwrap().clone();
    println!("a_mat: {:?}", solver.a_mat());
    println!("b_vec: {:?}", solver.b_vec());
    println!("x_vec: {:?}", solver.x_vec());

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
        let mut solver = RSparseSolver::new(SIZE).unwrap();

        for r in 0..SIZE {
            for c in 0..SIZE {
                solver.insert_a(&(r, c, jacobian_mat[&(r, c)]));
            }
        }

        rhs.iter().enumerate().take(SIZE).for_each(|(r, _)| {
            solver.insert_b(&(r, rhs[r]));
        });

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
