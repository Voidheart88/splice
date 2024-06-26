use super::{Backend, BackendError, Col, Row};
use crate::models::{Doubles, Triples};
use na::LU;
use nalgebra as na;

/// A backend implementation using the Nalgebra library.
pub(crate) struct NalgebraBackend {
    /// The conductance matrix `A`.
    a_mat: na::DMatrix<f64>,
    /// The non linear elements of the conductance matrix `A`.
    /// The vector `b`.
    b_vec: na::DVector<f64>,
    /// The Solution vector
    x1: na::DVector<f64>,
}

impl Backend for NalgebraBackend {
    /// Creates a new instance of the Nalgebra backend with the given number of variables.
    fn new(vars: usize) -> Result<NalgebraBackend, BackendError> {
        let a_mat = na::DMatrix::zeros(vars, vars);
        let b_vec = na::DVector::zeros(vars);
        let x1 = na::DVector::zeros(vars);

        Ok(Self { a_mat, b_vec, x1 })
    }

    /// Sets the conductance matrix (`a_mat`) and the vector (`b_vec`) into the backend.
    /// It can be used to change only the necessary changes.
    fn set_a(&mut self, a_mat: &Triples) {
        match a_mat {
            Triples::Empty => {}
            Triples::Single(tr) => self.set_single(tr),
            Triples::Double(tr) => self.set_double(tr),
            Triples::Quad(tr) => self.set_quad(tr),
            Triples::Vec(triples) => triples.iter().for_each(|(row, col, val)| {
                self.a_mat[(row.0, col.0)] = *val;
            }),
        };
    }

    fn set_b(&mut self, b_vec: &Doubles) {
        match b_vec {
            Doubles::Empty => {}
            Doubles::Single((col, val)) => {
                self.b_vec[col.0] = *val;
            }
            Doubles::Double([(col1, val1), (col2, val2)]) => {
                self.b_vec[col1.0] = *val1;
                self.b_vec[col2.0] = *val2;
            }
            Doubles::Vec(doubles) => doubles.iter().for_each(|(col, val)| {
                self.b_vec[col.0] = *val;
            }),
        }
    }

    /// Inserts the conductance matrix (`a_mat`) and the vector (`b_vec`) into the backend.
    /// It can be used to change only the necessary changes.
    fn insert_a(&mut self, a_mat: &Triples) {
        match a_mat {
            Triples::Empty => {}
            Triples::Single(tr) => self.insert_single(tr),
            Triples::Double(tr) => self.insert_double(tr),
            Triples::Quad(tr) => self.insert_quad(tr),
            Triples::Vec(triples) => triples.iter().for_each(|(row, col, val)| {
                self.a_mat[(row.0, col.0)] += *val;
            }),
        };
    }

    fn insert_b(&mut self, b_vec: &Doubles) {
        match b_vec {
            Doubles::Empty => {}
            Doubles::Single((col, val)) => {
                self.b_vec[col.0] += *val;
            }
            Doubles::Double([(col1, val1), (col2, val2)]) => {
                self.b_vec[col1.0] += *val1;
                self.b_vec[col2.0] += *val2;
            }
            Doubles::Vec(doubles) => doubles.iter().for_each(|(col, val)| {
                self.b_vec[col.0] += *val;
            }),
        }
    }

    fn solve(&mut self) -> Result<&Vec<f64>, BackendError> {
        // Cloning only the necessary matrices for LU decomposition
        let lu = LU::new(self.a_mat.clone());

        // Solving the equations without unnecessary cloning
        self.x1 = match lu.solve(&self.b_vec) {
            Some(v) => v,
            None => return Err(BackendError::MatrixNonInvertible),
        };

        // Returning a reference to the solution vector
        Ok(&self.x1.data.as_vec())
    }
}

impl NalgebraBackend {
    /// Sets a single-valued triple into the conductance matrix.
    fn set_single(&mut self, triple: &(Row, Col, f64)) {
        self.a_mat[(triple.0 .0, triple.1 .0)] = triple.2;
    }

    /// Sets a double-valued triple into the conductance matrix.
    fn set_double(&mut self, triple: &[(Row, Col, f64); 2]) {
        self.a_mat[(triple[0].0 .0, triple[0].1 .0)] = triple[0].2;
        self.a_mat[(triple[1].0 .0, triple[1].1 .0)] = triple[1].2;
    }

    /// Sets a quad-valued triple into the conductance matrix.
    fn set_quad(&mut self, triple: &[(Row, Col, f64); 4]) {
        self.a_mat[(triple[0].0 .0, triple[0].1 .0)] = triple[0].2;
        self.a_mat[(triple[1].0 .0, triple[1].1 .0)] = triple[1].2;
        self.a_mat[(triple[2].0 .0, triple[2].1 .0)] = triple[2].2;
        self.a_mat[(triple[3].0 .0, triple[3].1 .0)] = triple[3].2;
    }

    /// Inserts a single-valued triple into the conductance matrix.
    fn insert_single(&mut self, triple: &(Row, Col, f64)) {
        self.a_mat[(triple.0 .0, triple.1 .0)] *= triple.2;
    }

    /// Inserts a double-valued triple into the conductance matrix.
    fn insert_double(&mut self, triple: &[(Row, Col, f64); 2]) {
        self.a_mat[(triple[0].0 .0, triple[0].1 .0)] *= triple[0].2;
        self.a_mat[(triple[1].0 .0, triple[1].1 .0)] *= triple[1].2;
    }

    /// Inserts a quad-valued triple into the conductance matrix.
    fn insert_quad(&mut self, triple: &[(Row, Col, f64); 4]) {
        self.a_mat[(triple[0].0 .0, triple[0].1 .0)] *= triple[0].2;
        self.a_mat[(triple[1].0 .0, triple[1].1 .0)] *= triple[1].2;
        self.a_mat[(triple[2].0 .0, triple[2].1 .0)] *= triple[2].2;
        self.a_mat[(triple[3].0 .0, triple[3].1 .0)] *= triple[3].2;
    }
}

#[cfg(test)]
impl NalgebraBackend {
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
    pub fn b_vec(
        &self,
    ) -> &na::Matrix<f64, na::Dyn, na::Const<1>, na::VecStorage<f64, na::Dyn, na::Const<1>>> {
        &self.b_vec
    }
}
