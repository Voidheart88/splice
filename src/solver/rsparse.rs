#![allow(unused)]

use super::{Solver, SolverError};
use crate::models::{Pairs, Triples};
use crate::spot::*;
use log::trace;
use num::complex::ComplexFloat;
use num::Complex;
use rsparse::data::{Nmrc, Sprs, Symb, Trpl};
use rsparse::lusol;

/// A Solver implementation using the Faer library.
pub(crate) struct RSparseSolver {
    vars: usize,
    /// The conductance matrix `A` as a sparse matrix.
    a: Trpl<Numeric>,
    /// The vector `b` as a dense vector.
    b: Vec<Numeric>,
    /// The Solution vector `x`.
    x: Vec<Numeric>,

    // Sparse Matrix Workspace
    sprs: Sprs<Numeric>,
    symb: Option<Symb>,
    lu: Nmrc<Numeric>,

    /// The conductance matrix `A` as a sparse matrix.
    cplx_a: Trpl<Numeric>,
    /// The vector `b` as a dense vector.
    cplx_b: Vec<Numeric>,
    /// The Solution vector `x`.
    cplx_x: Vec<Complex<Numeric>>,

    //Complex Sparse Matrix Workspace
    cplx_sprs: Sprs<Numeric>,
    cplx_symb: Option<Symb>,
}

impl Solver for RSparseSolver {
    /// Creates a new instance of the Faer Solver with the given number of variables.
    fn new(vars: usize) -> Result<Self, SolverError> {
        let a = Trpl::new();
        let b = vec![0.; vars];
        let x = vec![0.; vars];
        let sprs = Sprs::new();
        let lu = Nmrc::new();

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
            symb: None,
            lu,
            cplx_a,
            cplx_b,
            cplx_x,
            cplx_sprs,
            cplx_symb: None,
        })
    }

    fn set_a(&mut self, a_mat: (usize,usize,Numeric)) {}

    fn set_b(&mut self, b_vec: (usize,Numeric)) {}


    /// Solves the system of equations (Ax = B for x) and returns a reference to the solution.
    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        self.sprs.from_trpl(&self.a);
        if self.symb.is_none() {
            self.symb = Some(rsparse::sqr(&self.sprs, 1, false))
        }
        let mut symb = self.symb.take().unwrap();
        self.lu = rsparse::lu(&self.sprs, &mut symb, 1e-6).unwrap();

        ipvec(self.sprs.n, &self.lu.pinv, &self.b, &mut self.x[..]);
        rsparse::lsolve(&self.lu.l, &mut self.x);
        rsparse::usolve(&self.lu.u, &mut self.x[..]);
        ipvec(self.sprs.n, &symb.q, &self.x[..], &mut self.b[..]);

        self.symb = Some(symb);
        Ok(&self.b)
    }

    fn set_cplx_a(&mut self, a_mat: (usize,usize,ComplexNumeric)) {
    }

    fn set_cplx_b(&mut self, b_vec: (usize,ComplexNumeric)) {
    }

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        // Convert the triplet matrix to a sparse matrix
        self.cplx_sprs.from_trpl(&self.cplx_a);
        rsparse::lusol(&self.cplx_sprs, &mut self.cplx_b, 1, 1e-6);
        self.cplx_x = self.real_vec_to_complex_vec();

        Ok(&self.cplx_x)
    }
}

impl RSparseSolver {

    pub fn real_vec_to_complex_vec(&self) -> Vec<ComplexNumeric> {
        let pivot = self.vars;
        let real = &self.cplx_b[..pivot];
        let imag = &self.cplx_b[pivot..];
        let iter = real.iter().zip(imag.iter());
        iter.map(|(re, im)| Complex { re: *re, im: *im }).collect()
    }
}

fn ipvec(n: usize, p: &Option<Vec<isize>>, b: &[Numeric], x: &mut [Numeric]) {
    for k in 0..n {
        if p.is_some() {
            x[p.as_ref().unwrap()[k] as usize] = b[k];
        } else {
            x[k] = b[k];
        }
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
    pub fn a_mat(&self) -> &Trpl<Numeric> {
        &self.a
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &Vec<Numeric> {
        &self.b
    }
}
