use faer::prelude::*;
use faer::sparse::linalg::LuError;
use faer::sparse::SparseColMat;
use faer::sparse::Triplet;
use num::Zero;

use super::{Solver, SolverError};
use crate::spot::*;

/// A Solver implementation using the Faer library with sparse matrices.
///
/// This solver uses sparse matrices from the Faer library to store the conductance matrix `A`
/// and the vector `b`. It is suitable for medium to large circuits where memory efficiency
/// and performance are critical. The sparse format only stores non-zero elements,
/// making it more efficient for circuits with many nodes but sparse connectivity.
///
/// The solver uses the Compressed Sparse Column (CSC) format, which is efficient for
/// matrix-vector operations and LU decomposition. It supports both real-valued (DC/OP/Transient)
/// and complex-valued (AC) analysis through separate sparse matrices for each type.
///
/// For real-valued analysis, it uses `SparseColMat<Numeric>` for the conductance matrix.
/// For complex-valued analysis, it uses `SparseColMat<ComplexNumeric>`.
///
/// The solver performs symbolic analysis and LU decomposition for solving linear systems
/// and is optimized for performance-critical applications.
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
=======
/// A Solver implementation using the Faer library with sparse matrices.
///
/// This solver uses sparse matrices from the Faer library to store the conductance matrix `A`
/// and the vector `b`. It is suitable for medium to large circuits where memory efficiency
/// and performance are critical. The sparse format only stores non-zero elements,
/// making it more efficient for circuits with many nodes but sparse connectivity.
///
/// The solver uses the Compressed Sparse Column (CSC) format, which is efficient for
/// matrix-vector operations and LU decomposition. It supports both real-valued (DC/OP/Transient)
/// and complex-valued (AC) analysis through separate sparse matrices for each type.
///
/// For real-valued analysis, it uses `SparseColMat<Numeric>` for the conductance matrix.
/// For complex-valued analysis, it uses `SparseColMat<ComplexNumeric>`.
///
/// The solver performs symbolic analysis and LU decomposition for solving linear systems
/// and is optimized for performance-critical applications.
=======
/// A Solver implementation using the Faer library with sparse matrices.
///
/// This solver uses sparse matrices from the Faer library to store the conductance matrix `A`
/// and the vector `b`. It is suitable for medium to large circuits where memory efficiency
/// and performance are critical. The sparse format only stores non-zero elements,
/// making it more efficient for circuits with many nodes but sparse connectivity.
///
/// The solver uses the Compressed Sparse Column (CSC) format, which is efficient for
/// matrix-vector operations and LU decomposition. It supports both real-valued (DC/OP/Transient)
/// and complex-valued (AC) analysis through separate sparse matrices for each type.
///
/// For real-valued analysis, it uses `SparseColMat<Numeric>` for the conductance matrix.
/// For complex-valued analysis, it uses `SparseColMat<ComplexNumeric>`.
///
/// The solver performs symbolic analysis and LU decomposition for solving linear systems
/// and is optimized for performance-critical applications.
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
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
        })
    }

    fn insert_a(&mut self, a_mat: &(usize, usize, Numeric)) {
        let (row, col, val) = *a_mat;
        self.a_mat.push(Triplet::new(row, col, val));
    }

    fn insert_b(&mut self, b_vec: &(usize, Numeric)) {
        let (row, val) = *b_vec;
        let value = self.b_vec.get_mut(row, 0);
        *value += val;
    }

    fn insert_cplx_a(&mut self, a_mat: &(usize, usize, ComplexNumeric)) {
        let (row, col, val) = *a_mat;
        self.cplx_a_mat.push(Triplet::new(row, col, val));
    }

    fn insert_cplx_b(&mut self, b_vec: &(usize, ComplexNumeric)) {
        let (row, val) = *b_vec;
        let value = self.cplx_b_vec.get_mut(row, 0);
        *value += val;
    }

    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        let a_mat =
            SparseColMat::try_new_from_triplets(self.x_vec.len(), self.x_vec.len(), &self.a_mat)
                .expect("Failed to create sparse matrix from triplets. This indicates invalid matrix dimensions or data.");

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
        .expect("Failed to create complex sparse matrix from triplets. This indicates invalid matrix dimensions or data.");

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

    fn init(&mut self, _a_matrix: Vec<(usize, usize)>, _cplx_a_matrix: Vec<(usize, usize)>) {}

    fn reset(&mut self) {
        self.a_mat.clear();
        self.b_vec
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = Numeric::zero());
        self.cplx_a_mat.clear();
        self.cplx_b_vec
            .row_iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|val| *val = ComplexNumeric::zero());
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
