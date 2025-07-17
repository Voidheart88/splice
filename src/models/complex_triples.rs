use num::Complex;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

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

impl Add for ComplexTriples {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut combined_elements = Vec::new();

        let push_elements =
            |elements_enum: Self, target_vec: &mut Vec<(usize, usize, Complex<f64>)>| {
                match elements_enum {
                    ComplexTriples::Empty => {}
                    ComplexTriples::Single(s) => target_vec.push(s),
                    ComplexTriples::Double(d) => target_vec.extend_from_slice(&d),
                    ComplexTriples::Quad(q) => target_vec.extend_from_slice(&q),
                    ComplexTriples::Vec(v) => target_vec.extend(v),
                }
            };

        push_elements(self, &mut combined_elements);
        push_elements(other, &mut combined_elements);

        let mut unique_elements_map: HashMap<(usize, usize), Complex<f64>> = HashMap::new();

        for (row, col, val) in combined_elements {
            *unique_elements_map
                .entry((row, col))
                .or_insert(Complex::new(0.0, 0.0)) += val;
        }

        let mut final_elements: Vec<(usize, usize, Complex<f64>)> = unique_elements_map
            .into_iter()
            .map(|((row, col), val)| (row, col, val))
            .filter(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON)
            .collect();

        final_elements.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        Self::from_vec(final_elements)
    }
}

impl ComplexTriples {
    pub fn from_vec(mut elements: Vec<(usize, usize, Complex<f64>)>) -> Self {
        elements.retain(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON);
        elements.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        match elements.len() {
            0 => ComplexTriples::Empty,
            1 => ComplexTriples::Single(elements.remove(0)),
            2 => ComplexTriples::Double([elements.remove(0), elements.remove(0)]),
            3 => ComplexTriples::Quad([
                elements.remove(0),
                elements.remove(0),
                elements.remove(0),
                (0, 0, Complex::new(0.0, 0.0)),
            ]),
            4 => ComplexTriples::Quad([
                elements.remove(0),
                elements.remove(0),
                elements.remove(0),
                elements.remove(0),
            ]),
            _ => ComplexTriples::Vec(elements),
        }
    }
}

impl PartialEq for ComplexTriples {
    fn eq(&self, other: &Self) -> bool {
        let self_triples: Vec<_> = match self {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
            ComplexTriples::Vec(triples) => triples.clone(), // Clone the Vec
        };

        let other_triples: Vec<_> = match other {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
            ComplexTriples::Vec(triples) => triples.clone(),
        };

        let mut self_filtered: Vec<_> = self_triples
            .into_iter()
            .filter(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON)
            .collect();
        let mut other_filtered: Vec<_> = other_triples
            .into_iter()
            .filter(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON)
            .collect();

        self_filtered.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        other_filtered.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        if self_filtered.len() != other_filtered.len() {
            return false;
        }

        for i in 0..self_filtered.len() {
            if self_filtered[i].0 != other_filtered[i].0
                || self_filtered[i].1 != other_filtered[i].1
            {
                return false;
            }
            if (self_filtered[i].2.re - other_filtered[i].2.re).abs() > f64::EPSILON
                || (self_filtered[i].2.im - other_filtered[i].2.im).abs() > f64::EPSILON
            {
                return false;
            }
        }
        true
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

        sorted_triples.retain(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON);

        sorted_triples.sort_by(|(row1, col1, _), (row2, col2, _)| {
            row1.cmp(row2).then_with(|| col1.cmp(col2))
        });

        write!(f, "[")?;
        for (i, (row, col, value)) in sorted_triples.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "({}, {}, {})", row, col, value)?;
        }
        write!(f, "]")
    }
}
