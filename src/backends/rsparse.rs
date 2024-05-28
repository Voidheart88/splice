#![allow(unused)]

use super::{Backend, BackendError, Col, Row};
use crate::models::{Doubles, Triples};
use log::trace;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::vec;
use rsparse::data::{Sprs, Trpl};
use rsparse::lusol;

/// A backend implementation using the Faer library.
pub(crate) struct RSparseBackend {
    /// The conductance matrix `A` as a sparse matrix.
    a: Trpl,
    /// The vector `b` as a dense vector.
    b: Vec<f64>,
    /// The Solution vector `x`.
    x: Vec<f64>,
}

impl Backend for RSparseBackend {
    /// Creates a new instance of the Faer backend with the given number of variables.
    fn new(vars: usize) -> Result<Self, BackendError> {
        let a = Trpl::new();
        let b = Vec::with_capacity(vars);
        let x = Vec::with_capacity(vars);

        Ok(Self { a, b, x })
    }

    /// Sets the conductance matrix (`a_mat`) into the backend.
    fn set_a(&mut self, a_mat: &Triples) {
        let mut new_a = Trpl::new();
        match a_mat {
            Triples::Empty => {}
            Triples::Single(tr) => new_a.append(tr.0 .0, tr.1 .0, tr.2),
            Triples::Double(tr) => {
                new_a.append(tr[0].0 .0, tr[0].1 .0, tr[0].2);
                new_a.append(tr[1].0 .0, tr[1].1 .0, tr[1].2);
            }
            Triples::Quad(tr) => {
                new_a.append(tr[0].0 .0, tr[0].1 .0, tr[0].2);
                new_a.append(tr[1].0 .0, tr[1].1 .0, tr[1].2);
                new_a.append(tr[2].0 .0, tr[2].1 .0, tr[2].2);
                new_a.append(tr[3].0 .0, tr[3].1 .0, tr[3].2);
            }
            Triples::Vec(triples) => {
                for (r, c, v) in triples.iter() {
                    new_a.append(r.0, c.0, *v);
                }
            }
        }
        self.a = new_a;
    }

    /// Sets the known values vector (`b_vec`) into the backend.
    fn set_b(&mut self, b_vec: &Doubles) {
        self.b = vec![0.0; self.b.capacity()];
        match b_vec {
            Doubles::Empty => {}
            Doubles::Single((col, val)) => self.b[col.0] = *val,
            Doubles::Double([(col1, val1), (col2, val2)]) => {
                self.b[col1.0] = *val1;
                self.b[col2.0] = *val2;
            }
            Doubles::Vec(doubles) => doubles.iter().for_each(|(col, val)| {
                self.b[col.0] = *val;
            }),
        }
    }

    /// Inserts the conductance matrix (`a_mat`) into the backend.
    fn insert_a(&mut self, a_mat: &Triples) {
        match a_mat {
            Triples::Empty => {}
            Triples::Single(tr) => self.a.append(tr.0 .0, tr.1 .0, tr.2),
            Triples::Double(tr) => {
                self.a.append(tr[0].0 .0, tr[0].1 .0, tr[0].2);
                self.a.append(tr[1].0 .0, tr[1].1 .0, tr[1].2);
            }
            Triples::Quad(tr) => {
                self.a.append(tr[0].0 .0, tr[0].1 .0, tr[0].2);
                self.a.append(tr[1].0 .0, tr[1].1 .0, tr[1].2);
                self.a.append(tr[2].0 .0, tr[2].1 .0, tr[2].2);
                self.a.append(tr[3].0 .0, tr[3].1 .0, tr[3].2);
            }
            Triples::Vec(triples) => {
                for (r, c, v) in triples.iter() {
                    self.a.append(r.0, c.0, *v);
                }
            }
        }
    }

    /// Inserts the known values vector (`b_vec`) into the backend.
    fn insert_b(&mut self, b_vec: &Doubles) {
        match b_vec {
            Doubles::Empty => {}
            Doubles::Single((col, val)) => self.b[col.0] += *val,
            Doubles::Double([(col1, val1), (col2, val2)]) => {
                self.b[col1.0] += *val1;
                self.b[col2.0] += *val2;
            }
            Doubles::Vec(doubles) => {
                for (col, val) in doubles.iter() {
                    self.b[col.0] += *val;
                }
            }
        }
    }

    /// Solves the system of equations (Ax = B for x) and returns a reference to the solution.
    fn solve(&mut self) -> Result<&Vec<f64>, BackendError> {
        // Convert the triplet matrix to a sparse matrix
        let mut sprs = Sprs::new_from_trpl(&self.a);

        rsparse::lusol(&sprs, &mut self.b, 1, 1e-6);

        Ok(&self.b)
    }
}

impl RSparseBackend {}

#[cfg(test)]
impl RSparseBackend {
    /// Returns the number of rows in the matrix `a_mat`.
    pub fn rows(&self) -> usize {
        self.a.n
    }

    /// Returns the number of columns in the matrix `a_mat`.
    pub fn cols(&self) -> usize {
        self.a.m
    }

    /// Returns the length of the vector `b_vec`.
    pub fn b_vec_len(&self) -> usize {
        self.b.len()
    }

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat(&self) -> &Trpl {
        &self.a
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &Vec<f64> {
        &self.b
    }
}
