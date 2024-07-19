use approx::relative_eq;
use assert_float_eq::*;
use std::sync::Arc;

/* -----------------------------------Tests---------------------------------- */
use super::super::*;
use crate::models::{DiodeBundle, Unit, VSourceBundle, Variable};
use crate::solver::RSparseSolver;

#[test]
fn test_new() {
    let solver = RSparseSolver::new(3).unwrap();

    // Lens should be 0 since no value were loaded
    assert_eq!(solver.rows(), 0);
    assert_eq!(solver.cols(), 0);
    assert_eq!(solver.b_vec_len(), 0);
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
fn test_solve3() {
    let a_mat = vec![
        vec![1.0, -2.0, 3.0, -4.0],
        vec![-9.0, 8.0, -7.0, 6.0],
        vec![0.0, -10.0, 11.0, -12.0],
        vec![-13.0, 14.0, 0.0, 16.0],
    ]
    .into();
    let b_vec = vec![-1.0, 2.0, -3.0, 4.0].into();
    let exp = vec![-2.0 / 45.0, 1.0 / 75.0, -1.0 / 25.0, 91.0 / 450.0];

    let mut solver = RSparseSolver::new(4).unwrap();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);
    //
    let result = solver.solve().unwrap();
    assert_f64_near!(result[0], exp[0], 36);
    assert_f64_near!(result[1], exp[1], 36);
    assert_f64_near!(result[2], exp[2], 36);
    assert_f64_near!(result[3], exp[3], 36);
}

#[test]
fn test_solve4() {
    let a_mat = vec![
        vec![
            0.366104800751686,
            0.783415601458037,
            0.622534742770930,
            -0.223827942462918,
            -0.387028076448005,
            -0.576666626541400,
            -0.639042737864275,
        ],
        vec![
            0.710132755822471,
            0.168797456256260,
            -0.806385180309670,
            -0.908443416566297,
            -0.122309959811038,
            -0.230942776744957,
            0.326177870909269,
        ],
        vec![
            -0.645929006466713,
            0.609400527706867,
            0.0845369336635708,
            0.0283623232128272,
            0.0752243349785038,
            0.0244662154942648,
            -0.552396702390117,
        ],
        vec![
            -0.756301115006787,
            -0.375766468932886,
            0.834780219025871,
            -0.624856979002598,
            -0.494334694705393,
            -0.832714785397389,
            -0.755986269771341,
        ],
        vec![
            0.447287509366027,
            -0.714833616810747,
            0.00229697168505916,
            -0.439469419011979,
            -0.788708359542590,
            0.945895888759815,
            0.461422576483074,
        ],
        vec![
            0.0121950817193410,
            0.0440042641935297,
            0.827392632455661,
            -0.623780216936174,
            0.753331386208702,
            0.445139590589124,
            0.357281194328389,
        ],
        vec![
            0.780816024784319,
            -0.155292786801655,
            0.366401548403756,
            -0.903599625162394,
            -0.191596735719217,
            0.684528909941880,
            0.0892023993241946,
        ],
    ]
    .into();
    let b_vec = vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0].into();
    let exp = vec![
        -6.32989007522271,
        3.79116020770979,
        4.60567941959593,
        -1.82597905538084,
        -1.15613409832909,
        -9.56602308726705,
        17.0448914778907,
    ];

    let mut solver = RSparseSolver::new(exp.len()).unwrap();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);
    //
    let result = solver.solve().unwrap();
    assert_f64_near!(result[0], exp[0], 18);
    assert_f64_near!(result[1], exp[1], 18);
    assert_f64_near!(result[2], exp[2], 18);
    assert_f64_near!(result[3], exp[3], 18);
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

#[test]
fn test_complex_triples_conversion_single() {
    let triple1 = ComplexTriples::Single((0, 0, Complex { re: 1.0, im: 2.0 }));
    let triple2 = ComplexTriples::Single((1, 1, Complex { re: 3.0, im: 4.0 }));

    let solver = RSparseSolver::new(2).unwrap();

    let exp1 = Triples::Quad([(0, 0, 1.0), (0, 2, -2.0), (2, 0, 2.0), (2, 2, 1.0)]);
    let res1 = solver.cplx_triple_to_triple(&triple1);
    assert_eq!(res1, exp1);

    let exp2 = Triples::Quad([(1, 1, 3.0), (1, 3, -4.0), (3, 1, 4.0), (3, 3, 3.0)]);
    let res2 = solver.cplx_triple_to_triple(&triple2);
    assert_eq!(res2, exp2);
}

#[test]
fn test_complex_triples_conversion_double() {
    let triples = ComplexTriples::Double([
        (0, 0, Complex { re: 1.0, im: 2.0 }),
        (1, 1, Complex { re: 3.0, im: 4.0 }),
    ]);

    let solver = RSparseSolver::new(2).unwrap();

    let exp = Triples::Vec(vec![
        (0, 0, 1.0),
        (0, 2, -2.0),
        (2, 0, 2.0),
        (2, 2, 1.0),
        (1, 1, 3.0),
        (1, 3, -4.0),
        (3, 1, 4.0),
        (3, 3, 3.0),
    ]);
    let res1 = solver.cplx_triple_to_triple(&triples);
    assert_eq!(res1, exp);
}

#[test]
fn test_complex_triples_conversion_vec() {
    let triples = ComplexTriples::Vec(vec![
        (0, 0, Complex { re: 1.0, im: 2.0 }),
        (1, 1, Complex { re: 3.0, im: 4.0 }),
        (2, 2, Complex { re: 5.0, im: 6.0 }),
    ]);

    let solver = RSparseSolver::new(3).unwrap();

    let exp = Triples::Vec(vec![
        (0, 0, 1.0),
        (0, 3, -2.0),
        (3, 0, 2.0),
        (3, 3, 1.0),
        (1, 1, 3.0),
        (1, 4, -4.0),
        (4, 1, 4.0),
        (4, 4, 3.0),
        (2, 2, 5.0),
        (2, 5, -6.0),
        (5, 2, 6.0),
        (5, 5, 5.0),
    ]);
    let res1 = solver.cplx_triple_to_triple(&triples);
    assert_eq!(res1, exp);
}
