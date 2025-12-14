use faer::prelude::*;
use num::Zero;

use super::{Solver, SolverError};
use crate::spot::*;

/// A backend implementation using the Faer library.
///
/// This solver uses dense matrices to store the conductance matrix `A` and the vector `b`.
/// It is suitable for small to medium-sized circuits where the overhead of sparse matrices
/// is not justified.
#[derive(Debug)]
pub struct FaerSolver {
    /// The conductance matrix `A` as a dense matrix.
    /// This matrix represents the linear part of the circuit equations.
    a_mat: Mat<Numeric>,

    /// The vector `b` as a dense vector.
    /// This vector represents the known values in the circuit equations.
    b_vec: Mat<Numeric>,

    /// The Solution vector `x`.
    /// This vector stores the solution to the system of equations `Ax = b`.
    x_vec: Vec<Numeric>,

    /// The conductance matrix `A` for complex numbers.
    /// This matrix is used for AC analysis where complex numbers are required.
    cplx_a_mat: Mat<c64>,

    /// The vector `b` for complex numbers.
    /// This vector is used for AC analysis where complex numbers are required.
    cplx_b_vec: Mat<c64>,

    /// The Solution vector `x` for complex numbers.
    /// This vector stores the solution to the system of equations `Ax = b` for complex numbers.
    cplx_x_vec: Vec<ComplexNumeric>,
}

impl Solver for FaerSolver {
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized,
    {
        // Creates a new `FaerSolver` with the given number of variables.
        //
        // # Arguments
        //
        // * `vars` - The number of variables in the system of equations.
        //
        // # Returns
        //
        // A new `FaerSolver` instance.
        Ok(FaerSolver {
            a_mat: Mat::zeros(vars, vars),
            b_vec: Mat::full(vars, 1, 0.0),
            x_vec: vec![0.0; vars],
            cplx_a_mat: Mat::zeros(vars, vars),
            cplx_b_vec: Mat::full(vars, 1, c64 { re: 0.0, im: 0.0 }),
            cplx_x_vec: vec![num::Complex { re: 0.0, im: 0.0 }; vars],
        })
    }

    fn insert_a(&mut self, a_mat: &(usize, usize, Numeric)) {
        // Inserts a value into the conductance matrix `A`.
        //
        // # Arguments
        //
        // * `a_mat` - A tuple containing the row index, column index, and value to insert.
        let (row, col, val) = *a_mat;
        self.a_mat[(row, col)] += val;
    }

    fn insert_b(&mut self, b_vec: &(usize, Numeric)) {
        // Inserts a value into the vector `b`.
        //
        // # Arguments
        //
        // * `b_vec` - A tuple containing the row index and value to insert.
        let (row, val) = *b_vec;
        let value = self.b_vec.get_mut(row, 0);
        *value += val;
    }

    fn insert_cplx_a(&mut self, a_mat: &(usize, usize, ComplexNumeric)) {
        // Inserts a complex value into the conductance matrix `A`.
        //
        // # Arguments
        //
        // * `a_mat` - A tuple containing the row index, column index, and complex value to insert.
        let (row, col, val) = *a_mat;
        self.cplx_a_mat[(row, col)] += val;
    }

    fn insert_cplx_b(&mut self, b_vec: &(usize, ComplexNumeric)) {
        // Inserts a complex value into the vector `b`.
        //
        // # Arguments
        //
        // * `b_vec` - A tuple containing the row index and complex value to insert.
        let (row, val) = *b_vec;
        let value = self.cplx_b_vec.get_mut(row, 0);
        *value += val;
    }
    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        // Solves the system of equations `Ax = b` for real numbers.
        //
        // This method uses LU decomposition with partial pivoting to solve the system.
        // After solving, the matrices and vectors are reset to ensure a clean state for the next solve.
        //
        // # Returns
        //
        // A reference to the solution vector `x`.
        //
        // # Errors
        //
        // Returns `SolverError::MatrixNonInvertible` if the matrix is singular and cannot be inverted.
        let lu = self.a_mat.partial_piv_lu();
        let res = lu.solve(&self.b_vec);

        for (idx, val) in res.col_as_slice(0).iter().enumerate() {
            self.x_vec[idx] = *val;
        }

        self.a_mat
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = Numeric::zero());
        self.b_vec
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = Numeric::zero());

        Ok(&self.x_vec)
    }

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        // Solves the system of equations `Ax = b` for complex numbers.
        //
        // This method uses LU decomposition with partial pivoting to solve the system.
        // After solving, the matrices and vectors are reset to ensure a clean state for the next solve.
        //
        // # Returns
        //
        // A reference to the solution vector `x` for complex numbers.
        //
        // # Errors
        //
        // Returns `SolverError::MatrixNonInvertible` if the matrix is singular and cannot be inverted.
        let lu = self.cplx_a_mat.partial_piv_lu();
        let res = lu.solve(&self.cplx_b_vec);

        for (idx, val) in res.col_as_slice(0).iter().enumerate() {
            self.cplx_x_vec[idx] = *val;
        }

        self.cplx_a_mat
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = ComplexNumeric::zero());
        self.cplx_b_vec
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = ComplexNumeric::zero());

        Ok(&self.cplx_x_vec)
    }

    fn init(&mut self, _a_matrix: Vec<(usize, usize)>, _cplx_a_matrix: Vec<(usize, usize)>) {}

    fn reset(&mut self) {
        // Resets the solver to a clean state by zeroing all matrices and vectors.
        //
        // This method is called between simulations to ensure that no old values
        // affect the new calculations. It is particularly important for transient
        // simulations where the matrices and vectors are rebuilt in each time step.
        self.a_mat
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = Numeric::zero());
        self.b_vec
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = Numeric::zero());
        self.cplx_a_mat
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = ComplexNumeric::zero());
        self.cplx_b_vec
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = ComplexNumeric::zero());
    }
}

#[cfg(test)]
impl FaerSolver {
    /// Returns the number of rows in the matrix `a_mat`.
    ///
    /// This is used for testing purposes to verify the dimensions of the matrix.
    pub fn rows(&self) -> usize {
        self.x_vec.len()
    }

    /// Returns the number of columns in the matrix `a_mat`.
    ///
    /// This is used for testing purposes to verify the dimensions of the matrix.
    pub fn cols(&self) -> usize {
        self.x_vec.len()
    }

    /// Returns the length of the vector `b_vec`.
    ///
    /// This is used for testing purposes to verify the dimensions of the vector.
    pub fn b_vec_len(&self) -> usize {
        self.b_vec.nrows()
    }
}
