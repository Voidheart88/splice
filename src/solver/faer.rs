use super::{Solver, SolverError};
use crate::models::{Pairs, Triples};
use faer::solvers::SpSolver;
use faer::sparse::{SparseColMat, SymbolicSparseColMat};
use faer::Mat;

/// A backend implementation using the Faer library.
pub(crate) struct FaerSolver {
    /// The conductance matrix `A`.
    a_mat: SparseColMat<usize, f64>,
    /// The vector `b`.
    b_vec: Mat<f64>,
    /// The Solution vector
    x_vec: Vec<f64>,
}

impl Solver for FaerSolver {
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized,
    {
        let a_mat_sym = SymbolicSparseColMat::new_checked(
            vars, 
            vars, 
            Vec::new(), 
            None, 
            Vec::new()
        );
        let a_mat = SparseColMat::new(a_mat_sym, Vec::new());

        Ok(FaerSolver {
            a_mat: a_mat,
            b_vec: Mat::new(),
            x_vec: Vec::new(),
        })
    }

    fn set_a(&mut self, a_mat: &Triples) {
        match a_mat {
            Triples::Empty => {}
            Triples::Single((row, col, val)) => {
                self.a_mat.as_mut()[(row.0,col.0)] = *val;
            }
            Triples::Double(vals) => {
                self.a_mat.as_mut()[(*vals[0].0,*vals[0].1)] = vals[0].2;
                self.a_mat.as_mut()[(*vals[1].0,*vals[1].1)] = vals[1].2;
            }
            Triples::Quad(vals) => {
                self.a_mat.as_mut()[(*vals[0].0,*vals[0].1)] = vals[0].2;
                self.a_mat.as_mut()[(*vals[1].0,*vals[1].1)] = vals[1].2;
                self.a_mat.as_mut()[(*vals[2].0,*vals[2].1)] = vals[2].2;
                self.a_mat.as_mut()[(*vals[3].0,*vals[3].1)] = vals[3].2;
            }
            Triples::Vec(vals) => {
                for triple in vals {
                    self.a_mat.as_mut()[(*triple.0,*triple.1)] = triple.2;
                }
            }
        }
    }

    fn set_b(&mut self, b_vec: &Pairs) {
        match b_vec {
            Pairs::Empty => {},
            Pairs::Single(val) => {
                self.b_vec.as_mut()[(*val.0,0)] = val.1;
            },
            Pairs::Double(vals) => {
                self.b_vec[(*vals[0].0,0)] = vals[0].1;
                self.b_vec[(*vals[1].0,0)] = vals[1].1;
            },
            Pairs::Vec(vals) => {
                self.b_vec[(*vals[0].0,0)] = vals[0].1;
                self.b_vec[(*vals[1].0,0)] = vals[1].1;
                self.b_vec[(*vals[2].0,0)] = vals[2].1;
                self.b_vec[(*vals[3].0,0)] = vals[3].1;
            },
        }
    }

    fn insert_a(&mut self, a_mat: &Triples) {
        match a_mat {
            Triples::Empty => {}
            Triples::Single((row, col, val)) => {
                self.a_mat.as_mut()[(row.0,col.0)] *= *val;
            }
            Triples::Double(vals) => {
                self.a_mat.as_mut()[(*vals[0].0,*vals[0].1)] *= vals[0].2;
                self.a_mat.as_mut()[(*vals[1].0,*vals[1].1)] *= vals[1].2;
            }
            Triples::Quad(vals) => {
                self.a_mat.as_mut()[(*vals[0].0,*vals[0].1)] *= vals[0].2;
                self.a_mat.as_mut()[(*vals[1].0,*vals[1].1)] *= vals[1].2;
                self.a_mat.as_mut()[(*vals[2].0,*vals[2].1)] *= vals[2].2;
                self.a_mat.as_mut()[(*vals[3].0,*vals[3].1)] *= vals[3].2;
            }
            Triples::Vec(vals) => {
                for triple in vals {
                    self.a_mat.as_mut()[(*triple.0,*triple.1)] *= triple.2;
                }
            }
        }
    }

    fn insert_b(&mut self, b_vec: &Pairs) {
        match b_vec {
            Pairs::Empty => {},
            Pairs::Single(val) => {
                self.b_vec.as_mut()[(*val.0,0)] *= val.1;
            },
            Pairs::Double(vals) => {
                self.b_vec[(*vals[0].0,0)] *= vals[0].1;
                self.b_vec[(*vals[1].0,0)] *= vals[1].1;
            },
            Pairs::Vec(vals) => {
                self.b_vec[(*vals[0].0,0)] *= vals[0].1;
                self.b_vec[(*vals[1].0,0)] *= vals[1].1;
                self.b_vec[(*vals[2].0,0)] *= vals[2].1;
                self.b_vec[(*vals[3].0,0)] *= vals[3].1;
            },
        }
    }

    fn solve(&mut self) -> Result<&Vec<f64>, SolverError> {
        // Cloning only the necessary matrices for LU decomposition
        let lu = self.a_mat.sp_lu().unwrap();

        // Solving the equations without unnecessary cloning
        let res = lu.solve(&self.b_vec);
        for (idx,val) in res.col_as_slice(0).iter().enumerate() {
            self.x_vec[idx] = *val;
        };

        // Returning a reference to the solution vector
        Ok(&self.x_vec)
    }
}

#[cfg(test)]
impl FaerSolver {
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
    pub fn a_mat(&self) -> &SparseColMat<usize,f64> {
        &self.a_mat
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &Mat<f64> {
        &self.b_vec
    }
}