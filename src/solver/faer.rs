use faer::dyn_stack::MemBuffer;
use faer::dyn_stack::MemStack;
use faer::linalg::lu::partial_pivoting::factor::lu_in_place;
use faer::linalg::lu::partial_pivoting::factor::lu_in_place_scratch;
use faer::linalg::lu::partial_pivoting::factor::PartialPivLuParams;
use faer::linalg::lu::partial_pivoting::solve::solve_in_place;
use faer::linalg::lu::partial_pivoting::solve::solve_in_place_scratch;
use faer::prelude::*;
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
    perm: Vec<usize>,
    perm_inv: Vec<usize>,
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
            perm: vec![0; vars],
            perm_inv: vec![0; vars],
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

    #[cfg(feature = "faer-in-place")]
    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        let params = PartialPivLuParams {
            recursion_threshold: 2,
            blocksize: 2,
            ..faer::Auto::<f64>::auto()
        };
        let lu_memory = lu_in_place_scratch::<usize, Numeric>(
            self.a_mat.nrows(),
            self.a_mat.ncols(),
            Par::Seq,
            params.into(),
        );

        let solve_memory = solve_in_place_scratch::<usize, Numeric>(
            self.a_mat.nrows(),
            self.a_mat.ncols(),
            Par::Seq,
        );

        // allocate the scratch space
        let mut memory = MemBuffer::new(lu_memory.or(solve_memory));
        let stack = MemStack::new(&mut memory);

        let (_, row_perm) = lu_in_place(
            self.a_mat.as_mut(),
            &mut self.perm,
            &mut self.perm_inv,
            Par::Seq,
            stack,
            params.into(),
        );
        self.x_vec_workspace = self.b_vec.to_owned();
        solve_in_place(
            self.l_mat.as_ref(),
            self.u_mat.as_ref(),
            row_perm,
            self.x_vec_workspace.as_mut(),
            Par::Seq,
            stack,
        );

        for (idx, val) in self.x_vec_workspace.col_as_slice(0).iter().enumerate() {
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
    
    #[cfg(not(feature = "faer-in-place"))]
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
