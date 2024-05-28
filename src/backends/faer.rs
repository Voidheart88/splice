#![allow(unused)]

use super::{Backend, BackendError, Col, Row};
use crate::models::{Doubles, Triples};
use faer::sparse::SparseRowMat;
use nalgebra::DVector;

/// A backend implementation using the Faer library.
pub(crate) struct FaerBackend {
    /// The conductance matrix `A` as a sparse matrix.
    a_mat: SparseRowMat<usize, f64>,
    /// The vector `b` as a dense vector.
    b_vec: SparseRowMat<usize, f64>,
    /// The Solution vector `x`.
    x1: DVector<f64>,
}

impl Backend for FaerBackend {
    /// Creates a new instance of the Faer backend with the given number of variables.
    fn new(vars: usize) -> Result<Self, BackendError> {
        let a_mat = SparseRowMat::try_new_from_triplets(vars, vars, &[])?;
        let b_vec = SparseRowMat::try_new_from_triplets(vars, 1, &[])?;
        let x1 = DVector::zeros(vars);

        Ok(Self { a_mat, b_vec, x1 })
    }

    /// Sets the conductance matrix (`a_mat`) into the backend.
    fn set_a(&mut self, a_mat: &Triples) {}

    /// Sets the known values vector (`b_vec`) into the backend.
    fn set_b(&mut self, b_vec: &Doubles) {}

    /// Inserts the conductance matrix (`a_mat`) into the backend.
    fn insert_a(&mut self, a_mat: &Triples) {}

    /// Inserts the known values vector (`b_vec`) into the backend.
    fn insert_b(&mut self, b_vec: &Doubles) {}

    /// Solves the system of equations (Ax = B for x) and returns a reference to the solution.
    fn solve(&mut self) -> Result<&Vec<f64>, BackendError> {
        Err(BackendError::Unimplemented)
    }
}

impl FaerBackend {}

#[cfg(test)]
impl FaerBackend {
    /// Returns the number of rows in the matrix `a_mat`.
    pub fn rows(&self) -> usize {
        self.a_mat.nrows()
    }

    /// Returns the number of columns in the matrix `a_mat`.
    pub fn cols(&self) -> usize {
        self.a_mat.ncols()
    }

    /// Returns the length of the vector `b_vec`.
    pub fn b_vec_len(&self) -> usize {
        self.b_vec.nrows()
    }

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat(&self) -> &SparseRowMat<usize, f64> {
        &self.a_mat
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &SparseRowMat<usize, f64> {
        &self.b_vec
    }
}
