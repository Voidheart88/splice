use rayon::prelude::*;
use std::iter::FromIterator;
use std::ops::Add;

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Pairs {
    Empty,
    Single((usize, f64)),
    Double([(usize, f64); 2]),
    Vec(Vec<(usize, f64)>),
}

#[cfg(test)]
impl Pairs {
    pub fn is_empty(&self) -> bool {
        matches!(self, Pairs::Empty)
    }

    pub fn len(&self) -> usize {
        match self {
            Pairs::Empty => 0,
            Pairs::Single(_) => 1,
            Pairs::Double(_) => 2,
            Pairs::Vec(v) => v.len(),
        }
    }
}

impl From<Vec<(usize, f64)>> for Pairs {
    /// Creates a `Pairs` object from a vector of pairs.
    fn from(value: Vec<(usize, f64)>) -> Self {
        match value.len() {
            0 => Pairs::Empty,
            1 => Pairs::Single(value[0]),
            2 => Pairs::Double([value[0], value[1]]),
            _ => Pairs::Vec(value),
        }
    }
}

impl From<Vec<f64>> for Pairs {
    /// Creates a `Pairs` object from a vector of values.
    ///
    /// Each value is converted to a pair (row, value).
    ///
    /// # Examples
    ///
    /// ```
    /// let values = vec![1.0, 2.0, 3.0];
    /// let pairs = Pairs::from(values);
    /// ```
    fn from(values: Vec<f64>) -> Pairs {
        let pairs: Vec<(usize, f64)> = values
            .into_par_iter()
            .enumerate()
            .map(|(row_idx, value)| (row_idx, value))
            .collect();
        Pairs::from(pairs)
    }
}

impl FromIterator<(usize, f64)> for Pairs {
    /// Creates a `Pairs` object from an iterator of pairs.
    fn from_iter<I: IntoIterator<Item = (usize, f64)>>(iter: I) -> Self {
        let vec: Vec<(usize, f64)> = iter.into_iter().collect();
        vec.into()
    }
}

impl Add for Pairs {
    type Output = Pairs;

    /// Adds two `Pairs` objects together.
    ///
    /// The inner data are merged, and entries with the same row are summed.
    fn add(self, other: Pairs) -> Pairs {
        let combined: Vec<(usize, f64)> = match (self, other) {
            (Pairs::Empty, other) => return other,
            (this, Pairs::Empty) => return this,
            (Pairs::Single(a), Pairs::Single(b)) => vec![a, b],
            (Pairs::Single(a), Pairs::Double(b)) => vec![a, b[0], b[1]],
            (Pairs::Single(a), Pairs::Vec(mut b)) => {
                b.insert(0, a);
                b
            }
            (Pairs::Double(a), Pairs::Single(b)) => vec![a[0], a[1], b],
            (Pairs::Double(a), Pairs::Double(b)) => vec![a[0], a[1], b[0], b[1]],
            (Pairs::Double(a), Pairs::Vec(mut b)) => {
                b.insert(0, a[1]);
                b.insert(0, a[0]);
                b
            }
            (Pairs::Vec(mut a), Pairs::Single(b)) => {
                a.push(b);
                a
            }
            (Pairs::Vec(mut a), Pairs::Double(b)) => {
                a.push(b[0]);
                a.push(b[1]);
                a
            }
            (Pairs::Vec(mut a), Pairs::Vec(mut b)) => {
                a.append(&mut b);
                a
            }
        };

        // Sort by row
        let mut combined = combined;
        combined.par_sort_by(|a, b| a.0.cmp(&b.0));

        // Combine entries with the same row
        let mut result: Vec<(usize, f64)> = Vec::new();
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
