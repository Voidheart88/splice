use std::collections::HashMap;
use std::ops::Add; // Needed for the HashMap approach

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Pairs {
    Empty,
    Single((usize, f64)),
    Double([(usize, f64); 2]),
    Vec(Vec<(usize, f64)>), // New Vec variant!
}

impl Add for Pairs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Collect all elements into a temporary Vec, then use from_vec to normalize.
        let mut combined_elements = Vec::new();

        // Helper to push elements from any variant into the Vec
        let push_elements = |elements_enum: Self, target_vec: &mut Vec<(usize, f64)>| {
            match elements_enum {
                Pairs::Empty => {}
                Pairs::Single(s) => target_vec.push(s),
                Pairs::Double(d) => target_vec.extend_from_slice(&d),
                Pairs::Vec(v) => target_vec.extend(v), // Extend directly from the Vec
            }
        };

        // Push elements from `self`
        push_elements(self, &mut combined_elements);
        // Push elements from `other`
        push_elements(other, &mut combined_elements);

        // Now, combine and sum duplicates in `combined_elements` using a HashMap
        let mut unique_elements_map: HashMap<usize, f64> = HashMap::new();

        for (row, val) in combined_elements {
            *unique_elements_map.entry(row).or_insert(0.0) += val;
        }

        // Convert the map back to a Vec of pairs, filtering out zero values
        let mut final_elements: Vec<(usize, f64)> = unique_elements_map
            .into_iter()
            .map(|(row, val)| (row, val))
            .filter(|&(_, val)| val.abs() > f64::EPSILON) // Filter out values effectively zero
            .collect();

        // Sort elements for deterministic output (useful for fixed-size arrays and tests)
        final_elements.sort_by_key(|p| p.0);

        // Now, use the from_vec helper to convert the final Vec into the appropriate enum variant
        Self::from_vec(final_elements)
    }
}

// Helper function to convert a Vec of pairs into the appropriate Pairs enum variant
impl Pairs {
    pub fn from_vec(mut elements: Vec<(usize, f64)>) -> Self {
        // Ensure no zero-value entries if not already filtered
        elements.retain(|&(_, val)| val.abs() > f64::EPSILON);
        // Ensure elements are sorted for consistency in fixed-size arrays and tests
        elements.sort_by_key(|p| p.0);

        match elements.len() {
            0 => Pairs::Empty,
            1 => Pairs::Single(elements.remove(0)),
            2 => Pairs::Double([elements.remove(0), elements.remove(0)]),
            _ => Pairs::Vec(elements), // If more than 2, store in Vec
        }
    }
}

// Keep the `is_empty` method if it's used elsewhere, otherwise it's redundant with `len() == 0`
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
            Pairs::Vec(vec) => vec.len(),
        }
    }
}
