use num::Complex;
use std::{fmt, ops::Add};

use super::Triples;

/// A structure representing the triples of an element.
///
/// Each triple consists of a row, a column, and a value of type `Complex<f64>`.
#[derive(Clone)]
pub(crate) enum ComplexTriples {
    Empty,
    Single((usize, usize, Complex<f64>)),
    Double([(usize, usize, Complex<f64>); 2]),
    Quad([(usize, usize, Complex<f64>); 4]),
}


impl PartialEq for ComplexTriples {
    fn eq(&self, other: &Self) -> bool {
        let self_triples: Vec<_> = match self {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
        };

        let other_triples: Vec<_> = match other {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
        };

        // Sort both vectors before comparing
        let mut self_triples_sorted = self_triples.clone();
        self_triples_sorted.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut other_triples_sorted = other_triples.clone();
        other_triples_sorted.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        self_triples_sorted == other_triples_sorted
    }
}

impl fmt::Debug for ComplexTriples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_triples: Vec<_> = match self {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
        };

        sorted_triples.sort_by(|(row1, col1, _), (row2, col2, _)| {
            row1.cmp(row2).then_with(|| col1.cmp(col2))
        });

        write!(f, "[")?;
        for (i, (row, col, value)) in sorted_triples.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "({:?}, {:?}, {})", row, col, value)?;
        }
        write!(f, "]")
    }
}

impl From<Triples> for ComplexTriples {
    fn from(value: Triples) -> Self {
        match value {
            Triples::Empty => ComplexTriples::Empty,
            Triples::Single((row, col, val)) => {
                ComplexTriples::Single((row, col, Complex { re: val, im: 0.0 }))
            }
            Triples::Double(triples) => ComplexTriples::Double([
                (
                    triples[0].0,
                    triples[0].1,
                    Complex {
                        re: triples[0].2,
                        im: 0.0,
                    },
                ),
                (
                    triples[1].0,
                    triples[1].1,
                    Complex {
                        re: triples[1].2,
                        im: 0.0,
                    },
                ),
            ]),
            Triples::Quad(triples) => ComplexTriples::Quad([
                (
                    triples[0].0,
                    triples[0].1,
                    Complex {
                        re: triples[0].2,
                        im: 0.0,
                    },
                ),
                (
                    triples[1].0,
                    triples[1].1,
                    Complex {
                        re: triples[1].2,
                        im: 0.0,
                    },
                ),
                (
                    triples[2].0,
                    triples[2].1,
                    Complex {
                        re: triples[2].2,
                        im: 0.0,
                    },
                ),
                (
                    triples[3].0,
                    triples[3].1,
                    Complex {
                        re: triples[3].2,
                        im: 0.0,
                    },
                ),
            ]),
        }
    }
}

#[cfg(test)]
impl ComplexTriples {
    pub fn len(&self) -> usize {
        match self {
            ComplexTriples::Empty => 0,
            ComplexTriples::Single(_) => 1,
            ComplexTriples::Double(_) => 2,
            ComplexTriples::Quad(_) => 4,
        }
    }
}
