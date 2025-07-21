use std::ops::{Index, IndexMut};

/// A structure representing triples of an element.
///
/// Each pair consists of a row and a value. The struct has a compile-time
/// fixed capacity `N`, but `length` tracks the actual number of valid elements currently stored.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Triples<T, const N: usize> {
    length: usize,
    data: [(usize, usize, T); N],
}

impl<T: Copy + Default, const N: usize> Triples<T, N> {
    pub(crate) fn new(initial_data: &[(usize, usize, T)]) -> Self {
        assert!(
            initial_data.len() <= N,
            "Initial data length exceeds the capacity N."
        );
        let mut data_array: [(usize, usize, T); N] = [(0, 0, T::default()); N];
        for (i, &item) in initial_data.iter().enumerate() {
            data_array[i] = item;
        }

        Self {
            length: initial_data.len(),
            data: data_array,
        }
    }

    pub(crate) fn iter(&self) -> TriplesIter<'_, T, N> {
        self.into_iter()
    }
    
    pub(crate) fn data(&self) -> [(usize, usize, T); N] {
        self.data
    }
}

impl<T: Copy + Default, const N: usize> Index<usize> for Triples<T, N> {
    type Output = (usize, usize, T);

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!(
                "Index {} out of bounds for Triples with length {}",
                index, self.length
            );
        }
        &self.data[index]
    }
}

impl<T: Copy + Default, const N: usize> IndexMut<usize> for Triples<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.length {
            panic!(
                "Index {} out of bounds for Triples with length {}",
                index, self.length
            );
        }
        &mut self.data[index]
    }
}

pub(crate) struct TriplesIter<'a, T, const N: usize> {
    triples: &'a Triples<T, N>,
    current: usize,
}

impl<'a, T, const N: usize> Iterator for TriplesIter<'a, T, N> {
    type Item = &'a (usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.triples.length {
            let item = &self.triples.data[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.triples.length - self.current;
        (remaining, Some(remaining))
    }
}

impl<'a, T, const N: usize> ExactSizeIterator for TriplesIter<'a, T, N> {
    fn len(&self) -> usize {
        self.triples.length - self.current
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Triples<T, N> {
    type Item = &'a (usize, usize, T);
    type IntoIter = TriplesIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        TriplesIter {
            triples: self,
            current: 0,
        }
    }
}

#[cfg(test)]
impl<T, const N: usize> Triples<T, N> {
    pub fn is_empty(&self) -> bool {
        return self.length == 0;
    }

    pub fn len(&self) -> usize {
        return self.length;
    }

}
