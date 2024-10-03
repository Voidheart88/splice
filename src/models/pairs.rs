/// A structure representing the Pairs of an element.
///
/// Each double consists of a row and a value of type `f64`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Pairs {
    Empty,
    Single((usize, f64)),
    Double([(usize, f64); 2]),
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
        }
    }
}
