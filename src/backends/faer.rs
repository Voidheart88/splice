use super::{Backend, BackendError};
use crate::models::{Doubles, Triples};
use faer::sparse::SparseColMat;


/// A backend implementation using the Faer library.
pub(crate) struct FaerBackend {
    /// The conductance matrix `A`.
    a_mat: SparseColMat<usize,f64>,
    /// The vector `b`.
    b_vec: SparseColMat<usize,f64>,
    /// The Solution vector
    x_vec: SparseColMat<usize,f64>,

}

impl Backend for FaerBackend {
    fn new(vars: usize) -> Result<Self, BackendError>
    where
        Self: Sized {
        todo!()
    }

    fn set_a(&mut self, a_mat: &Triples) {
        todo!()
    }

    fn set_b(&mut self, b_vec: &Doubles) {
        todo!()
    }

    fn insert_a(&mut self, a_mat: &Triples) {
        todo!()
    }

    fn insert_b(&mut self, b_vec: &Doubles) {
        todo!()
    }

    fn solve(&mut self) -> Result<&Vec<f64>, BackendError> {
        todo!()
    }
}