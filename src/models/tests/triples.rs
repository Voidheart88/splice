use num::One;

use crate::models::Triples;
use crate::spot::*;

#[test]
fn init_one_triple() {
    let triplet = (0, 0, Numeric::one());
    let triples = Triples::new(&[triplet]);

    assert_eq!(triples.len(), 1);
    assert_eq!(triples.data(), [(0, 0, Numeric::one())]);
}

#[test]
fn init_two_triple() {
    let triplet_1 = (0, 0, Numeric::one());
    let triplet_2 = (1, 1, Numeric::one() + Numeric::one());
    let triples = Triples::new(&[triplet_1, triplet_2]);

    assert_eq!(triples.len(), 2);
    assert_eq!(
        triples.data(),
        [
            (0, 0, Numeric::one()),
            (1, 1, Numeric::one() + Numeric::one())
        ]
    );
}

#[test]
fn init_complex_two_triple() {
    let triplet_1 = (0, 0, ComplexNumeric::one());
    let triplet_2 = (1, 1, ComplexNumeric::one() + ComplexNumeric::one());
    let triples = Triples::new(&[triplet_1, triplet_2]);

    assert_eq!(triples.len(), 2);
    assert_eq!(
        triples.data(),
        [
            (0, 0, ComplexNumeric::one()),
            (1, 1, ComplexNumeric::one() + ComplexNumeric::one())
        ]
    );
}
