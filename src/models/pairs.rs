use std::ops::{Index, IndexMut};

/// A structure representing pairs of an element.
///
/// Each pair consists of a row and a value. The struct has a compile-time
/// fixed capacity `N`, but `length` tracks the actual number of valid elements currently stored.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Pairs<T, const N: usize> {
    length: usize,
    data: [(usize, T); N],
}

impl<T: Copy + Default, const N: usize> Pairs<T, N> {
    pub(crate) fn new(initial_data: &[(usize, T)]) -> Self {
        assert!(
            initial_data.len() <= N,
            "Initial data length exceeds the capacity N."
        );
        let mut data_array: [(usize, T); N] = [(0, T::default()); N];
        for (i, &item) in initial_data.iter().enumerate() {
            data_array[i] = item;
        }

        Self {
            length: initial_data.len(),
            data: data_array,
        }
    }

    pub(crate) fn data(&self) -> [(usize, T); N] {
        self.data
    }
}

impl<T: Copy + Default, const N: usize> Index<usize> for Pairs<T, N> {
    type Output = (usize, T);

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!(
                "Index {} out of bounds for Pairs with length {}",
                index, self.length
            );
        }
        &self.data[index]
    }
}

impl<T: Copy + Default, const N: usize> IndexMut<usize> for Pairs<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.length {
            panic!(
                "Index {} out of bounds for Pairs with length {}",
                index, self.length
            );
        }
        &mut self.data[index]
    }
}

pub(crate) struct PairsIter<'a, T, const N: usize> {
    pairs: &'a Pairs<T, N>,
    current: usize,
}

impl<'a, T, const N: usize> Iterator for PairsIter<'a, T, N> {
    type Item = &'a (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.pairs.length {
            let item = &self.pairs.data[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.pairs.length - self.current;
        (remaining, Some(remaining))
    }
}

impl<'a, T, const N: usize> ExactSizeIterator for PairsIter<'a, T, N> {
    fn len(&self) -> usize {
        self.pairs.length - self.current
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Pairs<T, N> {
    type Item = &'a (usize, T);
    type IntoIter = PairsIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        PairsIter {
            pairs: self,
            current: 0,
        }
    }
}

/// A structure representing pair index of an element.
///
/// Each pair consists of a row and a col. The struct has a compile-time
/// fixed capacity `N`, but `length` tracks the actual number of valid elements currently stored.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct PairIdx<const N: usize> {
    length: usize,
    data: [usize; N],
}

impl<const N: usize> PairIdx<N> {
    pub(crate) fn new(initial_data: &[usize]) -> Self {
        assert!(
            initial_data.len() <= N,
            "Initial data length exceeds the capacity N."
        );
        let mut data_array: [usize; N] = [0; N];
        for (i, &item) in initial_data.iter().enumerate() {
            data_array[i] = item;
        }

        Self {
            length: initial_data.len(),
            data: data_array,
        }
    }

    pub(crate) fn data(&self) -> [usize; N] {
        self.data
    }
}

#[cfg(test)]
impl<T, const N: usize> Pairs<T, N> {
    pub fn len(&self) -> usize {
        return self.length;
    }
}
