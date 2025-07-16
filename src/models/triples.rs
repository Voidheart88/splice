use std::ops::Add;
use std::{cmp::Ordering, fmt};

/// A structure representing the triples of an element.
///
/// Each triple consists of a row, a column, and a value of type `f64`.
#[derive(Clone, PartialOrd)] // Added Debug and PartialEq for testing
pub(crate) enum Triples {
    #[allow(unused)]
    Empty,
    Single((usize, usize, f64)),
    Double([(usize, usize, f64); 2]),
    Quad([(usize, usize, f64); 4]),
    Vec(Vec<(usize, usize, f64)>), // New variant!
}

impl Add for Triples {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Collect all elements into a temporary Vec, then use from_vec to normalize.
        let mut combined_elements = Vec::new();

        // Helper to push elements from any variant into the Vec
        let push_elements = |elements_enum: Self, target_vec: &mut Vec<(usize, usize, f64)>| {
            match elements_enum {
                Triples::Empty => {}
                Triples::Single(s) => target_vec.push(s),
                Triples::Double(d) => target_vec.extend_from_slice(&d),
                Triples::Quad(q) => target_vec.extend_from_slice(&q),
                Triples::Vec(v) => target_vec.extend(v), // Extend directly from the Vec
            }
        };

        // Push elements from `self`
        push_elements(self, &mut combined_elements);
        // Push elements from `other`
        push_elements(other, &mut combined_elements);

        // Now, combine and sum duplicates in `combined_elements`
        let mut unique_elements_map: std::collections::HashMap<(usize, usize), f64> =
            std::collections::HashMap::new();

        for (r, c, val) in combined_elements {
            *unique_elements_map.entry((r, c)).or_insert(0.0) += val;
        }

        // Convert the map back to a Vec of triples, filtering out zeros
        let mut final_elements: Vec<(usize, usize, f64)> = unique_elements_map
            .into_iter()
            .map(|((r, c), val)| (r, c, val))
            .filter(|&(_, _, val)| val.abs() > f64::EPSILON)
            .collect();

        // Sort elements for deterministic output if you care about the order within the Vec variant
        // This is good practice for consistency, especially in tests
        final_elements.sort_by_key(|t| (t.0, t.1));

        // Now, use the from_vec helper to convert the final Vec into the appropriate enum variant
        Self::from_vec(final_elements)
    }
}

// Helper function to convert a Vec of triples into the appropriate Triples enum variant
impl Triples {
    pub fn from_vec(mut elements: Vec<(usize, usize, f64)>) -> Self {
        // Ensure no zero-value entries if not already filtered
        elements.retain(|&(_, _, val)| val.abs() > f64::EPSILON);
        // Ensure elements are sorted for consistency in fixed-size arrays and tests
        elements.sort_by_key(|t| (t.0, t.1));

        match elements.len() {
            0 => Triples::Empty,
            1 => Triples::Single(elements.remove(0)),
            2 => Triples::Double([elements.remove(0), elements.remove(0)]),
            3 => {
                let mut arr: [(usize, usize, f64); 4] = [(0, 0, 0.0); 4];
                // Use `drain(..)` to take elements out of the Vec efficiently
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
            _ => {
                // Now, if more than 4, we store it in the Vec variant
                Triples::Vec(elements)
            }
        }
    }
}

impl PartialEq for Triples {
    fn eq(&self, other: &Self) -> bool {
        // Helper to convert any Triples variant into a canonical Vec for comparison
        let to_canonical_vec = |t: &Triples| -> Vec<(usize, usize, f64)> {
            let mut temp_vec = match t {
                Triples::Empty => vec![],
                Triples::Single(triple) => vec![*triple],
                Triples::Double(triples) => triples.to_vec(),
                Triples::Quad(triples) => triples.to_vec(),
                Triples::Vec(triples_vec) => triples_vec.clone(), // Clone the inner Vec
            };

            // Filter out zero-value entries for robust comparison, as they are often treated as equivalent to non-existence
            temp_vec.retain(|&(_, _, val)| val.abs() > f64::EPSILON);

            // Sort before comparing to ensure order independence
            temp_vec.sort_by(|a, b| {
                a.0.cmp(&b.0)
                    .then_with(|| a.1.cmp(&b.1))
                    // For f64, use partial_cmp. unwrap_or(Ordering::Equal) handles NaNs,
                    // but usually you want NaNs to not be equal, so typically use if let Some.
                    // For typical numeric data, this is often fine.
                    .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
            });
            temp_vec
        };

        let self_triples_canonical = to_canonical_vec(self);
        let other_triples_canonical = to_canonical_vec(other);

        // Finally, compare the canonical (sorted, non-zero) vectors
        // Note: For floating-point equality, it's better to compare with epsilon,
        // but `Vec == Vec` uses element-wise `PartialEq` which for f64 uses exact equality.
        // If exact equality for f64 is not desired, this would need a custom loop.
        // Given your current `sort_by` which uses `partial_cmp`, this means
        // your float comparison within the sort key is robust. However, `==` for `f64`
        // is exact. We should manually compare floats.
        if self_triples_canonical.len() != other_triples_canonical.len() {
            return false;
        }

        self_triples_canonical
            .iter()
            .zip(other_triples_canonical.iter())
            .all(|(a, b)| a.0 == b.0 && a.1 == b.1 && (a.2 - b.2).abs() < f64::EPSILON)
    }
}

// Fix your Debug implementation
impl fmt::Debug for Triples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_triples: Vec<_> = match self {
            Triples::Empty => vec![],
            Triples::Single(triple) => vec![*triple],
            Triples::Double(triples) => triples.to_vec(),
            Triples::Quad(triples) => triples.to_vec(),
            Triples::Vec(triples_vec) => triples_vec.clone(), // Clone the inner Vec
        };

        // Filter out zero-value entries for cleaner debug output if desired
        sorted_triples.retain(|&(_, _, val)| val.abs() > f64::EPSILON);

        sorted_triples.sort_by(|(row1, col1, _), (row2, col2, _)| {
            row1.cmp(row2).then_with(|| col1.cmp(col2))
        });

        write!(f, "Triples[")?; // Indicate it's a Triples enum
        for (i, (row, col, value)) in sorted_triples.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "({}, {}, {})", row, col, value)?; // Simplified output
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
