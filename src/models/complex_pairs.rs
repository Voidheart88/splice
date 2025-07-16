use num::Complex;
use std::collections::HashMap;
use std::ops::Add;

use crate::models::Pairs; // Needed for the HashMap approach

// Assuming `super::Pairs` is correctly defined elsewhere and potentially updated with a Vec variant too.

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq)] // Removed PartialOrd, as Complex doesn't derive it
pub(crate) enum ComplexPairs {
    Empty,
    Single((usize, Complex<f64>)),
    Double([(usize, Complex<f64>); 2]),
    Vec(Vec<(usize, Complex<f64>)>), // New Vec variant
}

impl Add for ComplexPairs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Collect all elements into a temporary Vec, then use from_vec to normalize.
        let mut combined_elements = Vec::new();

        // Helper to push elements from any variant into the Vec
        let push_elements = |elements_enum: Self, target_vec: &mut Vec<(usize, Complex<f64>)>| {
            match elements_enum {
                ComplexPairs::Empty => {}
                ComplexPairs::Single(s) => target_vec.push(s),
                ComplexPairs::Double(d) => target_vec.extend_from_slice(&d),
                ComplexPairs::Vec(v) => target_vec.extend(v),
            }
        };

        // Push elements from `self`
        push_elements(self, &mut combined_elements);
        // Push elements from `other`
        push_elements(other, &mut combined_elements);

        // Now, combine and sum duplicates in `combined_elements` using a HashMap
        let mut unique_elements_map: HashMap<usize, Complex<f64>> = HashMap::new();

        for (row, val) in combined_elements {
            *unique_elements_map
                .entry(row)
                .or_insert(Complex::new(0.0, 0.0)) += val;
        }

        // Convert the map back to a Vec of pairs, filtering out zero-valued complex numbers
        let mut final_elements: Vec<(usize, Complex<f64>)> = unique_elements_map
            .into_iter()
            .map(|(row, val)| (row, val))
            .filter(|&(_, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON) // Use norm_sq for Complex zero check
            .collect();

        // Sort elements for deterministic output (useful for fixed-size arrays and tests)
        final_elements.sort_by_key(|p| p.0);

        // Now, use the from_vec helper to convert the final Vec into the appropriate enum variant
        Self::from_vec(final_elements)
    }
}

// Helper function to convert a Vec of pairs into the appropriate ComplexPairs enum variant
impl ComplexPairs {
    pub fn from_vec(mut elements: Vec<(usize, Complex<f64>)>) -> Self {
        // Ensure no zero-value entries if not already filtered
        elements.retain(|&(_, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON);
        // Ensure elements are sorted for consistency in fixed-size arrays and tests
        elements.sort_by_key(|p| p.0);

        match elements.len() {
            0 => ComplexPairs::Empty,
            1 => ComplexPairs::Single(elements.remove(0)),
            2 => ComplexPairs::Double([elements.remove(0), elements.remove(0)]),
            _ => ComplexPairs::Vec(elements), // If more than 2, store in Vec
        }
    }
}

// Fix your `From<Pairs> for ComplexPairs` implementation to handle Pairs::Vec if it exists
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
            // Add the new Vec variant handling for `Pairs` if `Pairs` also has a Vec variant
            // This assumes `Pairs::Vec` would be `Pairs::Vec(Vec<(usize, f64)>)`
            #[allow(unreachable_patterns)]
            // If Pairs doesn't have a Vec variant, this will be unreachable
            Pairs::Vec(pairs_vec) => ComplexPairs::Vec(
                pairs_vec
                    .into_iter()
                    .map(|(idx, val)| (idx, Complex { re: val, im: 0.0 }))
                    .collect(),
            ),
        }
    }
}
