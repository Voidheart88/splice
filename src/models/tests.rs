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