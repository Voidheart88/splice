#![allow(unused)]

use std::collections::HashMap;

use super::{Solver, SolverError};
use crate::models::{Pairs, Triples};
use crate::spot::*;
use log::trace;
use num::complex::ComplexFloat;
use num::{Complex, Zero};
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
    /// Creates a new instance of the Solver with the given number of variables.
    fn new(vars: usize) -> Result<Self, SolverError> {
        let a = Trpl::new();
        let b = vec![0.; vars];
        let x = vec![0.; vars];
        let sprs = Sprs::new();
        let lu = Nmrc::new();

        let cplx_a = Trpl::new();
        let cplx_b = vec![0.; 2 * vars];
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

    fn insert_a(&mut self, a_mat: &(usize, usize, Numeric)) {
        let (row, col, val) = *a_mat;
        self.a.append(row, col, val);
    }

    fn insert_b(&mut self, b_vec: &(usize, Numeric)) {
        let (row, val) = *b_vec;
        self.b[row] = val;
    }

    fn insert_cplx_a(&mut self, a_mat: &(usize, usize, ComplexNumeric)) {
        let (row, col, val) = *a_mat;
        let pivot = self.vars;

        self.cplx_a.append(row, col, val.re);
        self.cplx_a.append(row, col + pivot, -val.im);
        self.cplx_a.append(row + pivot, col, val.im);
        self.cplx_a.append(row + pivot, col + pivot, val.re);
    }

    fn insert_cplx_b(&mut self, b_vec: &(usize, ComplexNumeric)) {
        let (row, val) = *b_vec;
        let pivot = self.cplx_b.len() / 2;
        self.cplx_b[row] = val.re;
        self.cplx_b[row + pivot] = val.im;
    }

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

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        // Convert the triplet matrix to a sparse matrix
        self.cplx_sprs.from_trpl(&self.cplx_a);
        rsparse::lusol(&self.cplx_sprs, &mut self.cplx_b, 1, 1e-6);
        self.cplx_x = self.real_vec_to_complex_vec();

        Ok(&self.cplx_x)
    }

    fn init(
        &mut self,
        a_matrix: Vec<(usize, usize)>,
        b_vec: Vec<usize>,
        cplx_a_matrix: Vec<(usize, usize)>,
        cplx_b_vec: Vec<usize>,
    ) {
        let mut trpl = Trpl::new();
        a_matrix
            .iter()
            .for_each(|val| trpl.append(val.0, val.1, Numeric::zero()));

        let pivot = self.vars;
        let mut cplx_trpl = Trpl::new();
        cplx_a_matrix.iter().for_each(|val| {
            cplx_trpl.append(val.0, val.1, Numeric::zero());
            cplx_trpl.append(val.0, val.1 + pivot, Numeric::zero());
            cplx_trpl.append(val.0 + pivot, val.1, Numeric::zero());
            cplx_trpl.append(val.0 + pivot, val.1 + pivot, Numeric::zero());
        });
        trpl.sum_dupl();

        let mut sprs = Sprs::new();
        sprs.from_trpl(&trpl);
        self.symb = Some(rsparse::sqr(&sprs, 1, false));

        let mut cplx_sprs = Sprs::new();
        cplx_sprs.from_trpl(&cplx_trpl);
        self.cplx_symb = Some(rsparse::sqr(&cplx_sprs, 1, false));
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

    /// Returns the number of rows in the matrix `a_mat`.
    pub fn cplx_rows(&self) -> usize {
        self.cplx_a.n
    }

    /// Returns the number of columns in the matrix `a_mat`.
    pub fn cplx_cols(&self) -> usize {
        self.cplx_a.m
    }

    /// Returns the length of the vector `b_vec`.
    pub fn b_vec_len(&self) -> usize {
        self.b.len()
    }

    /// Returns the length of the vector `b_vec`.
    pub fn cplx_b_vec_len(&self) -> usize {
        self.cplx_b.len()
    }

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat(&self) -> &Trpl<Numeric> {
        &self.a
    }

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat_mut(&mut self) -> &mut Trpl<Numeric> {
        &mut self.a
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &Vec<Numeric> {
        &self.b
    }

    /// Returns a reference to the matrix `cplx_a_mat`.
    pub fn cplx_a_mat(&self) -> &Trpl<Numeric> {
        &self.cplx_a
    }

    /// Returns a reference to the vector `cplx_b_vec`.
    pub fn cplx_b_vec(&self) -> &Vec<Numeric> {
        &self.cplx_b
    }

    pub fn print_matrix_from_trpl(triple: Trpl<f64>) {
        let m = triple.m;
        let n = triple.n;
        let p = triple.p;
        let i = triple.i;
        let x = triple.x;

        let mut matrix = vec![vec![0.0; n]; m];

        for k in 0..x.len() {
            let row = i[k];
            let col = p[k];
            if row < m && col < n as isize {
                matrix[row][col as usize] = x[k];
            } else {
                eprintln!(
                    "Warning: Index out of bounds detected for element at index {}. Skipping.",
                    k
                );
            }
        }

        // Print the matrix in a formatted way.
        for row in 0..m {
            print!("[");
            for col in 0..n {
                print!("{:>8.2}", matrix[row][col]); // Format to 2 decimal places, right-aligned, 8 chars wide
                if col < n - 1 {
                    print!(", ");
                }
            }
            println!("]");
        }
    }
}
