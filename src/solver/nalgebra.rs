use super::{Solver, SolverError};
use crate::models::{ComplexPairs, ComplexTriples, Pairs, Triples}; // Ensure these are correctly imported
use na::LU;
use nalgebra as na;
use num::Complex;

/// A Solver implementation using the Nalgebra library.
pub(crate) struct NalgebraSolver {
    /// The conductance matrix `A`.
    a_mat: na::DMatrix<f64>,
    /// The vector `b`.
    b_vec: na::DVector<f64>,
    /// The Solution vector
    x_vec: na::DVector<f64>,
    /// The conductance matrix `A`.
    cplx_a_mat: na::DMatrix<Complex<f64>>,
    /// The vector `b`.
    cplx_b_vec: na::DVector<Complex<f64>>,
    /// The Solution vector
    cplx_x_vec: na::DVector<Complex<f64>>,
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

    /// Sets the conductance matrix (`a_mat`) and the vector (`b_vec`) into the Solver.
    /// It can be used to change only the necessary changes.
    fn set_a(&mut self, a_mat: &Triples) {
        match a_mat {
            Triples::Empty => {}
            Triples::Single(tr) => self.set_single(tr),
            Triples::Double(tr) => self.set_double(tr),
            Triples::Quad(tr) => self.set_quad(tr),
            Triples::Vec(trs_vec) => self.set_vec(trs_vec), // New match arm for Vec variant
        };
    }

    fn set_b(&mut self, b_vec: &Pairs) {
        match b_vec {
            Pairs::Empty => {}
            Pairs::Single((col, val)) => {
                self.b_vec[*col] = *val;
            }
            Pairs::Double([(col1, val1), (col2, val2)]) => {
                self.b_vec[*col1] = *val1;
                self.b_vec[*col2] = *val2;
            }
            Pairs::Vec(pairs_vec) => self.set_b_vec_from_pairs_vec(pairs_vec), // New for Pairs::Vec
        }
    }

    fn solve(&mut self) -> Result<&Vec<f64>, SolverError> {
        // Cloning only the necessary matrices for LU decomposition
        let lu = LU::new(self.a_mat.clone());

        // Solving the equations without unnecessary cloning
        self.x_vec = match lu.solve(&self.b_vec) {
            Some(v) => v,
            None => return Err(SolverError::MatrixNonInvertible),
        };

        // Returning a reference to the solution vector
        Ok(&self.x_vec.data.as_vec())
    }

    fn set_cplx_a(&mut self, a_mat: &ComplexTriples) {
        match a_mat {
            ComplexTriples::Empty => {}
            ComplexTriples::Single(tr) => self.set_cplx_single(tr),
            ComplexTriples::Double(tr) => self.set_cplx_double(tr),
            ComplexTriples::Quad(tr) => self.set_cplx_quad(tr),
            ComplexTriples::Vec(trs_vec) => self.set_cplx_vec(trs_vec), // New match arm for ComplexTriples::Vec
        };
    }

    fn set_cplx_b(&mut self, b_vec: &ComplexPairs) {
        match b_vec {
            ComplexPairs::Empty => {}
            ComplexPairs::Single((col, val)) => {
                self.cplx_b_vec[*col] = *val;
            }
            ComplexPairs::Double([(col1, val1), (col2, val2)]) => {
                self.cplx_b_vec[*col1] = *val1;
                self.cplx_b_vec[*col2] = *val2;
            }
            ComplexPairs::Vec(pairs_vec) => self.set_cplx_b_vec_from_pairs_vec(pairs_vec), // New for ComplexPairs::Vec
        }
    }

    fn solve_cplx(&mut self) -> Result<&Vec<num::Complex<f64>>, SolverError> {
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

impl NalgebraSolver {
    /// Sets a single-valued triple into the conductance matrix.
    fn set_single(&mut self, triple: &(usize, usize, f64)) {
        self.a_mat[(triple.0, triple.1)] = triple.2;
    }

    /// Sets a double-valued triple into the conductance matrix.
    fn set_double(&mut self, triple: &[(usize, usize, f64); 2]) {
        self.a_mat[(triple[0].0, triple[0].1)] = triple[0].2;
        self.a_mat[(triple[1].0, triple[1].1)] = triple[1].2;
    }

    /// Sets a quad-valued triple into the conductance matrix.
    fn set_quad(&mut self, triple: &[(usize, usize, f64); 4]) {
        self.a_mat[(triple[0].0, triple[0].1)] = triple[0].2;
        self.a_mat[(triple[1].0, triple[1].1)] = triple[1].2;
        self.a_mat[(triple[2].0, triple[2].1)] = triple[2].2;
        self.a_mat[(triple[3].0, triple[3].1)] = triple[3].2;
    }

    /// Sets a vector of triples into the conductance matrix.
    // New helper function for Triples::Vec
    fn set_vec(&mut self, triples_vec: &Vec<(usize, usize, f64)>) {
        for triple in triples_vec.iter() {
            // Re-use the logic from set_single for each element
            self.set_single(triple);
        }
    }

    /// Sets a vector of pairs into the b_vec.
    // New helper function for Pairs::Vec
    fn set_b_vec_from_pairs_vec(&mut self, pairs_vec: &Vec<(usize, f64)>) {
        for (col, val) in pairs_vec.iter() {
            self.b_vec[*col] = *val;
        }
    }

    /// Sets a single-valued triple into the complex conductance matrix.
    fn set_cplx_single(&mut self, triple: &(usize, usize, Complex<f64>)) {
        self.cplx_a_mat[(triple.0, triple.1)] = triple.2;
    }

    /// Sets a double-valued triple into the complex conductance matrix.
    fn set_cplx_double(&mut self, triple: &[(usize, usize, Complex<f64>); 2]) {
        self.cplx_a_mat[(triple[0].0, triple[0].1)] = triple[0].2;
        self.cplx_a_mat[(triple[1].0, triple[1].1)] = triple[1].2;
    }

    /// Sets a quad-valued triple into the complex conductance matrix.
    fn set_cplx_quad(&mut self, triple: &[(usize, usize, Complex<f64>); 4]) {
        self.cplx_a_mat[(triple[0].0, triple[0].1)] = triple[0].2;
        self.cplx_a_mat[(triple[1].0, triple[1].1)] = triple[1].2;
        self.cplx_a_mat[(triple[2].0, triple[2].1)] = triple[2].2;
        self.cplx_a_mat[(triple[3].0, triple[3].1)] = triple[3].2;
    }

    /// Sets a vector of complex triples into the complex conductance matrix.
    // New helper function for ComplexTriples::Vec
    fn set_cplx_vec(&mut self, triples_vec: &Vec<(usize, usize, Complex<f64>)>) {
        for triple in triples_vec.iter() {
            // Re-use the logic from set_cplx_single for each element
            self.set_cplx_single(triple);
        }
    }

    /// Sets a vector of complex pairs into the complex b_vec.
    // New helper function for ComplexPairs::Vec
    fn set_cplx_b_vec_from_pairs_vec(&mut self, pairs_vec: &Vec<(usize, Complex<f64>)>) {
        for (col, val) in pairs_vec.iter() {
            self.cplx_b_vec[*col] = *val;
        }
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
    ) -> &na::Matrix<f64, na::Dyn, na::Dyn, na::VecStorage<f64, na::Dyn, na::Dyn>> {
        &self.a_mat
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &na::Matrix<f64, na::Dyn, na::U1, na::VecStorage<f64, na::Dyn, na::U1>> {
        &self.b_vec
    }
}
