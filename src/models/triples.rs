use std::ops::{Index, IndexMut};

/// A structure representing triples of an element.
///
/// Each triple consists of a row a col and a value. The struct has a compile-time
/// fixed capacity `N`, but `length` tracks the actual number of valid elements currently stored.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Triples<T, const N: usize> {
    length: usize,
    data: [(usize, usize, T); N],
}

impl<T: Copy + Default, const N: usize> Triples<T, N> {
    pub fn new(initial_data: &[(usize, usize, T)]) -> Self {
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

pub struct TriplesIter<'a, T, const N: usize> {
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

impl<T, const N: usize> ExactSizeIterator for TriplesIter<'_, T, N> {
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

/// A structure representing triple index of an element.
///
/// Each triple consists of a row and a col. The struct has a compile-time
/// fixed capacity `N`, but `length` tracks the actual number of valid elements currently stored.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TripleIdx<const N: usize> {
    length: usize,
    data: [(usize, usize); N],
}

impl<const N: usize> TripleIdx<N> {
    pub fn new(initial_data: &[(usize, usize)]) -> Self {
        assert!(
            initial_data.len() <= N,
            "Initial data length exceeds the capacity N."
        );
        let mut data_array: [(usize, usize); N] = [(0, 0); N];
        for (i, &item) in initial_data.iter().enumerate() {
            data_array[i] = item;
        }

        Self {
            length: initial_data.len(),
            data: data_array,
        }
    }

    pub fn data(&self) -> [(usize, usize); N] {
        self.data
    }
}

#[cfg(test)]
impl<T, const N: usize> Triples<T, N> {
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[cfg(test)]
impl<const N: usize> TripleIdx<N> {
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}
