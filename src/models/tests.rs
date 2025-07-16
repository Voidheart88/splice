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
fn test_add_quad_with_overlap() {
    let q1 = ComplexTriples::Quad([
        (0, 0, c(1.0, 1.0)),
        (1, 1, c(1.0, 1.0)),
        (2, 2, c(1.0, 1.0)),
        (3, 3, c(1.0, 1.0)),
    ]);
    let q2 = ComplexTriples::Quad([
        (0, 0, c(1.0, 1.0)),
        (1, 1, c(1.0, 1.0)),
        (4, 4, c(1.0, 1.0)),
        (5, 5, c(1.0, 1.0)),
    ]);
    let result = q1 + q2;
    // This will result in 6 unique elements (0,0), (1,1), (2,2), (3,3), (4,4), (5,5)
    // Which exceeds Quad capacity, so it should return Empty
    assert_eq!(result, ComplexTriples::Empty);
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

#[test]
fn test_add_double_to_quad_overflow() {
    let d = ComplexTriples::Double([(4, 4, c(1.0, 1.0)), (5, 5, c(2.0, 2.0))]);
    let q = ComplexTriples::Quad([
        (0, 0, c(3.0, 3.0)),
        (1, 1, c(4.0, 4.0)),
        (2, 2, c(5.0, 5.0)),
        (3, 3, c(6.0, 6.0)),
    ]);
    let result = d + q;
    // This will result in 6 unique elements, which exceeds Quad capacity
    assert_eq!(result, ComplexTriples::Empty);
}

// --------------------------------Pairs Tests--------------------------------
#[test]
fn test_add_empty() {
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
fn test_add_single_to_double_match() {
    let s = ComplexPairs::Single((0, Complex::new(1.0, 1.0)));
    let d = ComplexPairs::Double([(0, Complex::new(2.0, 2.0)), (1, Complex::new(3.0, 3.0))]);
    assert_eq!(
        s + d,
        ComplexPairs::Double([(0, Complex::new(3.0, 3.0)), (1, Complex::new(3.0, 3.0))])
    );
}

#[test]
fn test_add_single_to_double_no_match_warning() {
    let s = ComplexPairs::Single((2, Complex::new(1.0, 1.0)));
    let d = ComplexPairs::Double([(0, Complex::new(2.0, 2.0)), (1, Complex::new(3.0, 3.0))]);
    // This will print a warning and return Empty based on our current implementation
    assert_eq!(s + d, ComplexPairs::Empty);
}

#[test]
fn test_add_double_same_rows() {
    let d1 = ComplexPairs::Double([(0, Complex::new(1.0, 1.0)), (1, Complex::new(2.0, 2.0))]);
    let d2 = ComplexPairs::Double([(0, Complex::new(3.0, 3.0)), (1, Complex::new(4.0, 4.0))]);
    let expected = ComplexPairs::Double([(0, Complex::new(4.0, 4.0)), (1, Complex::new(6.0, 6.0))]);
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

#[test]
fn test_add_double_different_rows_warning() {
    let d1 = ComplexPairs::Double([(0, Complex::new(1.0, 1.0)), (1, Complex::new(2.0, 2.0))]);
    let d2 = ComplexPairs::Double([(2, Complex::new(3.0, 3.0)), (3, Complex::new(4.0, 4.0))]);
    // This will result in more than 2 unique pairs, so it returns Empty and prints a warning
    assert_eq!(d1 + d2, ComplexPairs::Empty);
}

#[test]
fn test_add_double_partial_overlap() {
    let d1 = ComplexPairs::Double([(0, Complex::new(1.0, 1.0)), (1, Complex::new(2.0, 2.0))]);
    let d2 = ComplexPairs::Double([(1, Complex::new(3.0, 3.0)), (2, Complex::new(4.0, 4.0))]);
    let result = d1 + d2;
    // The expected result should combine (1, 2.0+3.0) and include (0, 1.0) and (2, 4.0)
    // This scenario results in more than two elements, so our current implementation returns Empty.
    assert_eq!(result, ComplexPairs::Empty);
}
