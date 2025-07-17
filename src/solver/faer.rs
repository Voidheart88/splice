use std::collections::HashMap;

use faer::{prelude::*, sparse::{linalg::LuError, Triplet}};

use super::{Solver, SolverError};
use crate::models::{ComplexPairs, ComplexTriples, Pairs, Triples};

//use faer::solvers::SpSolver;
//use faer::sparse::{LuError, SparseColMat};

/// A backend implementation using the Faer library.
pub(crate) struct FaerSolver {
    /// The conductance matrix `A`.
    a_mat: HashMap<(usize, usize), f64>,

    /// The vector `b`.
    b_vec: Mat<f64>,

    /// The Solution vector
    x_vec: Vec<f64>,

    /// The conductance matrix `A`.
    cplx_a_mat: HashMap<(usize, usize), c64>,

    /// The vector `b`.
    cplx_b_vec: Mat<c64>,

    /// The Solution vector
    cplx_x_vec: Vec<num::Complex<f64>>,
}

impl FaerSolver {
    fn set_value(&mut self, row: usize, col: usize, val: f64) {
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

    fn set_a(&mut self, a_mat: &Triples) {
        match a_mat {
            Triples::Empty => {}
            Triples::Single((row, col, val)) => {
                self.set_value(*row, *col, *val);
            }
            Triples::Double(vals) => {
                self.set_value(vals[0].0, vals[0].1, vals[0].2);
                self.set_value(vals[1].0, vals[1].1, vals[1].2);
            }
            Triples::Quad(vals) => {
                self.set_value(vals[0].0, vals[0].1, vals[0].2);
                self.set_value(vals[1].0, vals[1].1, vals[1].2);
                self.set_value(vals[2].0, vals[2].1, vals[2].2);
                self.set_value(vals[2].0, vals[3].1, vals[3].2);
            }
            Triples::Vec(vec_triples) => {
                for (row, col, val) in vec_triples {
                    self.set_value(*row, *col, *val);
                }
            }
        }
    }

    fn set_b(&mut self, b_vec: &Pairs) {
        match b_vec {
            Pairs::Empty => {}
            Pairs::Single(val) => {
                self.b_vec[(val.0, 0)] = val.1;
            }
            Pairs::Double(vals) => {
                self.b_vec[(vals[0].0, 0)] = vals[0].1;
                self.b_vec[(vals[1].0, 0)] = vals[1].1;
            }
            Pairs::Vec(vec_pairs) => {
                for (col, val) in vec_pairs {
                    self.b_vec[(*col, 0)] = *val;
                }
            }
        }
    }

    fn solve(&mut self) -> Result<&Vec<f64>, SolverError> {
        let triples: Vec<Triplet<usize, usize, f64>> = self
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

    fn set_cplx_a(&mut self, a_mat: &crate::models::ComplexTriples) {
        match a_mat {
            ComplexTriples::Empty => {}
            ComplexTriples::Single((row, col, val)) => {
                self.set_cplx_value(*row, *col, into_c64(*val));
            }
            ComplexTriples::Double(vals) => {
                self.set_cplx_value(vals[0].0, vals[0].1, into_c64(vals[0].2));
                self.set_cplx_value(vals[1].0, vals[1].1, into_c64(vals[1].2));
            }
            ComplexTriples::Quad(vals) => {
                self.set_cplx_value(vals[0].0, vals[0].1, into_c64(vals[0].2));
                self.set_cplx_value(vals[1].0, vals[1].1, into_c64(vals[1].2));
                self.set_cplx_value(vals[2].0, vals[2].1, into_c64(vals[2].2));
                self.set_cplx_value(vals[3].0, vals[3].1, into_c64(vals[3].2)); // Corrected: was vals[2].0
            }
            ComplexTriples::Vec(vec_triples) => {
                for (row, col, val) in vec_triples {
                    self.set_cplx_value(*row, *col, into_c64(*val));
                }
            }
        }
    }

    fn set_cplx_b(&mut self, b_vec: &crate::models::ComplexPairs) {
        match b_vec {
            ComplexPairs::Empty => {}
            ComplexPairs::Single(val) => {
                self.cplx_b_vec[(val.0, 0)] = into_c64(val.1);
            }
            ComplexPairs::Double(vals) => {
                self.cplx_b_vec[(vals[0].0, 0)] = into_c64(vals[0].1);
                self.cplx_b_vec[(vals[1].0, 0)] = into_c64(vals[1].1);
            }
            ComplexPairs::Vec(_) => todo!(),
        }
    }

    fn solve_cplx(&mut self) -> Result<&Vec<num::Complex<f64>>, SolverError> {
        let triples: Vec<Triplet<usize, usize, c64>> = self // <--- Type for Vec changed
            .cplx_a_mat
            .iter()
            .map(|((row, col), val)| {
                // Create a Triplet struct instance
                Triplet {
                    row: *row,
                    col: *col,
                    val: *val, // `val` is already `c64` here
                }
            })
            .collect();

        let cplx_a_mat = SparseColMat::try_new_from_triplets(
            self.cplx_x_vec.len(),
            self.cplx_x_vec.len(),
            &triples, // <--- Now `triples` is `&[Triplet<usize, usize, c64>]`
        )
        .unwrap();

        let lu = match cplx_a_mat.sp_lu() {
            Ok(lu) => lu,
            Err(_) => return Err(SolverError::MatrixNonInvertible),
        };

        let res = lu.solve(&self.cplx_b_vec);
        for (idx, val) in res.col_as_slice(0).iter().enumerate() {
            self.cplx_x_vec[idx] = into_complex(*val);
        }

        Ok(&self.cplx_x_vec)
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
    pub fn a_mat(&self) -> &HashMap<(usize, usize), f64> {
        &self.a_mat
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &Mat<f64> {
        &self.b_vec
    }
}

impl From<LuError> for SolverError {
    fn from(value: LuError) -> Self {
        match value {
            LuError::Generic(_) => SolverError::MatrixNonInvertible,
            LuError::SymbolicSingular { index:_ } => SolverError::MatrixNonInvertible,
        }
    }
}

fn into_c64(val: num::Complex<f64>) -> c64 {
    c64 {
        re: val.re,
        im: val.im,
    }
}

fn into_complex(val: c64) -> num::Complex<f64> {
    num::Complex {
        re: val.re,
        im: val.im,
    }
}
