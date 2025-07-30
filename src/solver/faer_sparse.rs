use faer::prelude::*;
use faer::sparse::linalg::LuError;
use faer::sparse::{SymbolicSparseColMat, Triplet};
use num::Zero;

use super::{Solver, SolverError};
use crate::spot::*;

/// A backend implementation using the Faer library.
pub struct FaerSparseSolver {
    /// The conductance matrix `A`.
    a_mat: Vec<Triplet<usize, usize, Numeric>>,

    /// The vector `b`.
    b_vec: Mat<Numeric>,

    /// The Solution vector
    x_vec: Vec<Numeric>,

    /// The conductance matrix `A`.
    cplx_a_mat: Vec<Triplet<usize, usize, ComplexNumeric>>,

    /// The vector `b`.
    cplx_b_vec: Mat<c64>,

    /// The Solution vector
    cplx_x_vec: Vec<ComplexNumeric>,

    // Workspace
    symb: Option<SymbolicSparseColMat<usize, usize, usize>>,
    argsort: Option<faer::sparse::Argsort<usize>>,
    cplx_symb: Option<SymbolicSparseColMat<usize, usize, usize>>,
    cplx_argsort: Option<faer::sparse::Argsort<usize>>,
}

impl Solver for FaerSparseSolver {
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized,
    {
        Ok(FaerSparseSolver {
            a_mat: Vec::new(),
            b_vec: Mat::full(vars, 1, 0.0),
            x_vec: vec![0.0; vars],
            cplx_a_mat: Vec::new(),
            cplx_b_vec: Mat::full(vars, 1, c64 { re: 0.0, im: 0.0 }),
            cplx_x_vec: vec![num::Complex { re: 0.0, im: 0.0 }; vars],
            symb: None,
            argsort: None,
            cplx_symb: None,
            cplx_argsort: None,
        })
    }

    fn insert_a(&mut self, a_mat: &(usize, usize, Numeric)) {
        let (row, col, val) = *a_mat;
        self.a_mat.push(Triplet::new(row, col, val));
    }

    fn insert_b(&mut self, b_vec: &(usize, Numeric)) {
        let (row, val) = *b_vec;
        let value = self.b_vec.get_mut(row, 0);
        *value = *value + val;
    }

    fn insert_cplx_a(&mut self, a_mat: &(usize, usize, ComplexNumeric)) {
        let (row, col, val) = *a_mat;
        self.cplx_a_mat.push(Triplet::new(row, col, val));
    }

    fn insert_cplx_b(&mut self, b_vec: &(usize, ComplexNumeric)) {
        let (row, val) = *b_vec;
        let value = self.cplx_b_vec.get_mut(row, 0);
        *value = *value + val;
    }

    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        let a_mat =
            SparseColMat::try_new_from_triplets(self.x_vec.len(), self.x_vec.len(), &self.a_mat)
                .unwrap();

        let lu = match a_mat.sp_lu() {
            Ok(lu) => lu,
            Err(_) => return Err(SolverError::MatrixNonInvertible),
        };

        let res = lu.solve(&self.b_vec);
        for (idx, val) in res.col_as_slice(0).iter().enumerate() {
            self.x_vec[idx] = *val;
        }

        self.a_mat.clear();

        for idx in 0..self.x_vec.len() {
            let value = self.b_vec.get_mut(idx, 0);
            *value = Numeric::zero();
        }
        Ok(&self.x_vec)
    }

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        let a_mat = SparseColMat::try_new_from_triplets(
            self.x_vec.len(),
            self.x_vec.len(),
            &self.cplx_a_mat,
        )
        .unwrap();

        let lu = match a_mat.sp_lu() {
            Ok(lu) => lu,
            Err(_) => return Err(SolverError::MatrixNonInvertible),
        };

        let res = lu.solve(&self.cplx_b_vec);
        for (idx, val) in res.col_as_slice(0).iter().enumerate() {
            self.cplx_x_vec[idx] = *val;
        }

        Ok(&self.cplx_x_vec)
    }

    fn init(&mut self, a_matrix: Vec<(usize, usize)>, cplx_a_matrix: Vec<(usize, usize)>) {
        let pairs: Vec<faer::sparse::Pair<usize, usize>> = a_matrix
            .iter()
            .map(|(row, col)| (faer::sparse::Pair::new(*row, *col)))
            .collect();

        let (symb, argsort) =
            SymbolicSparseColMat::try_new_from_indices(a_matrix.len(), a_matrix.len(), &pairs)
                .unwrap();

        self.symb = Some(symb);
        self.argsort = Some(argsort);

        let pairs: Vec<faer::sparse::Pair<usize, usize>> = cplx_a_matrix
            .iter()
            .map(|(row, col)| (faer::sparse::Pair::new(*row, *col)))
            .collect();

        let (symb, argsort) =
            SymbolicSparseColMat::try_new_from_indices(a_matrix.len(), a_matrix.len(), &pairs)
                .unwrap();

        self.cplx_symb = Some(symb);
        self.cplx_argsort = Some(argsort);
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

#[cfg(test)]
impl FaerSparseSolver {
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
