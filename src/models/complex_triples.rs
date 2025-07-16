use num::Complex;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

use crate::models::Triples; // Needed for the HashMap approach

// Assuming `super::Triples` is correctly defined elsewhere and potentially updated with a Vec variant too.

/// A structure representing the triples of an element.
///
/// Each triple consists of a row, a column, and a value of type `Complex<f64>`.
#[derive(Clone)] // Removed PartialEq and Debug from derive, implementing manually for precision and sorting
pub(crate) enum ComplexTriples {
    Empty,
    Single((usize, usize, Complex<f64>)),
    Double([(usize, usize, Complex<f64>); 2]),
    Quad([(usize, usize, Complex<f64>); 4]),
    Vec(Vec<(usize, usize, Complex<f64>)>), // New Vec variant!
}

impl Add for ComplexTriples {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut combined_elements = Vec::new();

        // Helper to push elements from any variant into the Vec
        let push_elements =
            |elements_enum: Self, target_vec: &mut Vec<(usize, usize, Complex<f64>)>| {
                match elements_enum {
                    ComplexTriples::Empty => {}
                    ComplexTriples::Single(s) => target_vec.push(s),
                    ComplexTriples::Double(d) => target_vec.extend_from_slice(&d),
                    ComplexTriples::Quad(q) => target_vec.extend_from_slice(&q),
                    ComplexTriples::Vec(v) => target_vec.extend(v), // Extend directly from the Vec
                }
            };

        // Push elements from `self`
        push_elements(self, &mut combined_elements);
        // Push elements from `other`
        push_elements(other, &mut combined_elements);

        // Now, combine and sum duplicates in `combined_elements` using a HashMap
        // The key for the HashMap will be (row, col)
        let mut unique_elements_map: HashMap<(usize, usize), Complex<f64>> = HashMap::new();

        for (row, col, val) in combined_elements {
            *unique_elements_map
                .entry((row, col))
                .or_insert(Complex::new(0.0, 0.0)) += val;
        }

        // Convert the map back to a Vec of triples, filtering out zero-valued complex numbers
        let mut final_elements: Vec<(usize, usize, Complex<f64>)> = unique_elements_map
            .into_iter()
            .map(|((row, col), val)| (row, col, val))
            .filter(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON) // Use norm_sq for Complex zero check
            .collect();

        // Sort elements for deterministic output (useful for fixed-size arrays and tests)
        final_elements.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        // Now, use the from_vec helper to convert the final Vec into the appropriate enum variant
        Self::from_vec(final_elements)
    }
}

impl ComplexTriples {
    pub fn from_vec(mut elements: Vec<(usize, usize, Complex<f64>)>) -> Self {
        // Ensure no zero-value entries if not already filtered
        elements.retain(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON);
        // Ensure elements are sorted for consistency in fixed-size arrays and tests
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
            ]), // Pad to 4
            4 => ComplexTriples::Quad([
                elements.remove(0),
                elements.remove(0),
                elements.remove(0),
                elements.remove(0),
            ]),
            _ => ComplexTriples::Vec(elements), // If more than 4, store in Vec
        }
    }
}

// Manual PartialEq implementation for float precision and order independence
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
            ComplexTriples::Vec(triples) => triples.clone(), // Clone the Vec
        };

        // Filter out zero-valued Complex numbers for comparison consistency
        let mut self_filtered: Vec<_> = self_triples
            .into_iter()
            .filter(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON)
            .collect();
        let mut other_filtered: Vec<_> = other_triples
            .into_iter()
            .filter(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON)
            .collect();

        // Sort both vectors before comparing
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

// Manual Debug implementation for consistent, sorted output
impl fmt::Debug for ComplexTriples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_triples: Vec<_> = match self {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
            ComplexTriples::Vec(triples) => triples.clone(), // Clone the Vec
        };

        // Filter out zero-valued Complex numbers for cleaner debug output
        sorted_triples.retain(|&(_, _, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON);

        sorted_triples.sort_by(|(row1, col1, _), (row2, col2, _)| {
            row1.cmp(row2).then_with(|| col1.cmp(col2))
        });

        write!(f, "[")?;
        for (i, (row, col, value)) in sorted_triples.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "({}, {}, {})", row, col, value)?; // Use default Complex debug which is fine
        }
        write!(f, "]")
    }
}

// Fix your `From<Triples> for ComplexTriples` implementation to handle Triples::Vec if it exists
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
            // Add the new Vec variant handling for `Triples` if `Triples` also has a Vec variant
            // This assumes `Triples::Vec` would be `Triples::Vec(Vec<(usize, usize, f64)>)`
            #[allow(unreachable_patterns)]
            // If Triples doesn't have a Vec variant, this will be unreachable
            Triples::Vec(triples_vec) => ComplexTriples::Vec(
                triples_vec
                    .into_iter()
                    .map(|(row, col, val)| (row, col, Complex { re: val, im: 0.0 }))
                    .collect(),
            ),
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
            ComplexTriples::Quad(_) => 4, // Quad is fixed at 4, even if some are zero
            ComplexTriples::Vec(v) => v.len(),
        }
    }
}
