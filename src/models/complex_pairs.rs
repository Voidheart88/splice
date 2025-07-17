use num::Complex;
use std::collections::HashMap;
use std::ops::Add;

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ComplexPairs {
    Empty,
    Single((usize, Complex<f64>)),
    Double([(usize, Complex<f64>); 2]),
    Vec(Vec<(usize, Complex<f64>)>),
}

impl ComplexPairs {
    fn from_vec(mut elements: Vec<(usize, Complex<f64>)>) -> Self {
        elements.retain(|&(_, val)| val.norm_sqr() > f64::EPSILON * f64::EPSILON);
        elements.sort_by_key(|p| p.0);

        match elements.len() {
            0 => ComplexPairs::Empty,
            1 => ComplexPairs::Single(elements.remove(0)),
            2 => ComplexPairs::Double([elements.remove(0), elements.remove(0)]),
            _ => ComplexPairs::Vec(elements),
        }
    }

    fn to_vec(&self) -> Vec<(usize, Complex<f64>)> {
        match self {
            ComplexPairs::Empty => Vec::new(),
            ComplexPairs::Single(s) => vec![*s],
            ComplexPairs::Double(d) => d.to_vec(),
            ComplexPairs::Vec(v) => v.clone(),
        }
    }
}

impl Add for ComplexPairs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (ComplexPairs::Empty, other) => other,
            (self_val, ComplexPairs::Empty) => self_val,

            (ComplexPairs::Single(s1), ComplexPairs::Single(s2)) => {
                if s1.0 == s2.0 {
                    let combined_val = s1.1 + s2.1;
                    if combined_val.norm_sqr() > f64::EPSILON * f64::EPSILON {
                        ComplexPairs::Single((s1.0, combined_val))
                    } else {
                        ComplexPairs::Empty
                    }
                } else {
                    let mut arr = [(0, Complex::new(0.0, 0.0)); 2];
                    if s1.0 < s2.0 {
                        arr[0] = s1;
                        arr[1] = s2;
                    } else {
                        arr[0] = s2;
                        arr[1] = s1;
                    }
                    ComplexPairs::Double(arr)
                }
            }

            (ComplexPairs::Single(s), ComplexPairs::Double(d))
            | (ComplexPairs::Double(d), ComplexPairs::Single(s)) => {
                let mut combined_elements = Vec::with_capacity(3);
                combined_elements.push(s);
                combined_elements.extend_from_slice(&d);
                process_elements_to_complex_pairs(combined_elements)
            }

            (ComplexPairs::Double(d1), ComplexPairs::Double(d2)) => {
                let mut combined_elements = Vec::with_capacity(4);
                combined_elements.extend_from_slice(&d1);
                combined_elements.extend_from_slice(&d2);
                process_elements_to_complex_pairs(combined_elements)
            }

            (ComplexPairs::Vec(mut v), other_val) => {
                v.extend(other_val.to_vec());
                process_elements_to_complex_pairs(v)
            }
            (other_val, ComplexPairs::Vec(v)) => {
                let mut temp_vec = other_val.to_vec();
                temp_vec.extend(v);
                process_elements_to_complex_pairs(temp_vec)
            }
        }
    }
}

fn process_elements_to_complex_pairs(elements: Vec<(usize, Complex<f64>)>) -> ComplexPairs {
    let mut unique_elements_map: HashMap<usize, Complex<f64>> = HashMap::new();
    for (row, val) in elements {
        *unique_elements_map
            .entry(row)
            .or_insert(Complex::new(0.0, 0.0)) += val;
    }

    let final_elements: Vec<(usize, Complex<f64>)> = unique_elements_map
        .into_iter()
        .map(|(row, val)| (row, val))
        .collect();

    ComplexPairs::from_vec(final_elements)
}
