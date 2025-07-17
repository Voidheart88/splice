use num::Complex;

use super::*;

// --------------------------------Triples Tests--------------------------------
#[test]
fn test_cplx_into() {
    let single = Triples::Single((1, 1, 1.0));
    let exp = ComplexTriples::Single((1, 1, Complex { re: 1.0, im: 0.0 }));
    assert_eq!(ComplexTriples::from(single), exp);

    let double = Triples::Double([(0, 0, 1.0), (1, 1, 2.0)]);
    let exp = ComplexTriples::Double([
        (0, 0, Complex { re: 1.0, im: 0.0 }),
        (1, 1, Complex { re: 2.0, im: 0.0 }),
    ]);
    assert_eq!(ComplexTriples::from(double), exp);
}

// Helper to create a Complex value
fn c(re: f64, im: f64) -> Complex<f64> {
    Complex::new(re, im)
}

#[test]
fn test_add_empty() {
    assert_eq!(
        ComplexTriples::Empty + ComplexTriples::Empty,
        ComplexTriples::Empty
    );
    assert_eq!(
        ComplexTriples::Empty + ComplexTriples::Single((0, 0, c(1.0, 1.0))),
        ComplexTriples::Single((0, 0, c(1.0, 1.0)))
    );
    assert_eq!(
        ComplexTriples::Single((0, 0, c(1.0, 1.0))) + ComplexTriples::Empty,
        ComplexTriples::Single((0, 0, c(1.0, 1.0)))
    );
}

#[test]
fn test_add_single_same_rc() {
    let s1 = ComplexTriples::Single((0, 0, c(1.0, 2.0)));
    let s2 = ComplexTriples::Single((0, 0, c(3.0, 4.0)));
    assert_eq!(s1 + s2, ComplexTriples::Single((0, 0, c(4.0, 6.0))));
}

#[test]
fn test_add_single_different_rc() {
    let s1 = ComplexTriples::Single((0, 0, c(1.0, 2.0)));
    let s2 = ComplexTriples::Single((1, 1, c(3.0, 4.0)));
    let result = s1 + s2;
    assert!(
        result == ComplexTriples::Double([(0, 0, c(1.0, 2.0)), (1, 1, c(3.0, 4.0))])
            || result == ComplexTriples::Double([(1, 1, c(3.0, 4.0)), (0, 0, c(1.0, 2.0))])
    );
}

#[test]
fn test_add_single_to_double_match() {
    let s = ComplexTriples::Single((0, 0, c(1.0, 1.0)));
    let d = ComplexTriples::Double([(0, 0, c(2.0, 2.0)), (1, 1, c(3.0, 3.0))]);
    assert_eq!(
        s + d,
        ComplexTriples::Double([(0, 0, c(3.0, 3.0)), (1, 1, c(3.0, 3.0))])
    );
}

#[test]
fn test_add_single_to_double_no_match_expands_to_quad() {
    let s = ComplexTriples::Single((2, 2, c(1.0, 1.0)));
    let d = ComplexTriples::Double([(0, 0, c(2.0, 2.0)), (1, 1, c(3.0, 3.0))]);
    let result = s + d;
    // This will result in 3 unique elements, which should fit into Quad (with a dummy 4th element)
    if let ComplexTriples::Quad(arr) = result {
        let mut found_00 = false;
        let mut found_11 = false;
        let mut found_22 = false;
        for (r, c_val, v) in arr.iter() {
            if *r == 0 && *c_val == 0 && *v == c(2.0, 2.0) {
                found_00 = true;
            }
            if *r == 1 && *c_val == 1 && *v == c(3.0, 3.0) {
                found_11 = true;
            }
            if *r == 2 && *c_val == 2 && *v == c(1.0, 1.0) {
                found_22 = true;
            }
        }
        assert!(found_00 && found_11 && found_22);
    } else {
        panic!("Expected Quad variant, got {:?}", result);
    }
}

#[test]
fn test_add_double_same_rcs() {
    let d1 = ComplexTriples::Double([(0, 0, c(1.0, 1.0)), (1, 1, c(2.0, 2.0))]);
    let d2 = ComplexTriples::Double([(0, 0, c(3.0, 3.0)), (1, 1, c(4.0, 4.0))]);
    let result = d1 + d2;
    if let ComplexTriples::Double(arr) = result {
        let mut found_00 = false;
        let mut found_11 = false;
        for (r, c_val, v) in arr.iter() {
            if *r == 0 && *c_val == 0 && *v == c(4.0, 4.0) {
                found_00 = true;
            }
            if *r == 1 && *c_val == 1 && *v == c(6.0, 6.0) {
                found_11 = true;
            }
        }
        assert!(found_00 && found_11);
    } else {
        panic!("Expected Double variant, got {:?}", result);
    }
}

#[test]
fn test_add_double_no_overlap_expands_to_quad() {
    let d1 = ComplexTriples::Double([(0, 0, c(1.0, 1.0)), (1, 1, c(2.0, 2.0))]);
    let d2 = ComplexTriples::Double([(2, 2, c(3.0, 3.0)), (3, 3, c(4.0, 4.0))]);
    let result = d1 + d2;
    if let ComplexTriples::Quad(arr) = result {
        let mut found_00 = false;
        let mut found_11 = false;
        let mut found_22 = false;
        let mut found_33 = false;
        for (r, c_val, v) in arr.iter() {
            if *r == 0 && *c_val == 0 && *v == c(1.0, 1.0) {
                found_00 = true;
            }
            if *r == 1 && *c_val == 1 && *v == c(2.0, 2.0) {
                found_11 = true;
            }
            if *r == 2 && *c_val == 2 && *v == c(3.0, 3.0) {
                found_22 = true;
            }
            if *r == 3 && *c_val == 3 && *v == c(4.0, 4.0) {
                found_33 = true;
            }
        }
        assert!(found_00 && found_11 && found_22 && found_33);
    } else {
        panic!("Expected Quad variant, got {:?}", result);
    }
}

#[test]
fn test_add_double_to_quad_match() {
    let d = ComplexTriples::Double([(0, 0, c(1.0, 1.0)), (1, 1, c(2.0, 2.0))]);
    let q = ComplexTriples::Quad([
        (0, 0, c(3.0, 3.0)),
        (1, 1, c(4.0, 4.0)),
        (2, 2, c(5.0, 5.0)),
        (3, 3, c(6.0, 6.0)),
    ]);
    let result = d + q;
    if let ComplexTriples::Quad(arr) = result {
        let mut found_00 = false;
        let mut found_11 = false;
        let mut found_22 = false;
        let mut found_33 = false;
        for (r, c_val, v) in arr.iter() {
            if *r == 0 && *c_val == 0 && *v == c(4.0, 4.0) {
                found_00 = true;
            }
            if *r == 1 && *c_val == 1 && *v == c(6.0, 6.0) {
                found_11 = true;
            }
            if *r == 2 && *c_val == 2 && *v == c(5.0, 5.0) {
                found_22 = true;
            }
            if *r == 3 && *c_val == 3 && *v == c(6.0, 6.0) {
                found_33 = true;
            }
        }
        assert!(found_00 && found_11 && found_22 && found_33);
    } else {
        panic!("Expected Quad variant, got {:?}", result);
    }
}

// --------------------------------Pairs Tests--------------------------------
#[test]
fn pairs_test_add_empty() {
    assert_eq!(
        ComplexPairs::Empty + ComplexPairs::Empty,
        ComplexPairs::Empty
    );
    assert_eq!(
        ComplexPairs::Empty + ComplexPairs::Single((0, Complex::new(1.0, 1.0))),
        ComplexPairs::Single((0, Complex::new(1.0, 1.0)))
    );
    assert_eq!(
        ComplexPairs::Single((0, Complex::new(1.0, 1.0))) + ComplexPairs::Empty,
        ComplexPairs::Single((0, Complex::new(1.0, 1.0)))
    );
}

#[test]
fn test_add_single_same_row() {
    let s1 = ComplexPairs::Single((0, Complex::new(1.0, 2.0)));
    let s2 = ComplexPairs::Single((0, Complex::new(3.0, 4.0)));
    assert_eq!(s1 + s2, ComplexPairs::Single((0, Complex::new(4.0, 6.0))));
}

#[test]
fn test_add_single_different_row() {
    let s1 = ComplexPairs::Single((0, Complex::new(1.0, 2.0)));
    let s2 = ComplexPairs::Single((1, Complex::new(3.0, 4.0)));
    // Order might vary, so we need to check both possibilities for the array
    let result = s1 + s2;
    assert!(
        result == ComplexPairs::Double([(0, Complex::new(1.0, 2.0)), (1, Complex::new(3.0, 4.0))])
            || result
                == ComplexPairs::Double([(1, Complex::new(3.0, 4.0)), (0, Complex::new(1.0, 2.0))])
    );
}

#[test]
fn pairs_test_add_single_to_double_match() {
    let s = ComplexPairs::Single((0, Complex::new(1.0, 1.0)));
    let d = ComplexPairs::Double([(0, Complex::new(2.0, 2.0)), (1, Complex::new(3.0, 3.0))]);
    assert_eq!(
        s + d,
        ComplexPairs::Double([(0, Complex::new(3.0, 3.0)), (1, Complex::new(3.0, 3.0))])
    );
}

#[test]
fn test_add_double_same_rows() {
    let d1 = ComplexPairs::Double([(0, Complex::new(1.0, 1.0)), (1, Complex::new(2.0, 2.0))]);
    let d2 = ComplexPairs::Double([(0, Complex::new(3.0, 3.0)), (1, Complex::new(4.0, 4.0))]);
    let _ = ComplexPairs::Double([(0, Complex::new(4.0, 4.0)), (1, Complex::new(6.0, 6.0))]);
    let result = d1 + d2;
    // Due to array order not being guaranteed, we'll check individual elements
    if let ComplexPairs::Double(arr) = result {
        let mut found_0 = false;
        let mut found_1 = false;
        for (row, val) in arr.iter() {
            if *row == 0 && *val == Complex::new(4.0, 4.0) {
                found_0 = true;
            } else if *row == 1 && *val == Complex::new(6.0, 6.0) {
                found_1 = true;
            }
        }
        assert!(found_0 && found_1);
    } else {
        panic!("Expected Double variant");
    }
}

// Helper for approximate float comparison in tests
const EPSILON: f64 = 1e-9;

fn assert_approx_eq_triple(actual: (usize, usize, f64), expected: (usize, usize, f64)) {
    assert_eq!(actual.0, expected.0, "Row mismatch");
    assert_eq!(actual.1, expected.1, "Column mismatch");
    assert!(
        (actual.2 - expected.2).abs() < EPSILON,
        "Value mismatch: actual={}, expected={}",
        actual.2,
        expected.2
    );
}

fn assert_approx_eq_triples(actual: Triples, expected: Triples) {
    match (actual, expected) {
        (Triples::Empty, Triples::Empty) => {}
        (Triples::Single(a), Triples::Single(e)) => assert_approx_eq_triple(a, e),
        (Triples::Double(a_arr), Triples::Double(e_arr)) => {
            // Convert to Vecs, sort, and then compare for order independence
            let mut a_vec: Vec<_> = a_arr.into_iter().collect();
            a_vec.sort_by_key(|t| (t.0, t.1)); // Sort by row then column
            let mut e_vec: Vec<_> = e_arr.into_iter().collect();
            e_vec.sort_by_key(|t| (t.0, t.1));

            assert_eq!(a_vec.len(), e_vec.len());
            for i in 0..a_vec.len() {
                assert_approx_eq_triple(a_vec[i], e_vec[i]);
            }
        }
        (Triples::Quad(a_arr), Triples::Quad(e_arr)) => {
            // Convert to Vecs, filter zeros, sort, and then compare for order independence
            let mut a_vec: Vec<_> = a_arr.into_iter().filter(|&t| t.2.abs() > EPSILON).collect();
            a_vec.sort_by_key(|t| (t.0, t.1));
            let mut e_vec: Vec<_> = e_arr.into_iter().filter(|&t| t.2.abs() > EPSILON).collect();
            e_vec.sort_by_key(|t| (t.0, t.1));

            assert_eq!(a_vec.len(), e_vec.len());
            for i in 0..a_vec.len() {
                assert_approx_eq_triple(a_vec[i], e_vec[i]);
            }
        }
        (a, e) => panic!("Mismatched enum variants: actual={:?}, expected={:?}", a, e),
    }
}

#[test]
fn triples_test_add_empty() {
    assert_approx_eq_triples(Triples::Empty + Triples::Empty, Triples::Empty);
    assert_approx_eq_triples(
        Triples::Empty + Triples::Single((0, 0, 1.0)),
        Triples::Single((0, 0, 1.0)),
    );
    assert_approx_eq_triples(
        Triples::Single((0, 0, 1.0)) + Triples::Empty,
        Triples::Single((0, 0, 1.0)),
    );
}

#[test]
fn triples_test_add_single_same_rc() {
    let s1 = Triples::Single((0, 0, 1.0));
    let s2 = Triples::Single((0, 0, 3.0));
    assert_approx_eq_triples(s1 + s2, Triples::Single((0, 0, 4.0)));
}

#[test]
fn triples_test_add_single_different_rc() {
    let s1 = Triples::Single((0, 0, 1.0));
    let s2 = Triples::Single((1, 1, 3.0));
    assert_approx_eq_triples(s1 + s2, Triples::Double([(0, 0, 1.0), (1, 1, 3.0)]));
}

#[test]
fn triples_test_add_single_to_double_match() {
    let s = Triples::Single((0, 0, 1.0));
    let d = Triples::Double([(0, 0, 2.0), (1, 1, 3.0)]);
    assert_approx_eq_triples(s + d, Triples::Double([(0, 0, 3.0), (1, 1, 3.0)]));
}

#[test]
fn triples_test_add_single_to_double_no_match_expands_to_quad() {
    let s = Triples::Single((2, 2, 1.0));
    let d = Triples::Double([(0, 0, 2.0), (1, 1, 3.0)]);
    // This will result in 3 unique elements, which should fit into Quad (with a dummy 4th element)
    assert_approx_eq_triples(
        s + d,
        Triples::Quad([(0, 0, 2.0), (1, 1, 3.0), (2, 2, 1.0), (0, 0, 0.0)]),
    );
}

#[test]
fn triples_test_add_double_same_rcs() {
    let d1 = Triples::Double([(0, 0, 1.0), (1, 1, 2.0)]);
    let d2 = Triples::Double([(0, 0, 3.0), (1, 1, 4.0)]);
    assert_approx_eq_triples(d1 + d2, Triples::Double([(0, 0, 4.0), (1, 1, 6.0)]));
}

#[test]
fn triples_test_add_double_no_overlap_expands_to_quad() {
    let d1 = Triples::Double([(0, 0, 1.0), (1, 1, 2.0)]);
    let d2 = Triples::Double([(2, 2, 3.0), (3, 3, 4.0)]);
    assert_approx_eq_triples(
        d1 + d2,
        Triples::Quad([(0, 0, 1.0), (1, 1, 2.0), (2, 2, 3.0), (3, 3, 4.0)]),
    );
}

#[test]
fn triples_test_add_double_to_quad_match() {
    let d = Triples::Double([(0, 0, 1.0), (1, 1, 2.0)]);
    let q = Triples::Quad([(0, 0, 3.0), (1, 1, 4.0), (2, 2, 5.0), (3, 3, 6.0)]);
    assert_approx_eq_triples(
        d + q,
        Triples::Quad([(0, 0, 4.0), (1, 1, 6.0), (2, 2, 5.0), (3, 3, 6.0)]),
    );
}

#[test]
fn test_add_resulting_in_zero() {
    let s1 = Triples::Single((0, 0, 5.0));
    let s2 = Triples::Single((0, 0, -5.0));
    assert_approx_eq_triples(s1 + s2, Triples::Empty); // Should result in Empty after filtering
}

#[test]
fn test_double_add_with_some_zeros() {
    let d1 = Triples::Double([(0, 0, 1.0), (1, 1, 2.0)]);
    let d2 = Triples::Double([(0, 0, -1.0), (2, 2, 3.0)]);
    // (0,0) becomes 0.0 and is filtered. Result should be (1,1,2.0) and (2,2,3.0) -> Double
    assert_approx_eq_triples(d1 + d2, Triples::Double([(1, 1, 2.0), (2, 2, 3.0)]));
}
