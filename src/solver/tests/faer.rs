use approx::relative_eq;
use std::sync::Arc;

use assert_float_eq::*;

/* -----------------------------------Tests---------------------------------- */
use super::super::*;
use crate::models::{DiodeBundle, Unit, VSourceBundle, Variable};
use crate::solver::FaerSolver;

#[test]
fn test_new() {
    let solver = FaerSolver::new(3).unwrap();

    // Lens should be 0 since no value were loaded
    assert_eq!(solver.rows(), 3);
    assert_eq!(solver.cols(), 3);
    assert_eq!(solver.b_vec_len(), 3);
}

#[test]
fn test_set_a() {
    let mut solver = FaerSolver::new(3).unwrap();

    let triples = Triples::Vec(vec![
        (0, 0, 1.0),
        (1, 1, 2.0),
        (2, 2, 3.0),
    ]);
    solver.set_a(&triples);

    assert_eq!(solver.a_mat()[&(0, 0)], 1.0);
    assert_eq!(solver.a_mat()[&(1, 1)], 2.0);
    assert_eq!(solver.a_mat()[&(2, 2)], 3.0);
}

#[test]
fn test_set_a2() {
    let a_mat = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
        vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0],
        vec![21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0],
        vec![31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0],
        vec![41.0, 42.0, 43.0, 44.0, 45.0, 46.0, 47.0],
        vec![51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0],
        vec![61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0],
    ]
    .into();
    let mut solver = FaerSolver::new(7).unwrap();
    solver.set_a(&a_mat);
    for row in 0..7 {
        for col in 0..7 {
            let val = solver.a_mat()[&(row, col)];
            let exp = row as f64 * 10.0 + col as f64 + 1.0;
            assert_f64_near!(val, exp)
        }
    }
}

#[test]
fn test_set_b1() {
    let mut solver = FaerSolver::new(2).unwrap();
    let pairs = Pairs::Double([(0, 3.0), (1, 4.0)]);
    solver.set_b(&pairs);

    assert_eq!(solver.b_vec()[(0, 0)], 3.0);
    assert_eq!(solver.b_vec()[(1, 0)], 4.0);
}

#[test]
fn test_set_b2() {
    let b_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0].into();
    let mut solver = FaerSolver::new(7).unwrap();
    solver.set_b(&b_vec);
    for row in 0..7 {
        let val = solver.b_vec()[(row, 0)];
        let exp = row as f64 + 1.0;
        assert_f64_near!(val, exp)
    }
}

#[test]
fn test_solve1() {
    // Solvable system
    let mut solver = FaerSolver::new(2).unwrap();
    let triples = Triples::Vec(vec![(0, 0, 1.0), (1, 1, 1.0)]);
    let pairs = Pairs::Double([(0, 3.0), (1, 4.0)]);
    solver.set_a(&triples);
    solver.set_b(&pairs);

    let solution = solver.solve().unwrap();
    assert_eq!(solution, &vec![3.0, 4.0]);
}

#[test]
fn test_solve2() {
    // Singular system
    let mut solver = FaerSolver::new(2).unwrap();
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

    let mut solver = FaerSolver::new(4).unwrap();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);
    //
    let result = solver.solve().unwrap();
    assert_f64_near!(result[0], exp[0], 5);
    assert_f64_near!(result[1], exp[1], 5);
    assert_f64_near!(result[2], exp[2], 5);
    assert_f64_near!(result[3], exp[3], 5);
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

    let mut solver = FaerSolver::new(7).unwrap();
    solver.set_a(&a_mat);
    solver.set_b(&b_vec);
    //
    let result = solver.solve().unwrap();
    assert_f64_near!(result[0], exp[0], 48);
    assert_f64_near!(result[1], exp[1], 48);
    assert_f64_near!(result[2], exp[2], 48);
    assert_f64_near!(result[3], exp[3], 48);
    assert_f64_near!(result[4], exp[4], 48);
    assert_f64_near!(result[5], exp[5], 48);
    assert_f64_near!(result[6], exp[6], 48);
}

#[test]
fn test_newton() {
    // Create an instance of the solver with 2 variables
    let mut solver = FaerSolver::new(2).unwrap();

    let diode = DiodeBundle::new(
        Arc::new("d1".into()),
        Some(Variable::new(Arc::new("1".into()), Unit::Volt, 0)),
        None,
        None,
    );
    let vsource = VSourceBundle::new(
        Arc::new("v1".into()),
        Variable::new(Arc::new("v1#branch".into()), Unit::Ampere, 1),
        None,
        Some(Variable::new(Arc::new("1".into()), Unit::Volt, 0)),
        0.7,
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

    assert_f64_near!(x[0], 0.7);
    assert!(relative_eq!(x[1], -0.01082060404, epsilon = 1e-6));
}

#[test]
fn test_newton2() {
    // Create an instance of the solver with 2 variables
    let mut solver = FaerSolver::new(2).unwrap();

    let diode = DiodeBundle::new(
        Arc::new("d1".into()),
        Some(Variable::new(Arc::new("1".into()), Unit::Volt, 0)),
        None,
        None,
    );
    let vsource = VSourceBundle::new(
        Arc::new("v1".into()),
        Variable::new(Arc::new("v1#branch".into()), Unit::Ampere, 1),
        None,
        Some(Variable::new(Arc::new("1".into()), Unit::Volt, 0)),
        0.8,
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
