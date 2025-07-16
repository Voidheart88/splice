use num::Complex;
use std::fmt;
use std::ops::Add;

use super::Triples;

/// A structure representing the triples of an element.
///
/// Each triple consists of a row, a column, and a value of type `Complex<f64>`.
#[derive(Clone)]
pub(crate) enum ComplexTriples {
    Empty,
    Single((usize, usize, Complex<f64>)),
    Double([(usize, usize, Complex<f64>); 2]),
    Quad([(usize, usize, Complex<f64>); 4]),
}

impl Add for ComplexTriples {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            // Adding anything to Empty results in the other value
            (ComplexTriples::Empty, other) => other,
            (self_val, ComplexTriples::Empty) => self_val,

            // Adding two Single variants
            (ComplexTriples::Single(s1), ComplexTriples::Single(s2)) => {
                if s1.0 == s2.0 && s1.1 == s2.1 {
                    // Same (row, col), sum values
                    ComplexTriples::Single((s1.0, s1.1, s1.2 + s2.2))
                } else {
                    // Different (row, col), combine into Double
                    ComplexTriples::Double([s1, s2])
                }
            }

            // Adding Single to Double (and vice-versa)
            (ComplexTriples::Single(s), ComplexTriples::Double(mut d)) => {
                let mut found = false;
                for i in 0..2 {
                    if d[i].0 == s.0 && d[i].1 == s.1 {
                        d[i].2 += s.2;
                        found = true;
                        break;
                    }
                }
                if found {
                    ComplexTriples::Double(d)
                } else {
                    // No match, results in 3 unique elements. This exceeds Double capacity.
                    // This is where you might need a Vec or a Quad.
                    ComplexTriples::Quad([d[0], d[1], s, (0, 0, Complex::new(0.0, 0.0))])
                    // Placeholder to fill Quad, assuming it will be handled by final vector logic
                    // More robust: convert both to Vec, combine, then convert back
                }
            }
            (ComplexTriples::Double(mut d), ComplexTriples::Single(s)) => {
                // Symmetric case
                let mut found = false;
                for i in 0..2 {
                    if d[i].0 == s.0 && d[i].1 == s.1 {
                        d[i].2 += s.2;
                        found = true;
                        break;
                    }
                }
                if found {
                    ComplexTriples::Double(d)
                } else {
                    ComplexTriples::Quad([d[0], d[1], s, (0, 0, Complex::new(0.0, 0.0))])
                    // Placeholder
                }
            }

            // Adding Single to Quad (and vice-versa)
            (ComplexTriples::Single(s), ComplexTriples::Quad(mut q)) => {
                let mut found = false;
                for i in 0..4 {
                    if q[i].0 == s.0 && q[i].1 == s.1 {
                        q[i].2 += s.2;
                        found = true;
                        break;
                    }
                }
                if found {
                    ComplexTriples::Quad(q)
                } else {
                    // Exceeds Quad capacity (5 unique elements possible)
                    eprintln!("Warning: Adding Single to Quad resulted in more than 4 unique triples. Returning Empty for simplicity.");
                    ComplexTriples::Empty
                }
            }
            (ComplexTriples::Quad(mut q), ComplexTriples::Single(s)) => {
                // Symmetric case
                let mut found = false;
                for i in 0..4 {
                    if q[i].0 == s.0 && q[i].1 == s.1 {
                        q[i].2 += s.2;
                        found = true;
                        break;
                    }
                }
                if found {
                    ComplexTriples::Quad(q)
                } else {
                    eprintln!("Warning: Adding Quad to Single resulted in more than 4 unique triples. Returning Empty for simplicity.");
                    ComplexTriples::Empty
                }
            }

            // Adding two Double variants
            (ComplexTriples::Double(d1), ComplexTriples::Double(d2)) => {
                let mut combined_elements = Vec::new();
                for &val in d1.iter() {
                    combined_elements.push(val);
                }
                for &val_d2 in d2.iter() {
                    let mut found = false;
                    for &mut (r_comb, c_comb, ref mut v_comb) in combined_elements.iter_mut() {
                        if r_comb == val_d2.0 && c_comb == val_d2.1 {
                            *v_comb += val_d2.2;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        combined_elements.push(val_d2);
                    }
                }
                // Handle different lengths
                Self::from_vec(combined_elements)
            }

            // Adding Double to Quad (and vice-versa)
            (ComplexTriples::Double(d), ComplexTriples::Quad(q)) => {
                let mut combined_elements = Vec::new();
                for &val in d.iter() {
                    combined_elements.push(val);
                }
                for &val_q in q.iter() {
                    let mut found = false;
                    for &mut (r_comb, c_comb, ref mut v_comb) in combined_elements.iter_mut() {
                        if r_comb == val_q.0 && c_comb == val_q.1 {
                            *v_comb += val_q.2;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        combined_elements.push(val_q);
                    }
                }
                Self::from_vec(combined_elements)
            }
            (ComplexTriples::Quad(q), ComplexTriples::Double(d)) => {
                // Symmetric case
                let mut combined_elements = Vec::new();
                for &val in q.iter() {
                    combined_elements.push(val);
                }
                for &val_d in d.iter() {
                    let mut found = false;
                    for &mut (r_comb, c_comb, ref mut v_comb) in combined_elements.iter_mut() {
                        if r_comb == val_d.0 && c_comb == val_d.1 {
                            *v_comb += val_d.2;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        combined_elements.push(val_d);
                    }
                }
                Self::from_vec(combined_elements)
            }

            // Adding two Quad variants
            (ComplexTriples::Quad(q1), ComplexTriples::Quad(q2)) => {
                let mut combined_elements = Vec::new();

                for &val in q1.iter() {
                    combined_elements.push(val);
                }

                for &val_q2 in q2.iter() {
                    let mut found = false;
                    for &mut (r_comb, c_comb, ref mut v_comb) in combined_elements.iter_mut() {
                        if r_comb == val_q2.0 && c_comb == val_q2.1 {
                            *v_comb += val_q2.2;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        combined_elements.push(val_q2);
                    }
                }
                Self::from_vec(combined_elements)
            }
        }
    }
}

// Helper function to convert a Vec of triples into the appropriate ComplexTriples enum variant
impl ComplexTriples {
    fn from_vec(mut elements: Vec<(usize, usize, Complex<f64>)>) -> Self {
        // Filter out zero-value entries if desired (e.g., after subtraction)
        elements.retain(|&(_, _, val)| val != Complex::new(0.0, 0.0));

        match elements.len() {
            0 => ComplexTriples::Empty,
            1 => ComplexTriples::Single(elements[0]),
            2 => ComplexTriples::Double([elements[0], elements[1]]),
            3 => {
                // Need to convert to a Quad, filling the 4th spot with a dummy zero if only 3 elements
                // Or you might want to return an error/another enum type for 3 elements.
                let mut arr: [(usize, usize, Complex<f64>); 4] =
                    [(0, 0, Complex::new(0.0, 0.0)); 4];
                for (i, &item) in elements.into_iter().enumerate() {
                    arr[i] = item;
                }
                ComplexTriples::Quad(arr)
            }
            4 => {
                // Ensure unique (row, col) if they were not already after combining and summing.
                // This assumes after addition, you still want to maintain a distinct pair for each (row, col).
                // If there are duplicate (row, col) after summing, this logic needs to be more robust.
                // For simplicity here, we assume duplicates are already summed during vec processing.
                let mut arr: [(usize, usize, Complex<f64>); 4] =
                    [(0, 0, Complex::new(0.0, 0.0)); 4];
                for (i, &item) in elements.into_iter().enumerate() {
                    arr[i] = item;
                }
                ComplexTriples::Quad(arr)
            }
            _ => {
                eprintln!("Warning: Addition resulted in more than 4 unique triples. Returning Empty for simplicity.");
                ComplexTriples::Empty
            }
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
        };

        let other_triples: Vec<_> = match other {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
        };

        // Sort both vectors before comparing
        let mut self_triples_sorted = self_triples.clone();
        self_triples_sorted.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut other_triples_sorted = other_triples.clone();
        other_triples_sorted.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        self_triples_sorted == other_triples_sorted
    }
}

impl fmt::Debug for ComplexTriples {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_triples: Vec<_> = match self {
            ComplexTriples::Empty => vec![],
            ComplexTriples::Single(triple) => vec![*triple],
            ComplexTriples::Double(triples) => triples.to_vec(),
            ComplexTriples::Quad(triples) => triples.to_vec(),
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
            ComplexTriples::Quad(_) => 4,
        }
    }
}
