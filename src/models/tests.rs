use num::Complex;

use super::*;

// --------------------------------Triples Tests--------------------------------
#[test]
fn test_from_vec() {
    let triples: Triples = vec![vec![1.0, 2.0], vec![3.0, 4.0]].into();

    let exp = Triples::Vec(vec![(0, 0, 1.0), (0, 1, 2.0), (1, 0, 3.0), (1, 1, 4.0)]);

    assert_eq!(triples, exp);
}

#[test]
fn test_addition_no_overlap() {
    let triples1 = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0)]);
    let triples2 = Triples::Vec(vec![(3, 3, 3.0), (4, 4, 4.0)]);
    let expected = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0), (3, 3, 3.0), (4, 4, 4.0)]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_with_overlap() {
    let triples1 = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0)]);
    let triples2 = Triples::Vec(vec![(1, 1, 3.0), (2, 2, 4.0)]);
    let expected = Triples::Vec(vec![(1, 1, 4.0), (2, 2, 6.0)]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_mixed() {
    let triples1 = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0), (3, 3, 3.0)]);
    let triples2 = Triples::Vec(vec![(1, 1, 3.0), (4, 4, 4.0), (3, 3, 3.0)]);
    let expected = Triples::Vec(vec![(1, 1, 4.0), (2, 2, 2.0), (3, 3, 6.0), (4, 4, 4.0)]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_empty() {
    let triples1 = Triples::Vec(vec![]);
    let triples2 = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0)]);
    let expected = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0)]);

    assert_eq!(triples1 + triples2, expected);

    let triples1 = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0)]);
    let triples2 = Triples::Vec(vec![]);
    let expected = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0)]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_both_empty() {
    let triples1 = Triples::Vec(vec![]);
    let triples2 = Triples::Vec(vec![]);
    let expected = Triples::Vec(vec![]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_len() {
    let single = Triples::Single((1, 1, 1.0));
    assert_eq!(single.len(), 1);

    let double = Triples::Double([(1, 1, 1.0), (2, 2, 2.0)]);
    assert_eq!(double.len(), 2);

    let quad = Triples::Quad([(1, 1, 1.0), (2, 2, 2.0), (3, 3, 3.0), (4, 4, 4.0)]);
    assert_eq!(quad.len(), 4);

    let vec_triples = Triples::Vec(vec![(1, 1, 1.0), (2, 2, 2.0), (3, 3, 3.0)]);
    assert_eq!(vec_triples.len(), 3);
}

// --------------------------------pairs Tests--------------------------------

#[test]
fn test_pairs_addition_no_overlap() {
    let pairs1 = Pairs::from(vec![(1, 1.0), (2, 2.0)]);
    let pairs2 = Pairs::from(vec![(3, 3.0), (4, 4.0)]);
    let expected = Pairs::from(vec![(1, 1.0), (2, 2.0), (3, 3.0), (4, 4.0)]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_pairs_addition_with_overlap() {
    let pairs1 = Pairs::from(vec![(1, 1.0), (2, 2.0)]);
    let pairs2 = Pairs::from(vec![(1, 3.0), (2, 4.0)]);
    let expected = Pairs::from(vec![(1, 4.0), (2, 6.0)]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_pairs_addition_mixed() {
    let pairs1 = Pairs::from(vec![(1, 1.0), (2, 2.0), (3, 3.0)]);
    let pairs2 = Pairs::from(vec![(1, 3.0), (4, 4.0), (3, 3.0)]);
    let expected = Pairs::from(vec![(1, 4.0), (2, 2.0), (3, 6.0), (4, 4.0)]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_pairs_addition_empty() {
    let pairs1 = Pairs::Empty;
    let pairs2 = Pairs::from(vec![(1, 1.0), (2, 2.0)]);
    let expected = Pairs::from(vec![(1, 1.0), (2, 2.0)]);

    assert_eq!(pairs1 + pairs2, expected);

    let pairs1 = Pairs::from(vec![(1, 1.0), (2, 2.0)]);
    let pairs2 = Pairs::Empty;
    let expected = Pairs::from(vec![(1, 1.0), (2, 2.0)]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_pairs_addition_both_empty() {
    let pairs1 = Pairs::Empty;
    let pairs2 = Pairs::Empty;
    let expected = Pairs::Empty;

    assert_eq!(pairs1 + pairs2, expected);
}

// ----------------------------Complex Triples Tests----------------------------
#[test]
fn test_cplx_from_vec() {
    let triples: ComplexTriples = vec![
        vec![Complex::new(1.0, 1.0), Complex::new(2.0, 2.0)],
        vec![Complex::new(3.0, 3.0), Complex::new(4.0, 4.0)],
    ]
    .into();

    let exp = ComplexTriples::Vec(vec![
        (0, 0, Complex::new(1.0, 1.0)),
        (0, 1, Complex::new(2.0, 2.0)),
        (1, 0, Complex::new(3.0, 3.0)),
        (1, 1, Complex::new(4.0, 4.0)),
    ]);

    assert_eq!(triples, exp);
}

#[test]
fn test_cplx_addition_no_overlap() {
    let triples1 = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 1.0)),
        (2, 2, Complex::new(2.0, 2.0)),
    ]);
    let triples2 = ComplexTriples::Vec(vec![
        (3, 3, Complex::new(3.0, 3.0)),
        (4, 4, Complex::new(4.0, 4.0)),
    ]);
    let expected = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 1.0)),
        (2, 2, Complex::new(2.0, 2.0)),
        (3, 3, Complex::new(3.0, 3.0)),
        (4, 4, Complex::new(4.0, 4.0)),
    ]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_cplx_addition_with_overlap() {
    let triples1 = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 1.0)),
        (2, 2, Complex::new(2.0, 2.0)),
    ]);
    let triples2 = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(3.0, 3.0)),
        (2, 2, Complex::new(4.0, 4.0)),
    ]);
    let expected = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(4.0, 4.0)),
        (2, 2, Complex::new(6.0, 6.0)),
    ]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_cplx_addition_mixed() {
    let triples1 = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
        (3, 3, Complex::new(3.0, 0.0)),
    ]);
    let triples2 = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(3.0, 0.0)),
        (4, 4, Complex::new(4.0, 0.0)),
        (3, 3, Complex::new(3.0, 0.0)),
    ]);
    let expected = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(4.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
        (3, 3, Complex::new(6.0, 0.0)),
        (4, 4, Complex::new(4.0, 0.0)),
    ]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_cplx_addition_empty() {
    let triples1 = ComplexTriples::Vec(vec![]);
    let triples2 = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
    ]);
    let expected = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
    ]);

    assert_eq!(triples1 + triples2, expected);

    let triples1 = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
    ]);
    let triples2 = ComplexTriples::Vec(vec![]);
    let expected = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
    ]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_cplx_addition_both_empty() {
    let triples1 = ComplexTriples::Vec(vec![]);
    let triples2 = ComplexTriples::Vec(vec![]);
    let expected = ComplexTriples::Vec(vec![]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_cplx_len() {
    let single = ComplexTriples::Single((1, 1, Complex::new(1.0, 0.0)));
    assert_eq!(single.len(), 1);

    let double = Triples::Double([(1, 1, 1.0), (2, 2, 2.0)]);
    assert_eq!(double.len(), 2);

    let quad = ComplexTriples::Quad([
        (1, 1, Complex::new(1.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
        (3, 3, Complex::new(3.0, 0.0)),
        (4, 4, Complex::new(4.0, 0.0)),
    ]);
    assert_eq!(quad.len(), 4);

    let vec_triples = ComplexTriples::Vec(vec![
        (1, 1, Complex::new(1.0, 0.0)),
        (2, 2, Complex::new(2.0, 0.0)),
        (3, 3, Complex::new(3.0, 0.0)),
    ]);
    assert_eq!(vec_triples.len(), 3);
}

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

// -----------------------------complex pairs Tests-----------------------------

#[test]
fn test_cplx_pairs_addition_no_overlap() {
    let pairs1 = ComplexPairs::from(vec![
        (1, Complex { re: 0.0, im: 1.0 }),
        (2, Complex { re: 0.0, im: 2.0 }),
    ]);
    let pairs2 = ComplexPairs::from(vec![
        (3, Complex { re: 0.0, im: 3.0 }),
        (4, Complex { re: 0.0, im: 4.0 }),
    ]);
    let expected = ComplexPairs::from(vec![
        (1, Complex { re: 0.0, im: 1.0 }),
        (2, Complex { re: 0.0, im: 2.0 }),
        (3, Complex { re: 0.0, im: 3.0 }),
        (4, Complex { re: 0.0, im: 4.0 }),
    ]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_cplx_pairs_addition_with_overlap() {
    let pairs1 = ComplexPairs::from(vec![
        (1, Complex { re: 1.0, im: 0.0 }),
        (2, Complex { re: 2.0, im: 0.0 }),
    ]);
    let pairs2 = ComplexPairs::from(vec![
        (1, Complex { re: 3.0, im: 0.0 }),
        (2, Complex { re: 4.0, im: 0.0 }),
    ]);
    let expected = ComplexPairs::from(vec![
        (1, Complex { re: 4.0, im: 0.0 }),
        (2, Complex { re: 6.0, im: 0.0 }),
    ]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_cplx_pairs_addition_mixed() {
    let pairs1 = ComplexPairs::from(vec![
        (1, Complex { re: 1.0, im: 0.0 }),
        (2, Complex { re: 2.0, im: 0.0 }),
        (3, Complex { re: 3.0, im: 0.0 }),
    ]);
    let pairs2 = ComplexPairs::from(vec![
        (1, Complex { re: 3.0, im: 0.0 }),
        (4, Complex { re: 4.0, im: 0.0 }),
        (3, Complex { re: 3.0, im: 0.0 }),
    ]);
    let expected = ComplexPairs::from(vec![
        (1, Complex { re: 4.0, im: 0.0 }),
        (2, Complex { re: 2.0, im: 0.0 }),
        (3, Complex { re: 6.0, im: 0.0 }),
        (4, Complex { re: 4.0, im: 0.0 }),
    ]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_cplx_pairs_addition_empty() {
    let pairs1 = ComplexPairs::Empty;
    let pairs2 = ComplexPairs::from(vec![
        (1, Complex { re: 1.0, im: 0.0 }),
        (2, Complex { re: 2.0, im: 0.0 }),
    ]);
    let expected = ComplexPairs::from(vec![
        (1, Complex { re: 1.0, im: 0.0 }),
        (2, Complex { re: 2.0, im: 0.0 }),
    ]);

    assert_eq!(pairs1 + pairs2, expected);

    let pairs1 = ComplexPairs::from(vec![
        (1, Complex { re: 1.0, im: 0.0 }),
        (2, Complex { re: 2.0, im: 0.0 }),
    ]);
    let pairs2 = ComplexPairs::Empty;
    let expected = ComplexPairs::from(vec![
        (1, Complex { re: 1.0, im: 0.0 }),
        (2, Complex { re: 2.0, im: 0.0 }),
    ]);

    assert_eq!(pairs1 + pairs2, expected);
}

#[test]
fn test_cplx_pairs_addition_both_empty() {
    let pairs1 = ComplexPairs::Empty;
    let pairs2 = ComplexPairs::Empty;
    let expected = ComplexPairs::Empty;

    assert_eq!(pairs1.clone() + pairs2.clone(), expected);
    assert!((pairs1 + pairs2).is_empty())
}
