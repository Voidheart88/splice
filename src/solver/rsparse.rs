#![allow(unused)]

use super::{Solver, SolverError};
use crate::models::{ComplexPairs, ComplexTriples, Pairs, Triples};
use log::trace;
use num::complex::ComplexFloat;
use num::Complex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::vec;
use rsparse::data::{Sprs, Trpl};
use rsparse::lusol;

/// A Solver implementation using the Faer library.
pub(crate) struct RSparseSolver {
    vars: usize,
    /// The conductance matrix `A` as a sparse matrix.
    a: Trpl,
    /// The vector `b` as a dense vector.
    b: Vec<f64>,
    /// The Solution vector `x`.
    x: Vec<f64>,

    // Sparse Matrix Workspace
    sprs: Sprs,

    /// The conductance matrix `A` as a sparse matrix.
    cplx_a: Trpl,
    /// The vector `b` as a dense vector.
    cplx_b: Vec<f64>,
    /// The Solution vector `x`.
    cplx_x: Vec<Complex<f64>>,

    //Complex Sparse Matrix Workspace
    cplx_sprs: Sprs,
}

impl Solver for RSparseSolver {
    /// Creates a new instance of the Faer Solver with the given number of variables.
    fn new(vars: usize) -> Result<Self, SolverError> {
        let a = Trpl::new();
        let b = Vec::with_capacity(vars);
        let x = Vec::with_capacity(vars);
        let sprs = Sprs::new();

        let cplx_a = Trpl::new();
        let cplx_b = Vec::with_capacity(2 * vars);
        let cplx_x = Vec::with_capacity(2 * vars);
        let cplx_sprs = Sprs::new();

        Ok(Self {
            vars,
            a,
            b,
            x,
            sprs,
            cplx_a,
            cplx_b,
            cplx_x,
            cplx_sprs,
        })
    }

    /// Sets the conductance matrix (`a_mat`) into the Solver.
    fn set_a(&mut self, a_mat: &Triples) {
        let mut new_a = Trpl::new();
        match a_mat {
            Triples::Empty => {}
            Triples::Single(tr) => new_a.append(tr.0, tr.1, tr.2),
            Triples::Double(tr) => {
                new_a.append(tr[0].0, tr[0].1, tr[0].2);
                new_a.append(tr[1].0, tr[1].1, tr[1].2);
            }
            Triples::Quad(tr) => {
                new_a.append(tr[0].0, tr[0].1, tr[0].2);
                new_a.append(tr[1].0, tr[1].1, tr[1].2);
                new_a.append(tr[2].0, tr[2].1, tr[2].2);
                new_a.append(tr[3].0, tr[3].1, tr[3].2);
            }
            Triples::Vec(triples) => {
                for (row, col, v) in triples.iter() {
                    new_a.append(*row, *col, *v);
                }
            }
        }
        self.a = new_a;
    }

    /// Sets the known values vector (`b_vec`) into the Solver.
    fn set_b(&mut self, b_vec: &Pairs) {
        self.b = vec![0.0; self.b.capacity()];
        match b_vec {
            Pairs::Empty => {}
            Pairs::Single((col, val)) => self.b[*col] = *val,
            Pairs::Double([(col1, val1), (col2, val2)]) => {
                self.b[*col1] = *val1;
                self.b[*col2] = *val2;
            }
            Pairs::Vec(pairs) => pairs.iter().for_each(|(col, val)| {
                self.b[*col] = *val;
            }),
        }
    }

    /// Solves the system of equations (Ax = B for x) and returns a reference to the solution.
    fn solve(&mut self) -> Result<&Vec<f64>, SolverError> {
        // Convert the triplet matrix to a sparse matrix
        self.sprs.from_trpl(&self.a);
        rsparse::lusol(&self.sprs, &mut self.b, 1, 1e-6);
        Ok(&self.b)
    }

    fn set_cplx_a(&mut self, a_mat: &ComplexTriples) {
        let triples = self.cplx_triple_to_triple(a_mat);
        let mut new_a = Trpl::new();
        match triples {
            Triples::Empty => {}
            Triples::Single(tr) => new_a.append(tr.0, tr.1, tr.2),
            Triples::Double(tr) => {
                new_a.append(tr[0].0, tr[0].1, tr[0].2);
                new_a.append(tr[1].0, tr[1].1, tr[1].2);
            }
            Triples::Quad(tr) => {
                new_a.append(tr[0].0, tr[0].1, tr[0].2);
                new_a.append(tr[1].0, tr[1].1, tr[1].2);
                new_a.append(tr[2].0, tr[2].1, tr[2].2);
                new_a.append(tr[3].0, tr[3].1, tr[3].2);
            }
            Triples::Vec(triples) => {
                for (row, col, v) in triples.iter() {
                    new_a.append(*row, *col, *v);
                }
            }
        }
        self.cplx_a = new_a;
    }

    fn set_cplx_b(&mut self, b_vec: &ComplexPairs) {
        let cplx_b_vec = self.complex_pair_to_pair(b_vec);
        self.cplx_b = vec![0.0; self.cplx_b.capacity()];
        match cplx_b_vec {
            Pairs::Empty => {}
            Pairs::Single((col, val)) => self.cplx_b[col] = val,
            Pairs::Double([(col1, val1), (col2, val2)]) => {
                self.cplx_b[col1] = val1;
                self.cplx_b[col2] = val2;
            }
            Pairs::Vec(pairs) => pairs.iter().for_each(|(col, val)| {
                self.cplx_b[*col] = *val;
            }),
        }
    }

    fn solve_cplx(&mut self) -> Result<&Vec<num::Complex<f64>>, SolverError> {
        // Convert the triplet matrix to a sparse matrix
        self.cplx_sprs.from_trpl(&self.cplx_a);
        rsparse::lusol(&self.cplx_sprs, &mut self.cplx_b, 1, 1e-6);
        self.cplx_x = self.real_vec_to_complex_vec();

        Ok(&self.cplx_x)
    }
}

impl RSparseSolver {
    /// Performs a conversion from complex Triples to rational Triples
    /// Ac in ℂ:
    /// Ac = Ar + jAi
    /// A  = (Ar -Ai)
    ///      (Ai  Ar)
    /// Example:
    /// Ac = ( 1+1i 2+2i )
    ///      ( 3+3i 4+4i )
    /// A  = ( 1 2 ) | ( -1 -2 )
    ///      ( 3 4 ) | ( -3 -4 )
    ///      --------+----------
    ///      ( 1 2 ) | (  1  2 )
    ///      ( 3 4 ) | (  3  4 )
    ///
    /// A = ( 1  2 -1 -2 )
    ///     ( 3  4 -3 -4 )
    ///     ( 1  2  1  2 )
    ///     ( 3  4  3  4 )
    pub fn cplx_triple_to_triple(&self, triple: &ComplexTriples) -> Triples {
        let pivot = self.vars;
        match triple {
            ComplexTriples::Empty => Triples::Empty,
            ComplexTriples::Single((row, col, val)) => Triples::Quad([
                (*row, *col, val.re()),
                (*row, pivot + col, -val.im()),
                (pivot + row, *col, val.im()),
                (pivot + row, pivot + col, val.re()),
            ]),
            ComplexTriples::Double(vals) => vals
                .iter()
                .flat_map(|val: &(usize, usize, Complex<f64>)| {
                    vec![
                        (val.0, val.1, val.2.re),
                        (val.0, pivot + val.1, -val.2.im),
                        (pivot + val.0, val.1, val.2.im),
                        (pivot + val.0, pivot + val.1, val.2.re),
                    ]
                })
                .collect(),
            ComplexTriples::Quad(vals) => vals
                .iter()
                .flat_map(|val: &(usize, usize, Complex<f64>)| {
                    vec![
                        (val.0, val.1, val.2.re),
                        (val.0, pivot + val.1, -val.2.im),
                        (pivot + val.0, val.1, val.2.im),
                        (pivot + val.0, pivot + val.1, val.2.re),
                    ]
                })
                .collect(),
            ComplexTriples::Vec(vals) => vals
                .iter()
                .flat_map(|val: &(usize, usize, Complex<f64>)| {
                    vec![
                        (val.0, val.1, val.2.re),
                        (val.0, pivot + val.1, -val.2.im),
                        (pivot + val.0, val.1, val.2.im),
                        (pivot + val.0, pivot + val.1, val.2.re),
                    ]
                })
                .collect(),
        }
    }

    pub fn complex_pair_to_pair(&self, pairs: &ComplexPairs) -> Pairs {
        let pivot = self.vars;
        match pairs {
            ComplexPairs::Empty => Pairs::Empty,
            ComplexPairs::Single((index, value)) => {
                Pairs::Double([(*index, value.re()), (pivot + index, value.im())])
            }
            ComplexPairs::Double(pairs) => pairs
                .iter()
                .flat_map(|val| vec![(val.0, val.1.re()), (val.0, val.1.im())])
                .collect(),
            ComplexPairs::Vec(pairs) => pairs
                .iter()
                .flat_map(|val| vec![(val.0, val.1.re()), (val.0, val.1.im())])
                .collect(),
        }
    }

    pub fn real_vec_to_complex_vec(&self) -> Vec<Complex<f64>> {
        let pivot = self.vars;
        let real = &self.cplx_b[..pivot];
        let imag = &self.cplx_b[pivot..];
        let iter = real.iter().zip(imag.iter());
        iter.map(|(re, im)| Complex { re: *re, im: *im }).collect()
    }
}

#[cfg(test)]
impl RSparseSolver {
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
