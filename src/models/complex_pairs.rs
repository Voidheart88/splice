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

#[cfg(test)]
impl ComplexPairs {
    pub fn is_empty(&self) -> bool {
        matches!(self, ComplexPairs::Empty)
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

#[cfg(test)]
mod tests {
    use super::*;
    use num::Complex;

    const EPSILON_SQ: f64 = 1e-18; // EPSILON * EPSILON for norm_sq comparison

    fn assert_approx_eq_complex_pair(
        actual: (usize, Complex<f64>),
        expected: (usize, Complex<f64>),
    ) {
        assert_eq!(actual.0, expected.0, "Row mismatch");
        assert!(
            (actual.1.re - expected.1.re).abs() < f64::EPSILON,
            "Real part mismatch: actual={:?}, expected={:?}",
            actual.1,
            expected.1
        );
        assert!(
            (actual.1.im - expected.1.im).abs() < f64::EPSILON,
            "Imaginary part mismatch: actual={:?}, expected={:?}",
            actual.1,
            expected.1
        );
    }

    fn assert_approx_eq_complex_pairs(actual: ComplexPairs, expected: ComplexPairs) {
        match (actual, expected) {
            (ComplexPairs::Empty, ComplexPairs::Empty) => {}
            (ComplexPairs::Single(a), ComplexPairs::Single(e)) => {
                assert_approx_eq_complex_pair(a, e)
            }
            (ComplexPairs::Double(a_arr), ComplexPairs::Double(e_arr)) => {
                let mut a_vec: Vec<_> = a_arr.into_iter().collect();
                a_vec.sort_by_key(|p| p.0);
                let mut e_vec: Vec<_> = e_arr.into_iter().collect();
                e_vec.sort_by_key(|p| p.0);

                assert_eq!(a_vec.len(), e_vec.len());
                for i in 0..a_vec.len() {
                    assert_approx_eq_complex_pair(a_vec[i], e_vec[i]);
                }
            }
            (ComplexPairs::Vec(a_vec), ComplexPairs::Vec(e_vec)) => {
                let mut sorted_a_vec = a_vec;
                sorted_a_vec.sort_by_key(|p| p.0);
                let mut sorted_e_vec = e_vec;
                sorted_e_vec.sort_by_key(|p| p.0);

                assert_eq!(sorted_a_vec.len(), sorted_e_vec.len());
                for i in 0..sorted_a_vec.len() {
                    assert_approx_eq_complex_pair(sorted_a_vec[i], sorted_e_vec[i]);
                }
            }
            (a, e) => panic!("Mismatched enum variants: actual={:?}, expected={:?}", a, e),
        }
    }

    #[test]
    fn test_add_empty() {
        assert_approx_eq_complex_pairs(
            ComplexPairs::Empty + ComplexPairs::Empty,
            ComplexPairs::Empty,
        );
        assert_approx_eq_complex_pairs(
            ComplexPairs::Empty + ComplexPairs::Single((0, Complex::new(1.0, 1.0))),
            ComplexPairs::Single((0, Complex::new(1.0, 1.0))),
        );
    }

    #[test]
    fn test_add_single_same_row() {
        let s1 = ComplexPairs::Single((0, Complex::new(1.0, 2.0)));
        let s2 = ComplexPairs::Single((0, Complex::new(3.0, 4.0)));
        assert_approx_eq_complex_pairs(s1 + s2, ComplexPairs::Single((0, Complex::new(4.0, 6.0))));
    }

    #[test]
    fn test_add_single_different_row() {
        let s1 = ComplexPairs::Single((0, Complex::new(1.0, 0.0)));
        let s2 = ComplexPairs::Single((1, Complex::new(3.0, 0.0)));
        assert_approx_eq_complex_pairs(
            s1 + s2,
            ComplexPairs::Double([(0, Complex::new(1.0, 0.0)), (1, Complex::new(3.0, 0.0))]),
        );
    }

    #[test]
    fn test_add_single_to_double_match() {
        let s = ComplexPairs::Single((0, Complex::new(1.0, 1.0)));
        let d = ComplexPairs::Double([(0, Complex::new(2.0, 2.0)), (1, Complex::new(3.0, 3.0))]);
        assert_approx_eq_complex_pairs(
            s + d,
            ComplexPairs::Double([(0, Complex::new(3.0, 3.0)), (1, Complex::new(3.0, 3.0))]),
        );
    }

    #[test]
    fn test_add_single_to_double_no_match_expands_to_vec() {
        let s = ComplexPairs::Single((2, Complex::new(1.0, 1.0)));
        let d = ComplexPairs::Double([(0, Complex::new(2.0, 2.0)), (1, Complex::new(3.0, 3.0))]);
        // This should now result in a Vec variant
        let expected_vec = ComplexPairs::Vec(vec![
            (0, Complex::new(2.0, 2.0)),
            (1, Complex::new(3.0, 3.0)),
            (2, Complex::new(1.0, 1.0)),
        ]);
        assert_approx_eq_complex_pairs(s + d, expected_vec);
    }

    #[test]
    fn test_add_double_same_rows() {
        let d1 = ComplexPairs::Double([(0, Complex::new(1.0, 1.0)), (1, Complex::new(2.0, 2.0))]);
        let d2 = ComplexPairs::Double([(0, Complex::new(3.0, 3.0)), (1, Complex::new(4.0, 4.0))]);
        assert_approx_eq_complex_pairs(
            d1 + d2,
            ComplexPairs::Double([(0, Complex::new(4.0, 4.0)), (1, Complex::new(6.0, 6.0))]),
        );
    }

    #[test]
    fn test_add_double_no_overlap_expands_to_vec() {
        let d1 = ComplexPairs::Double([(0, Complex::new(1.0, 1.0)), (1, Complex::new(2.0, 2.0))]);
        let d2 = ComplexPairs::Double([(2, Complex::new(3.0, 3.0)), (3, Complex::new(4.0, 4.0))]);
        // This will result in 4 unique pairs, expands to Vec
        let expected_vec = ComplexPairs::Vec(vec![
            (0, Complex::new(1.0, 1.0)),
            (1, Complex::new(2.0, 2.0)),
            (2, Complex::new(3.0, 3.0)),
            (3, Complex::new(4.0, 4.0)),
        ]);
        assert_approx_eq_complex_pairs(d1 + d2, expected_vec);
    }

    #[test]
    fn test_add_resulting_in_zero() {
        let s1 = ComplexPairs::Single((0, Complex::new(5.0, 5.0)));
        let s2 = ComplexPairs::Single((0, Complex::new(-5.0, -5.0)));
        assert_approx_eq_complex_pairs(s1 + s2, ComplexPairs::Empty); // Should result in Empty after filtering
    }

    #[test]
    fn test_add_vec_variant() {
        let v1 = ComplexPairs::Vec(vec![
            (0, Complex::new(1.0, 1.0)),
            (1, Complex::new(2.0, 2.0)),
            (2, Complex::new(3.0, 3.0)),
        ]);
        let v2 = ComplexPairs::Vec(vec![
            (0, Complex::new(1.0, 1.0)),
            (3, Complex::new(4.0, 4.0)),
        ]);
        let expected = ComplexPairs::Vec(vec![
            (0, Complex::new(2.0, 2.0)),
            (1, Complex::new(2.0, 2.0)),
            (2, Complex::new(3.0, 3.0)),
            (3, Complex::new(4.0, 4.0)),
        ]);
        assert_approx_eq_complex_pairs(v1 + v2, expected);
    }

    #[test]
    fn test_add_single_to_vec() {
        let s = ComplexPairs::Single((4, Complex::new(5.0, 5.0)));
        let v = ComplexPairs::Vec(vec![
            (0, Complex::new(1.0, 1.0)),
            (1, Complex::new(2.0, 2.0)),
        ]);
        let expected = ComplexPairs::Vec(vec![
            (0, Complex::new(1.0, 1.0)),
            (1, Complex::new(2.0, 2.0)),
            (4, Complex::new(5.0, 5.0)),
        ]);
        assert_approx_eq_complex_pairs(s + v, expected);
    }

    #[test]
    fn test_from_pairs_with_vec() {
        // This test assumes `Pairs` also has a Vec variant
        // If `Pairs` does not have a Vec variant, this test would be invalid
        let p_vec = Pairs::Vec(vec![(0, 1.0), (1, 2.0), (2, 3.0)]);
        let expected_cplx_vec = ComplexPairs::Vec(vec![
            (0, Complex::new(1.0, 0.0)),
            (1, Complex::new(2.0, 0.0)),
            (2, Complex::new(3.0, 0.0)),
        ]);
        assert_approx_eq_complex_pairs(ComplexPairs::from(p_vec), expected_cplx_vec);
    }
}
