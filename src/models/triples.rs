use crate::solver::{Col, Row};
use rayon::prelude::*;
use std::{cmp::Ordering, fmt, ops::Add};

/// A structure representing the triples of an element.
///
/// Each triple consists of a row, a column, and a value of type `f64`.
#[derive(Clone, PartialOrd)]
pub(crate) enum Triples {
    #[allow(unused)]
    Empty,
    Single((Row, Col, f64)),
    Double([(Row, Col, f64); 2]),
    Quad([(Row, Col, f64); 4]),
    Vec(Vec<(Row, Col, f64)>),
}

impl From<Vec<(Row, Col, f64)>> for Triples {
    /// Creates a `Triples` object from a vector of triples.
    ///
    /// # Examples
    ///
    /// ```
    /// let triples = Triples::from(vec![(1, 2, 3.0), (4, 5, 6.0)]);
    /// ```
    fn from(value: Vec<(Row, Col, f64)>) -> Triples {
        Triples::Vec(value)
    }
}

impl From<Vec<Vec<f64>>> for Triples {
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
    fn from(matrix: Vec<Vec<f64>>) -> Triples {
        let mut triples = Vec::new();
        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                if value != 0.0 {
                    triples.push((Row(row_idx), Col(col_idx), value));
                }
            }
        }
        Triples::from(triples)
    }
}

impl PartialEq for Triples {
    fn eq(&self, other: &Self) -> bool {
        let self_triples: Vec<_> = match self {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![*triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
            Triples::Vec(triples) => triples.clone(),
        };

        let other_triples: Vec<_> = match other {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![*triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
            Triples::Vec(triples) => triples.clone(),
        };

        // Sort both vectors before comparing
        let mut self_triples_sorted = self_triples.clone();
        self_triples_sorted.par_sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
        });

        let mut other_triples_sorted = other_triples.clone();
        other_triples_sorted.par_sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
        });

        self_triples_sorted == other_triples_sorted
    }
}

impl fmt::Debug for Triples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_triples: Vec<_> = match self {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![*triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
            Triples::Vec(triples) => triples.clone(),
        };

        sorted_triples.par_sort_by(|(row1, col1, _), (row2, col2, _)| {
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

impl FromIterator<(Row, Col, f64)> for Triples {
    fn from_iter<I: IntoIterator<Item = (Row, Col, f64)>>(iter: I) -> Self {
        let vec: Vec<_> = iter.into_iter().collect();
        match vec.len() {
            1 => Triples::Single(vec[0]),
            4 => Triples::Quad([vec[0], vec[1], vec[2], vec[3]]),
            _ => Triples::Vec(vec),
        }
    }
}

impl Add for Triples {
    type Output = Triples;

    fn add(self, other: Triples) -> Triples {
        let mut combined: Vec<(Row, Col, f64)> = match self {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
            Triples::Vec(triples) => triples,
        };

        combined.extend(match other {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
            Triples::Vec(triples) => triples,
        });

        combined.par_sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut result: Vec<(Row, Col, f64)> = Vec::new();

        for (i, &(row, col, value)) in combined.iter().enumerate() {
            if i == 0 || result.last().unwrap().0 != row || result.last().unwrap().1 != col {
                result.push((row, col, value));
            } else {
                let last = result.last_mut().unwrap();
                last.2 += value;
            }
        }

        match result.len() {
            1 => Triples::Single(result[0]),
            4 => Triples::Quad([result[0], result[1], result[2], result[3]]),
            _ => Triples::Vec(result),
        }
    }
}

impl Triples {
    #[cfg(test)]
    pub fn len(&self) -> usize {
        match self {
            Triples::Empty => 0,
            Triples::Single(_) => 1,
            Triples::Double(_) => 2,
            Triples::Quad(_) => 4,
            Triples::Vec(v) => v.len(),
        }
    }
}
