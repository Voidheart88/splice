use num::Complex;
use std::ops::Add;

use super::Pairs;

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ComplexPairs {
    Empty,
    Single((usize, Complex<f64>)),
    Double([(usize, Complex<f64>); 2]),
}

impl Add for ComplexPairs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            // Adding anything to Empty results in the other value
            (ComplexPairs::Empty, other) => other,
            (self_val, ComplexPairs::Empty) => self_val,

            // Adding two Single variants
            (ComplexPairs::Single(s1), ComplexPairs::Single(s2)) => {
                // If they have the same row, add their Complex values
                if s1.0 == s2.0 {
                    ComplexPairs::Single((s1.0, s1.1 + s2.1))
                } else {
                    // If rows are different, combine into a Double
                    // You might need to decide on an ordering or sorting if this is important
                    // For simplicity here, we just put them in an array.
                    ComplexPairs::Double([s1, s2])
                }
            }

            // Adding a Single to a Double
            (ComplexPairs::Single(s), ComplexPairs::Double(mut d)) => {
                // Check if the single element's row matches one in the double
                if d[0].0 == s.0 {
                    d[0].1 += s.1;
                    ComplexPairs::Double(d)
                } else if d[1].0 == s.0 {
                    d[1].1 += s.1;
                    ComplexPairs::Double(d)
                } else {
                    // If no match, this scenario implies you'd have more than 2 pairs,
                    // which isn't covered by your current enum definition.
                    // You might need a different enum variant (e.g., `Multiple(Vec<(usize, Complex<f64>)>)`)
                    // or a strategy to handle overflow beyond two pairs.
                    // For now, we'll return an Empty or panic, depending on desired strictness.
                    // A more robust solution might return an error or a different enum variant.
                    // Here, we'll just return Empty to avoid a panic, but this might not be
                    // the desired behavior in a real application.
                    eprintln!("Warning: Adding Single to Double resulted in more than 2 unique pairs. Returning Empty for simplicity.");
                    ComplexPairs::Empty
                }
            }
            (ComplexPairs::Double(d), ComplexPairs::Single(s)) => {
                // Symmetric case to the above
                if d[0].0 == s.0 {
                    let mut new_d = d;
                    new_d[0].1 += s.1;
                    ComplexPairs::Double(new_d)
                } else if d[1].0 == s.0 {
                    let mut new_d = d;
                    new_d[1].1 += s.1;
                    ComplexPairs::Double(new_d)
                } else {
                    eprintln!("Warning: Adding Double to Single resulted in more than 2 unique pairs. Returning Empty for simplicity.");
                    ComplexPairs::Empty
                }
            }

            // Adding two Double variants
            (ComplexPairs::Double(d1), ComplexPairs::Double(d2)) => {
                let mut combined_elements = Vec::new();

                // Add elements from d1
                for &(row, val) in d1.iter() {
                    combined_elements.push((row, val));
                }

                // Add elements from d2, handling potential overlaps
                for &(row_d2, val_d2) in d2.iter() {
                    let mut found = false;
                    for &mut (ref row_combined, ref mut val_combined) in
                        combined_elements.iter_mut()
                    {
                        if *row_combined == row_d2 {
                            *val_combined += val_d2;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        combined_elements.push((row_d2, val_d2));
                    }
                }

                // After combining, you'll need to decide how to handle the result.
                // If `combined_elements.len()` is:
                // - 0: return Empty
                // - 1: return Single
                // - 2: return Double
                // - >2: This indicates a limitation of your current enum structure.
                //   You'll need to decide on a truncation strategy, return an error,
                //   or introduce a new enum variant for more than two pairs.
                match combined_elements.len() {
                    0 => ComplexPairs::Empty,
                    1 => ComplexPairs::Single(combined_elements[0]),
                    2 => {
                        // Ensure unique rows if they were not already.
                        // This assumes after addition, you still want to maintain a distinct pair for each row.
                        if combined_elements[0].0 == combined_elements[1].0 {
                            ComplexPairs::Single((
                                combined_elements[0].0,
                                combined_elements[0].1 + combined_elements[1].1,
                            ))
                        } else {
                            ComplexPairs::Double([combined_elements[0], combined_elements[1]])
                        }
                    }
                    _ => {
                        eprintln!("Warning: Adding two Double variants resulted in more than 2 unique pairs. Returning Empty for simplicity.");
                        ComplexPairs::Empty
                    }
                }
            }
        }
    }
}

#[cfg(test)]
impl ComplexPairs {
    pub fn is_empty(&self) -> bool {
        matches!(self, ComplexPairs::Empty)
    }
}

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
        }
    }
}
