use num::Complex;

use super::Pairs;

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ComplexPairs {
    Empty,
    Single((usize, Complex<f64>)),
    Double([(usize, Complex<f64>); 2]),
}

#[cfg(test)]
impl ComplexPairs {
    pub fn is_empty(&self) -> bool {
        matches!(self, ComplexPairs::Empty)
    }
}

impl From<Pairs> for ComplexPairs {
    fn from(value: Pairs) -> Self {
        match value {
            Pairs::Empty => ComplexPairs::Empty,
            Pairs::Single((idx, val)) => ComplexPairs::Single((idx, Complex { re: val, im: 0.0 })),
            Pairs::Double(pairs) => ComplexPairs::Double([
                (
                    pairs[0].0,
                    Complex {
                        re: pairs[0].1,
                        im: 0.0,
                    },
                ),
                (
                    pairs[1].0,
                    Complex {
                        re: pairs[1].1,
                        im: 0.0,
                    },
                ),
            ]),
        }
    }
}
