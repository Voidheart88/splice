use std::collections::HashMap;

use faer::{
    prelude::*,
    sparse::{linalg::LuError, Triplet},
};

use super::{Solver, SolverError};
use crate::{models::{Pairs, Triples}, spot::ComplexNumeric};
use crate::spot::Numeric;

//use faer::solvers::SpSolver;
//use faer::sparse::{LuError, SparseColMat};

/// A backend implementation using the Faer library.
pub(crate) struct FaerSolver {
    /// The conductance matrix `A`.
    a_mat: HashMap<(usize, usize), Numeric>,

    /// The vector `b`.
    b_vec: Mat<Numeric>,

    /// The Solution vector
    x_vec: Vec<Numeric>,

    /// The conductance matrix `A`.
    cplx_a_mat: HashMap<(usize, usize), c64>,

    /// The vector `b`.
    cplx_b_vec: Mat<c64>,

    /// The Solution vector
    cplx_x_vec: Vec<num::Complex<Numeric>>,
}

impl FaerSolver {
    fn set_value(&mut self, row: usize, col: usize, val: Numeric) {
        self.a_mat.insert((row, col), val);
    }
    fn set_cplx_value(&mut self, row: usize, col: usize, val: c64) {
        self.cplx_a_mat.insert((row, col), val);
    }
}

impl Solver for FaerSolver {
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized,
    {
        Ok(FaerSolver {
            a_mat: HashMap::new(),
            b_vec: Mat::full(vars, 1, 0.0),
            x_vec: vec![0.0; vars],
            cplx_a_mat: HashMap::new(),
            cplx_b_vec: Mat::full(vars, 1, c64 { re: 0.0, im: 0.0 }),
            cplx_x_vec: vec![num::Complex { re: 0.0, im: 0.0 }; vars],
        })
    }

    fn set_a(&mut self, a_mat: (usize,usize,Numeric)) {}

    fn set_b(&mut self, b_vec: (usize,Numeric)) {}

    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        let triples: Vec<Triplet<usize, usize, Numeric>> = self
            .a_mat
            .iter()
            .map(|((row, col), val)| Triplet {
                row: *row,
                col: *col,
                val: *val,
            })
            .collect();
        let a_mat =
            SparseColMat::try_new_from_triplets(self.x_vec.len(), self.x_vec.len(), &triples)
                .unwrap();

        let lu = match a_mat.sp_lu() {
            Ok(lu) => lu,
            Err(_) => return Err(SolverError::MatrixNonInvertible),
        };

        let res = lu.solve(&self.b_vec);
        for (idx, val) in res.col_as_slice(0).iter().enumerate() {
            self.x_vec[idx] = *val;
        }

        Ok(&self.x_vec)
    }

    fn set_cplx_a(&mut self, a_mat: (usize,usize,ComplexNumeric)) {
    }

    fn set_cplx_b(&mut self, b_vec: (usize,ComplexNumeric)) {
    }

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        Err(SolverError::MatrixNonInvertible)
    }
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

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat(&self) -> &HashMap<(usize, usize), Numeric> {
        &self.a_mat
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &Mat<Numeric> {
        &self.b_vec
    }
}

impl From<LuError> for SolverError {
    fn from(value: LuError) -> Self {
        match value {
            LuError::Generic(_) => SolverError::MatrixNonInvertible,
            LuError::SymbolicSingular { index: _ } => SolverError::MatrixNonInvertible,
        }
    }
}

fn into_c64(val: ComplexNumeric) -> c64 {
    c64 {
        re: val.re,
        im: val.im,
    }
}

fn into_complex(val: c64) -> ComplexNumeric {
    num::Complex {
        re: val.re,
        im: val.im,
    }
}
