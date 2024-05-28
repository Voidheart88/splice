use crate::backends::Row;
use std::iter::FromIterator;
use std::ops::Add;

/// A structure representing the doubles of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Doubles {
    Empty,
    Single((Row, f64)),
    Double([(Row, f64); 2]),
    Vec(Vec<(Row, f64)>),
}

#[cfg(test)]
impl Doubles {
    pub fn is_empty(&self) -> bool {
        matches!(self, Doubles::Empty)
    }

    pub fn len(&self) -> usize {
        match self {
            Doubles::Empty => 0,
            Doubles::Single(_) => 1,
            Doubles::Double(_) => 2,
            Doubles::Vec(v) => v.len(),
        }
    }
}

impl From<Vec<(Row, f64)>> for Doubles {
    /// Creates a `Doubles` object from a vector of doubles.
    fn from(value: Vec<(Row, f64)>) -> Self {
        match value.len() {
            0 => Doubles::Empty,
            1 => Doubles::Single(value[0]),
            2 => Doubles::Double([value[0], value[1]]),
            _ => Doubles::Vec(value),
        }
    }
}

impl FromIterator<(Row, f64)> for Doubles {
    /// Creates a `Doubles` object from an iterator of doubles.
    fn from_iter<I: IntoIterator<Item = (Row, f64)>>(iter: I) -> Self {
        let vec: Vec<(Row, f64)> = iter.into_iter().collect();
        vec.into()
    }
}

impl Add for Doubles {
    type Output = Doubles;

    /// Adds two `Doubles` objects together.
    ///
    /// The inner data are merged, and entries with the same row are summed.
    fn add(self, other: Doubles) -> Doubles {
        let combined: Vec<(Row, f64)> = match (self, other) {
            (Doubles::Empty, other) => return other,
            (this, Doubles::Empty) => return this,
            (Doubles::Single(a), Doubles::Single(b)) => vec![a, b],
            (Doubles::Single(a), Doubles::Double(b)) => vec![a, b[0], b[1]],
            (Doubles::Single(a), Doubles::Vec(mut b)) => {
                b.insert(0, a);
                b
            }
            (Doubles::Double(a), Doubles::Single(b)) => vec![a[0], a[1], b],
            (Doubles::Double(a), Doubles::Double(b)) => vec![a[0], a[1], b[0], b[1]],
            (Doubles::Double(a), Doubles::Vec(mut b)) => {
                b.insert(0, a[1]);
                b.insert(0, a[0]);
                b
            }
            (Doubles::Vec(mut a), Doubles::Single(b)) => {
                a.push(b);
                a
            }
            (Doubles::Vec(mut a), Doubles::Double(b)) => {
                a.push(b[0]);
                a.push(b[1]);
                a
            }
            (Doubles::Vec(mut a), Doubles::Vec(mut b)) => {
                a.append(&mut b);
                a
            }
        };

        // Sort by row
        let mut combined = combined;
        combined.sort_by(|a, b| a.0.cmp(&b.0));

        // Combine entries with the same row
        let mut result: Vec<(Row, f64)> = Vec::new();
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
