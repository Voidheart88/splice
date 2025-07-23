use super::{Solver, SolverError};
use crate::spot::{ComplexNumeric, Numeric}; // Ensure these are correctly imported
use na::LU;
use nalgebra as na;

/// A Solver implementation using the Nalgebra library.
pub(crate) struct NalgebraSolver {
    /// The conductance matrix `A`.
    a_mat: na::DMatrix<Numeric>,
    /// The vector `b`.
    b_vec: na::DVector<Numeric>,
    /// The Solution vector
    x_vec: na::DVector<Numeric>,
    /// The conductance matrix `A`.
    cplx_a_mat: na::DMatrix<ComplexNumeric>,
    /// The vector `b`.
    cplx_b_vec: na::DVector<ComplexNumeric>,
    /// The Solution vector
    cplx_x_vec: na::DVector<ComplexNumeric>,
}

impl Solver for NalgebraSolver {
    /// Creates a new instance of the Nalgebra Solver with the given number of variables.
    fn new(vars: usize) -> Result<NalgebraSolver, SolverError> {
        let a = na::DMatrix::zeros(vars, vars);
        let b = na::DVector::zeros(vars);
        let x = na::DVector::zeros(vars);

        let cplx_a = na::DMatrix::zeros(vars, vars);
        let cplx_b = na::DVector::zeros(vars);
        let cplx_x = na::DVector::zeros(vars);

        Ok(Self {
            a_mat: a,
            b_vec: b,
            x_vec: x,
            cplx_a_mat: cplx_a,
            cplx_b_vec: cplx_b,
            cplx_x_vec: cplx_x,
        })
    }

    fn insert_a(&mut self, a_mat: &(usize, usize, Numeric)) {
        let (row, col, val) = *a_mat;
        self.a_mat[(row, col)] += val;
    }

    fn insert_b(&mut self, b_vec: &(usize, Numeric)) {
        let (row, val) = *b_vec;
        self.b_vec[row] += val;
    }

    fn insert_cplx_a(&mut self, a_mat: &(usize, usize, ComplexNumeric)) {
        let (row, col, val) = *a_mat;
        self.cplx_a_mat[(row, col)] += val;
    }

    fn insert_cplx_b(&mut self, b_vec: &(usize, ComplexNumeric)) {
        let (row, val) = *b_vec;
        self.cplx_b_vec[row] += val;
    }

    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        let lu = LU::new(self.a_mat.clone());

        self.x_vec = match lu.solve(&self.b_vec) {
            Some(v) => v,
            None => return Err(SolverError::MatrixNonInvertible),
        };

        Ok(&self.x_vec.data.as_vec())
    }

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        let lu = LU::new(self.cplx_a_mat.clone());

        self.cplx_x_vec = match lu.solve(&self.cplx_b_vec) {
            Some(v) => v,
            None => return Err(SolverError::MatrixNonInvertible),
        };

        Ok(&self.cplx_x_vec.data.as_vec())
    }
}

#[cfg(test)]
impl NalgebraSolver {
    pub fn rows(&self) -> usize {
        self.a_mat.nrows()
    }

    pub fn cols(&self) -> usize {
        self.a_mat.ncols()
    }

    pub fn b_vec_len(&self) -> usize {
        self.b_vec.len()
    }
}
