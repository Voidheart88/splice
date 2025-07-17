use std::ops::Add;

/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Pairs {
    Empty,
    Single((usize, f64)),
    Double([(usize, f64); 2]),
    Vec(Vec<(usize, f64)>),
}

impl Pairs {
    fn normalize_elements(mut elements: Vec<(usize, f64)>) -> Vec<(usize, f64)> {
        if elements.is_empty() {
            return elements;
        }

        elements.sort_unstable_by_key(|p| p.0);
        let mut write_idx = 0;
        for read_idx in 0..elements.len() {
            let (current_row, current_val) = elements[read_idx];

            if write_idx > 0 && elements[write_idx - 1].0 == current_row {
                elements[write_idx - 1].1 += current_val;
            } else {
                if read_idx != write_idx {
                    elements[write_idx] = elements[read_idx];
                }
                write_idx += 1;
            }
        }

        elements.truncate(write_idx);
        let mut final_elements: Vec<(usize, f64)> = elements
            .into_iter()
            .filter(|&(_, val)| val.abs() > f64::EPSILON)
            .collect();
        final_elements.sort_by_key(|p| p.0);

        final_elements
    }
}

impl Add for Pairs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Pairs::Empty, other) => other,
            (self_val, Pairs::Empty) => self_val,

            (Pairs::Single((r1, v1)), Pairs::Single((r2, v2))) => {
                if r1 == r2 {
                    let sum = v1 + v2;
                    if sum.abs() > f64::EPSILON {
                        Pairs::Single((r1, sum))
                    } else {
                        Pairs::Empty
                    }
                } else {
                    let mut elements = [(r1, v1), (r2, v2)];
                    elements.sort_by_key(|p| p.0);
                    Pairs::Double(elements)
                }
            }

            (Pairs::Single(s), Pairs::Double(d)) | (Pairs::Double(d), Pairs::Single(s)) => {
                let mut combined = Vec::with_capacity(3);
                combined.push(s);
                combined.extend_from_slice(&d);
                Self::from_vec(Pairs::normalize_elements(combined))
            }

            (Pairs::Double(d1), Pairs::Double(d2)) => {
                let mut combined = Vec::with_capacity(4);
                combined.extend_from_slice(&d1);
                combined.extend_from_slice(&d2);
                Self::from_vec(Pairs::normalize_elements(combined))
            }

            (Pairs::Vec(mut v1), Pairs::Vec(v2)) => {
                v1.extend(v2);
                Self::from_vec(Pairs::normalize_elements(v1))
            }
            (Pairs::Vec(mut v), s_or_d) => {
                match s_or_d {
                    Pairs::Single(s) => v.push(s),
                    Pairs::Double(d) => v.extend_from_slice(&d),
                    _ => unreachable!(),
                }
                Self::from_vec(Pairs::normalize_elements(v))
            }
            (s_or_d, Pairs::Vec(v)) => {
                let mut temp_vec = Vec::new();
                match s_or_d {
                    Pairs::Single(s) => temp_vec.push(s),
                    Pairs::Double(d) => temp_vec.extend_from_slice(&d),
                    _ => unreachable!(),
                }
                temp_vec.extend(v);
                Self::from_vec(Pairs::normalize_elements(temp_vec))
            }
        }
    }
}

impl Pairs {
    pub fn from_vec(mut elements: Vec<(usize, f64)>) -> Self {
        elements.retain(|&(_, val)| val.abs() > f64::EPSILON);
        elements.sort_by_key(|p| p.0);

        match elements.len() {
            0 => Pairs::Empty,
            1 => Pairs::Single(elements.remove(0)),
            2 => Pairs::Double([elements.remove(0), elements.remove(0)]),
            _ => Pairs::Vec(elements),
        }
    }
}

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
