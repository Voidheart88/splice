use num::Complex;
use std::iter::FromIterator;
use std::ops::Add;

use super::Pairs;

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ComplexPairs {
    Empty,
    Single((usize, Complex<f64>)),
    Double([(usize, Complex<f64>); 2]),
    Vec(Vec<(usize, Complex<f64>)>),
}

#[cfg(test)]
impl ComplexPairs {
    pub fn is_empty(&self) -> bool {
        matches!(self, ComplexPairs::Empty)
    }
}

impl From<Vec<(usize, Complex<f64>)>> for ComplexPairs {
    /// Creates a `Pairs` object from a vector of pairs.
    fn from(value: Vec<(usize, Complex<f64>)>) -> Self {
        match value.len() {
            0 => ComplexPairs::Empty,
            1 => ComplexPairs::Single(value[0]),
            2 => ComplexPairs::Double([value[0], value[1]]),
            _ => ComplexPairs::Vec(value),
        }
    }
}

impl From<Vec<Complex<f64>>> for ComplexPairs {
    /// Creates a `Pairs` object from a vector of values.
    ///
    /// Each value is converted to a pair (row, value).
    ///
    /// # Examples
    ///
    /// ```
    /// let values = vec![1.0, 2.0, 3.0];
    /// let pairs = ComplexPairs::from(values);
    /// ```
    fn from(values: Vec<Complex<f64>>) -> ComplexPairs {
        let pairs: Vec<(usize, Complex<f64>)> = values
            .into_iter()
            .enumerate()
            .map(|(row_idx, value)| (row_idx, value))
            .collect();
        ComplexPairs::from(pairs)
    }
}

impl FromIterator<(usize, Complex<f64>)> for ComplexPairs {
    /// Creates a `Pairs` object from an iterator of pairs.
    fn from_iter<I: IntoIterator<Item = (usize, Complex<f64>)>>(iter: I) -> Self {
        let vec: Vec<(usize, Complex<f64>)> = iter.into_iter().collect();
        vec.into()
    }
}

impl Add for ComplexPairs {
    type Output = ComplexPairs;

    /// Adds two `Pairs` objects together.
    ///
    /// The inner data are merged, and entries with the same row are summed.
    fn add(self, other: ComplexPairs) -> ComplexPairs {
        let combined: Vec<(usize, Complex<f64>)> = match (self, other) {
            (ComplexPairs::Empty, other) => return other,
            (this, ComplexPairs::Empty) => return this,
            (ComplexPairs::Single(a), ComplexPairs::Single(b)) => vec![a, b],
            (ComplexPairs::Single(a), ComplexPairs::Double(b)) => vec![a, b[0], b[1]],
            (ComplexPairs::Single(a), ComplexPairs::Vec(mut b)) => {
                b.insert(0, a);
                b
            }
            (ComplexPairs::Double(a), ComplexPairs::Single(b)) => vec![a[0], a[1], b],
            (ComplexPairs::Double(a), ComplexPairs::Double(b)) => vec![a[0], a[1], b[0], b[1]],
            (ComplexPairs::Double(a), ComplexPairs::Vec(mut b)) => {
                b.insert(0, a[1]);
                b.insert(0, a[0]);
                b
            }
            (ComplexPairs::Vec(mut a), ComplexPairs::Single(b)) => {
                a.push(b);
                a
            }
            (ComplexPairs::Vec(mut a), ComplexPairs::Double(b)) => {
                a.push(b[0]);
                a.push(b[1]);
                a
            }
            (ComplexPairs::Vec(mut a), ComplexPairs::Vec(mut b)) => {
                a.append(&mut b);
                a
            }
        };

        // Sort by row
        let mut combined = combined;
        combined.sort_by(|a, b| a.0.cmp(&b.0));

        // Combine entries with the same row
        let mut result: Vec<(usize, Complex<f64>)> = Vec::new();
        for (i, &(row, value)) in combined.iter().enumerate() {
            if i == 0 || result.last().unwrap().0 != row {
                result.push((row, value));
            } else {
                let last = result.last_mut().unwrap();
                last.1 += value;
            }
        }

        result.into()
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
            Pairs::Vec(pairs) => pairs
                .iter()
                .map(|(idx, val)| (*idx, Complex { re: *val, im: 0.0 }))
                .collect(),
        }
    }
}
