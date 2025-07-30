use faer::dyn_stack::MemBuffer;
use faer::dyn_stack::MemStack;
use faer::linalg::lu;
use faer::linalg::lu::partial_pivoting::factor::lu_in_place;
use faer::linalg::lu::partial_pivoting::factor::lu_in_place_scratch;
use faer::linalg::lu::partial_pivoting::factor::PartialPivLuParams;
use faer::linalg::lu::partial_pivoting::solve;
use faer::linalg::lu::partial_pivoting::solve::solve_in_place;
use faer::prelude::*;
use faer::Auto;
use faer::Spec;
use num::Zero;

use super::{Solver, SolverError};
use crate::spot::*;

/// A backend implementation using the Faer library.
pub struct FaerSolver {
    /// The conductance matrix `A`.
    a_mat: Mat<Numeric>,

    /// The vector `b`.
    b_vec: Mat<Numeric>,

    /// The Solution vector
    x_vec: Vec<Numeric>,

    /// The conductance matrix `A`.
    cplx_a_mat: Mat<c64>,

    /// The vector `b`.
    cplx_b_vec: Mat<c64>,

    /// The Solution vector
    cplx_x_vec: Vec<ComplexNumeric>,
    // Workspace
    l_mat: Mat<Numeric>,
    u_mat: Mat<Numeric>,
    row_perm_fwd: Vec<usize>,
    row_perm_bwd: Vec<usize>,
    x_vec_workspace: Mat<Numeric>,
}

impl Solver for FaerSolver {
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized,
    {
        Ok(FaerSolver {
            a_mat: Mat::zeros(vars, vars),
            b_vec: Mat::full(vars, 1, 0.0),
            x_vec: vec![0.0; vars],
            cplx_a_mat: Mat::zeros(vars, vars),
            cplx_b_vec: Mat::full(vars, 1, c64 { re: 0.0, im: 0.0 }),
            cplx_x_vec: vec![num::Complex { re: 0.0, im: 0.0 }; vars],
            l_mat: Mat::zeros(vars, vars),
            u_mat: Mat::zeros(vars, vars),
            row_perm_fwd: vec![0; vars],
            row_perm_bwd: vec![0; vars],
            x_vec_workspace: Mat::full(vars, 1, 0.0),
        })
    }

    fn insert_a(&mut self, a_mat: &(usize, usize, Numeric)) {
        let (row, col, val) = *a_mat;
        self.a_mat[(row, col)] += val;
    }

    fn insert_b(&mut self, b_vec: &(usize, Numeric)) {
        let (row, val) = *b_vec;
        let value = self.b_vec.get_mut(row, 0);
        *value = *value + val;
    }

    fn insert_cplx_a(&mut self, a_mat: &(usize, usize, ComplexNumeric)) {
        let (row, col, val) = *a_mat;
        self.cplx_a_mat[(row, col)] += val;
    }

    fn insert_cplx_b(&mut self, b_vec: &(usize, ComplexNumeric)) {
        let (row, val) = *b_vec;
        let value = self.cplx_b_vec.get_mut(row, 0);
        *value = *value + val;
    }

    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        
        let params  = PartialPivLuParams::auto(); 
        let spec = Spec::new(params);
        let lu_memory = lu_in_place_scratch::<usize,Numeric>(
            self.a_mat.nrows(), 
            self.a_mat.ncols(), 
            Par::Seq, 
            spec,
        );
        
        
        
        
        for (idx, val) in self.b_vec.col_as_slice(0).iter().enumerate() {
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

        println!("{:?}",self.b_vec);
        
        Ok(&self.x_vec)
    }
    /*
    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
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
    */
    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
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
}

#[cfg(test)]
impl FaerSolver {
    /// Returns the number of rows in the matrix `a_mat`.
    pub fn rows(&self) -> usize {
        self.x_vec.len()
    }

    /// Returns the number of columns in the matrix `a_mat`.
    pub fn cols(&self) -> usize {
        self.x_vec.len()
    }

    /// Returns the length of the vector `b_vec`.
    pub fn b_vec_len(&self) -> usize {
        self.b_vec.nrows()
    }
}
