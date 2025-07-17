use std::ops::Add;
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
    Vec(Vec<(usize, usize, f64)>),
}

impl Add for Triples {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut combined_elements = Vec::new();
        let push_elements =
            |elements_enum: Self, target_vec: &mut Vec<(usize, usize, f64)>| match elements_enum {
                Triples::Empty => {}
                Triples::Single(s) => target_vec.push(s),
                Triples::Double(d) => target_vec.extend_from_slice(&d),
                Triples::Quad(q) => target_vec.extend_from_slice(&q),
                Triples::Vec(v) => target_vec.extend(v),
            };

        push_elements(self, &mut combined_elements);
        push_elements(other, &mut combined_elements);

        let mut unique_elements_map: std::collections::HashMap<(usize, usize), f64> =
            std::collections::HashMap::new();

        for (r, c, val) in combined_elements {
            *unique_elements_map.entry((r, c)).or_insert(0.0) += val;
        }

        let mut final_elements: Vec<(usize, usize, f64)> = unique_elements_map
            .into_iter()
            .map(|((r, c), val)| (r, c, val))
            .filter(|&(_, _, val)| val.abs() > f64::EPSILON)
            .collect();

        final_elements.sort_by_key(|t| (t.0, t.1));
        Self::from_vec(final_elements)
    }
}

impl Triples {
    pub fn from_vec(mut elements: Vec<(usize, usize, f64)>) -> Self {
        elements.retain(|&(_, _, val)| val.abs() > f64::EPSILON);
        elements.sort_by_key(|t| (t.0, t.1));

        match elements.len() {
            0 => Triples::Empty,
            1 => Triples::Single(elements.remove(0)),
            2 => Triples::Double([elements.remove(0), elements.remove(0)]),
            3 => {
                let mut arr: [(usize, usize, f64); 4] = [(0, 0, 0.0); 4];
                for (i, item) in elements.drain(..).enumerate() {
                    arr[i] = item;
                }
                Triples::Quad(arr)
            }
            4 => {
                let mut arr: [(usize, usize, f64); 4] = [(0, 0, 0.0); 4];
                for (i, item) in elements.drain(..).enumerate() {
                    arr[i] = item;
                }
                Triples::Quad(arr)
            }
            _ => Triples::Vec(elements),
        }
    }
}

impl PartialEq for Triples {
    fn eq(&self, other: &Self) -> bool {
        let to_canonical_vec = |t: &Triples| -> Vec<(usize, usize, f64)> {
            let mut temp_vec = match t {
                Triples::Empty => vec![],
                Triples::Single(triple) => vec![*triple],
                Triples::Double(triples) => triples.to_vec(),
                Triples::Quad(triples) => triples.to_vec(),
                Triples::Vec(triples_vec) => triples_vec.clone(),
            };

            temp_vec.retain(|&(_, _, val)| val.abs() > f64::EPSILON);
            temp_vec.sort_by(|a, b| {
                a.0.cmp(&b.0)
                    .then_with(|| a.1.cmp(&b.1))
                    .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
            });
            temp_vec
        };

        let self_triples_canonical = to_canonical_vec(self);
        let other_triples_canonical = to_canonical_vec(other);
        if self_triples_canonical.len() != other_triples_canonical.len() {
            return false;
        }

        self_triples_canonical
            .iter()
            .zip(other_triples_canonical.iter())
            .all(|(a, b)| a.0 == b.0 && a.1 == b.1 && (a.2 - b.2).abs() < f64::EPSILON)
    }
}

impl fmt::Debug for Triples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_triples: Vec<_> = match self {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![*triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
            Triples::Vec(triples_vec) => triples_vec.clone(),
        };

        sorted_triples.retain(|&(_, _, val)| val.abs() > f64::EPSILON);
        sorted_triples.sort_by(|(row1, col1, _), (row2, col2, _)| {
            row1.cmp(row2).then_with(|| col1.cmp(col2))
        });

        write!(f, "Triples[")?;
        for (i, (row, col, value)) in sorted_triples.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "({}, {}, {})", row, col, value)?;
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
            Triples::Vec(vec) => vec.len(),
        }
    }
}
