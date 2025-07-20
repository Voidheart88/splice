use super::{Solver, SolverError};
use crate::{models::{Pairs, Triples}, spot::{ComplexNumeric, Numeric}}; // Ensure these are correctly imported
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

    fn set_a(&mut self, a_mat: (usize,usize,Numeric)) {}

    fn set_b(&mut self, b_vec: (usize,Numeric)) {}


    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        let lu = LU::new(self.a_mat.clone());

        self.x_vec = match lu.solve(&self.b_vec) {
            Some(v) => v,
            None => return Err(SolverError::MatrixNonInvertible),
        };

        Ok(&self.x_vec.data.as_vec())
    }

    fn set_cplx_a(&mut self, a_mat: (usize,usize,ComplexNumeric)) {
    }

    fn set_cplx_b(&mut self, b_vec: (usize,ComplexNumeric)) {
    }

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        // Cloning only the necessary matrices for LU decomposition
        let lu = LU::new(self.cplx_a_mat.clone());

        // Solving the equations without unnecessary cloning
        self.cplx_x_vec = match lu.solve(&self.cplx_b_vec) {
            Some(v) => v,
            None => return Err(SolverError::MatrixNonInvertible),
        };

        // Returning a reference to the solution vector
        Ok(&self.cplx_x_vec.data.as_vec())
    }
}


#[cfg(test)]
impl NalgebraSolver {
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
        self.b_vec.len()
    }

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat(
        &self,
    ) -> &na::Matrix<Numeric, na::Dyn, na::Dyn, na::VecStorage<Numeric, na::Dyn, na::Dyn>> {
        &self.a_mat
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &na::Matrix<Numeric, na::Dyn, na::U1, na::VecStorage<Numeric, na::Dyn, na::U1>> {
        &self.b_vec
    }
}
