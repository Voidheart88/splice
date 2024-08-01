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
    Vec(Vec<(usize, usize, Complex<f64>)>),
}

impl From<Vec<(usize, usize, Complex<f64>)>> for ComplexTriples {
    /// Creates a `Triples` object from a vector of triples.
    ///
    /// # Examples
    ///
    /// ```
    /// let triples = Triples::from(vec![(1, 2, 3.0), (4, 5, 6.0)]);
    /// ```
    fn from(value: Vec<(usize, usize, Complex<f64>)>) -> ComplexTriples {
        ComplexTriples::Vec(value)
    }
}

impl From<Vec<Vec<Complex<f64>>>> for ComplexTriples {
    /// Creates a `Triples` object from a 2D vector of values.
    ///
    /// Each non-zero value is converted to a triple (row, col, value).
    ///
    /// # Examples
    ///
    /// ```
    /// let matrix = vec![
    ///     vec![0.0, 1.0, 0.0],
    ///     vec![0.0, 0.0, 2.0],
    ///     vec![3.0, 0.0, 0.0],
    /// ];
    /// let triples = Triples::from(matrix);
    /// ```
    fn from(matrix: Vec<Vec<Complex<f64>>>) -> ComplexTriples {
        let mut triples = Vec::new();
        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value != Complex::ZERO {
                    triples.push((row_idx, col_idx, value));
                }
            }
        }
        ComplexTriples::from(triples)
    }
}

impl PartialEq for ComplexTriples {
    fn eq(&self, other: &Self) -> bool {
        let self_triples: Vec<_> = match self {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
            ComplexTriples::Vec(triples) => triples.clone(),
        };

        let other_triples: Vec<_> = match other {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
            ComplexTriples::Vec(triples) => triples.clone(),
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
            ComplexTriples::Vec(triples) => triples.clone(),
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

impl FromIterator<(usize, usize, Complex<f64>)> for ComplexTriples {
    fn from_iter<I: IntoIterator<Item = (usize, usize, Complex<f64>)>>(iter: I) -> Self {
        let vec: Vec<_> = iter.into_iter().collect();
        match vec.len() {
            1 => ComplexTriples::Single(vec[0]),
            4 => ComplexTriples::Quad([vec[0], vec[1], vec[2], vec[3]]),
            _ => ComplexTriples::Vec(vec),
        }
    }
}

impl Add for ComplexTriples {
    type Output = ComplexTriples;

    fn add(self, other: ComplexTriples) -> ComplexTriples {
        let mut combined: Vec<(usize, usize, Complex<f64>)> = match self {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
            ComplexTriples::Vec(triples) => triples,
        };

        combined.extend(match other {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
            ComplexTriples::Vec(triples) => triples,
        });

        combined.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut result: Vec<(usize, usize, Complex<f64>)> = Vec::new();

        for (i, &(row, col, value)) in combined.iter().enumerate() {
            if i == 0 || result.last().unwrap().0 != row || result.last().unwrap().1 != col {
                result.push((row, col, value));
            } else {
                let last = result.last_mut().unwrap();
                last.2 += value;
            }
        }

        match result.len() {
            1 => ComplexTriples::Single(result[0]),
            4 => ComplexTriples::Quad([result[0], result[1], result[2], result[3]]),
            _ => ComplexTriples::Vec(result),
        }
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
            Triples::Vec(triples) => triples
                .iter()
                .map(|(row, col, val)| (*row, *col, Complex { re: *val, im: 0.0 }))
                .collect(),
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
            ComplexTriples::Vec(v) => v.len(),
        }
    }
}
