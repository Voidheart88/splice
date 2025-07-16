use std::{cmp::Ordering, fmt};

/// A structure representing the triples of an element.
///
/// Each triple consists of a row, a column, and a value of type `f64`.
#[derive(Clone, PartialOrd)]
pub(crate) enum Triples {
    #[allow(unused)]
    Empty,
    Single((usize, usize, f64)),
    Double([(usize, usize, f64); 2]),
    Quad([(usize, usize, f64); 4]),
}

impl PartialEq for Triples {
    fn eq(&self, other: &Self) -> bool {
        let self_triples: Vec<_> = match self {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![*triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
        };

        let other_triples: Vec<_> = match other {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![*triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
        };

        // Sort both vectors before comparing
        let mut self_triples_sorted = self_triples.clone();
        self_triples_sorted.sort_by(|a, b| {
            a.0.cmp(&b.0)
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
        });

        let mut other_triples_sorted = other_triples.clone();
        other_triples_sorted.sort_by(|a, b| {
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

impl Triples {
    #[cfg(test)]
    pub fn len(&self) -> usize {
        match self {
            Triples::Empty => 0,
            Triples::Single(_) => 1,
            Triples::Double(_) => 2,
            Triples::Quad(_) => 4,
        }
    }
}
