/// A structure representing pairs of an element.
///
/// Each pair consists of a row and a value. The struct has a compile-time
/// fixed capacity `N`, but `length` tracks the actual number of valid elements currently stored.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Pairs<const N: usize, T> {
    length: usize,
    data: [(usize, T); N],
}

impl<const N: usize, T: Copy + Default> Pairs<N, T> {
    pub(crate) fn new(initial_data: &[(usize, T)]) -> Self {
        assert!(initial_data.len() <= N, "Initial data length exceeds the capacity N.");
        let mut data_array: [(usize, T); N] = [(0, T::default()); N]; 
        for (i, &item) in initial_data.iter().enumerate() {
            data_array[i] = item;
        }

        Pairs {
            length: initial_data.len(),
            data: data_array,
        }
    }
}

#[cfg(test)]
impl<const N: usize, T: Copy + Default> Pairs<N, T> {
    pub fn is_empty(&self) -> bool {
        return self.length == 0
    }

    pub fn len(&self) -> usize {
        return self.length
    }
}
