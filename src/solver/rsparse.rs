#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

use super::{Solver, SolverError};
use crate::models::{Pairs, Triples};
use crate::spot::*;
use log::trace;
use log::warn;
use num::complex::ComplexFloat;
use num::{Complex, Zero};
use rsparse::data::{Nmrc, Sprs, Symb, Trpl};
use rsparse::lusol;

/// A Solver implementation using the Faer library.
#[derive(Debug)]
pub struct RSparseSolver {
    vars: usize,
    /// The conductance matrix `A` as a sparse matrix.
    a_mat: HashMap<(usize, usize), Numeric>,
    /// The vector `b` as a dense vector.
    b_vec: Vec<Numeric>,
    /// The Solution vector `x`.
    x_vec: Vec<Numeric>,

    // Sparse Matrix Workspace
    sprs: Sprs<Numeric>,
    symb: Option<Symb>,
    lu: Nmrc<Numeric>,

    /// The conductance matrix `A` as a sparse matrix.
    cplx_a_mat: HashMap<(usize, usize), ComplexNumeric>,
    /// The vector `b` as a dense vector.
    cplx_b_vec: Vec<Numeric>,
    /// The Solution vector `x`.
    cplx_x_vec: Vec<Complex<Numeric>>,

    //Complex Sparse Matrix Workspace
    cplx_sprs: Sprs<Numeric>,
    cplx_symb: Option<Symb>,
}

impl Solver for RSparseSolver {
    /// Creates a new instance of the Solver with the given number of variables.
    fn new(vars: usize) -> Result<Self, SolverError> {
        let a_mat = HashMap::new();
        let b_vec = vec![0.; vars];
        let x_vec = vec![0.; vars];
        let sprs = Sprs::new();
        let lu = Nmrc::new();

        let cplx_a_mat = HashMap::new();
        let cplx_b_vec = vec![0.; 2 * vars];
        let cplx_x_vec = vec![ComplexNumeric { re: 0.0, im: 0.0 }; vars];
        let cplx_sprs = Sprs::new();

        Ok(Self {
            vars,
            a_mat,
            b_vec,
            x_vec,
            sprs,
            symb: None,
            lu,
            cplx_a_mat,
            cplx_b_vec,
            cplx_x_vec,
            cplx_sprs,
            cplx_symb: None,
        })
    }

    fn insert_a(&mut self, a_mat: &(usize, usize, Numeric)) {
        let (row, col, val) = *a_mat;
        match self.a_mat.get_mut(&(row, col)) {
            Some(v) => *v += val,
            None => {
                self.a_mat.insert((row, col), val);
            }
        };
    }

    fn insert_b(&mut self, b_vec: &(usize, Numeric)) {
        let (row, val) = *b_vec;
        self.b_vec[row] += val;
    }

    fn insert_cplx_a(&mut self, a_mat: &(usize, usize, ComplexNumeric)) {
        let (row, col, val) = *a_mat;
        match self.cplx_a_mat.get_mut(&(row, col)) {
            Some(v) => *v += val,
            None => {
                self.cplx_a_mat.insert((row, col), val);
            }
        };
    }

    fn insert_cplx_b(&mut self, b_vec: &(usize, ComplexNumeric)) {
        let (row, val) = *b_vec;
        let pivot = self.vars;
        self.cplx_b_vec[row] += val.re;
        self.cplx_b_vec[row + pivot] += val.im;
    }

    /// Solves the system of equations (Ax = B for x) and returns a reference to the solution.
    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError> {
        self.update_from_hashmap();
        if self.symb.is_none() {
            self.symb = Some(rsparse::sqr(&self.sprs, 1, false))
        }
        let mut symb = self.symb.take()
            .expect("Symbolic analysis data missing. This indicates the solver was not properly initialized.");

        self.lu = rsparse::lu(&self.sprs, &mut symb, 1e-6)
            .expect("LU decomposition failed. This indicates a singular or ill-conditioned matrix.");

        ipvec(self.sprs.n, &self.lu.pinv, &self.b_vec, &mut self.x_vec[..]);
        rsparse::lsolve(&self.lu.l, &mut self.x_vec);
        rsparse::usolve(&self.lu.u, &mut self.x_vec[..]);
        ipvec(self.sprs.n, &symb.q, &self.x_vec[..], &mut self.b_vec[..]);

        self.a_mat.clear();
        self.b_vec.iter_mut().for_each(|val| *val = Numeric::zero());
        self.symb = Some(symb);
        Ok(&self.x_vec)
    }

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError> {
        self.update_cplx_from_hashmap();
        rsparse::lusol(&self.cplx_sprs, &mut self.cplx_b_vec, 1, 1e-12);
        self.cplx_x_vec = self.real_vec_to_complex_vec();

        Ok(&self.cplx_x_vec)
    }

    fn init(&mut self, a_matrix: Vec<(usize, usize)>, cplx_a_matrix: Vec<(usize, usize)>) {
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

    fn reset(&mut self) {
        self.a_mat.clear();
        self.b_vec.iter_mut().for_each(|val| *val = Numeric::zero());
        self.cplx_a_mat.clear();
        self.cplx_b_vec
            .iter_mut()
            .for_each(|val| *val = Numeric::zero());
    }
}

impl RSparseSolver {
    fn real_vec_to_complex_vec(&self) -> Vec<ComplexNumeric> {
        let pivot = self.vars;
        let real = &self.cplx_b_vec[..pivot];
        let imag = &self.cplx_b_vec[pivot..];
        let iter = real.iter().zip(imag.iter());
        iter.map(|(re, im)| Complex { re: *re, im: *im }).collect()
    }

    /// Updates the complex sparse workspace from the hashmap
    fn update_cplx_from_hashmap(&mut self) {
        self.cplx_sprs.p.clear();
        self.cplx_sprs.i.clear();
        self.cplx_sprs.x.clear();

        if self.cplx_a_mat.is_empty() {
            self.cplx_sprs.nzmax = 0;
            self.cplx_sprs.m = 0;
            self.cplx_sprs.n = 0;
            self.cplx_sprs.p.push(0);
            return;
        }

        let mut max_row = 0;
        let mut max_col = 0;
        for ((r, c), _) in self.cplx_a_mat.iter() {
            if *r > max_row {
                max_row = *r;
            }
            if *c > max_col {
                max_col = *c;
            }
        }

        self.cplx_sprs.m = 2 * (max_row + 1);
        self.cplx_sprs.n = 2 * (max_col + 1);

        let mut entries: Vec<(usize, usize, Numeric)> = Vec::new();
        self.cplx_a_mat.iter().for_each(|((row, col), val)| {
            entries.push((*row, *col, val.re));
            entries.push((*row, *col + self.vars, val.im));
            entries.push((*row + self.vars, *col, -val.im));
            entries.push((*row + self.vars, *col + self.vars, val.re));
        });

        entries.sort_unstable_by(
            |(r1, c1, _), (r2, c2, _)| {
                if c1 != c2 {
                    c1.cmp(c2)
                } else {
                    r1.cmp(r2)
                }
            },
        );

        self.cplx_sprs.nzmax = entries.len();
        self.cplx_sprs.p.resize(self.cplx_sprs.n + 1, 0);
        self.cplx_sprs.i.reserve(self.cplx_sprs.nzmax);
        self.cplx_sprs.x.reserve(self.cplx_sprs.nzmax);

        let mut current_col = 0;
        for (idx, (row, col, val)) in entries.into_iter().enumerate() {
            while col > current_col {
                self.cplx_sprs.p[current_col + 1] = idx as isize;
                current_col += 1;
            }
            self.cplx_sprs.i.push(row);
            self.cplx_sprs.x.push(val);
        }

        while current_col < self.cplx_sprs.n {
            self.cplx_sprs.p[current_col + 1] = self.cplx_sprs.nzmax as isize;
            current_col += 1;
        }
    }

    /// Updates the sparse workspace from the hashmap
    pub fn update_from_hashmap(&mut self) {
        self.sprs.p.clear();
        self.sprs.i.clear();
        self.sprs.x.clear();

        if self.a_mat.is_empty() {
            self.sprs.nzmax = 0;
            self.sprs.m = 0;
            self.sprs.n = 0;
            self.sprs.p.push(0);
            return;
        }

        let mut max_row = 0;
        let mut max_col = 0;
        for ((r, c), _) in self.a_mat.iter() {
            if *r > max_row {
                max_row = *r;
            }
            if *c > max_col {
                max_col = *c;
            }
        }

        self.sprs.m = max_row + 1;
        self.sprs.n = max_col + 1;

        let mut entries: Vec<(usize, usize, Numeric)> = Vec::new();
        self.a_mat
            .iter()
            .for_each(|((row, col), val)| entries.push((*row, *col, *val)));

        entries.sort_unstable_by(
            |(r1, c1, _), (r2, c2, _)| {
                if c1 != c2 {
                    c1.cmp(c2)
                } else {
                    r1.cmp(r2)
                }
            },
        );

        self.sprs.nzmax = entries.len();
        self.sprs.p.resize(self.sprs.n + 1, 0);
        self.sprs.i.reserve(self.sprs.nzmax);
        self.sprs.x.reserve(self.sprs.nzmax);

        let mut current_col = 0;
        for (idx, (row, col, val)) in entries.into_iter().enumerate() {
            while col > current_col {
                self.sprs.p[current_col + 1] = idx as isize;
                current_col += 1;
            }
            self.sprs.i.push(row);
            self.sprs.x.push(val);
        }

        while current_col < self.sprs.n {
            self.sprs.p[current_col + 1] = self.sprs.nzmax as isize;
            current_col += 1;
        }
    }
}

fn ipvec(n: usize, p: &Option<Vec<isize>>, b: &[Numeric], x: &mut [Numeric]) {
    for k in 0..n {
        if p.is_some() {
            x[p.as_ref().expect("Permutation vector missing in ipvec. This indicates a solver internal error.")[k] as usize] = b[k];
        } else {
            x[k] = b[k];
        }
    }
}

#[cfg(test)]
impl RSparseSolver {
    /// Returns the length of the vector `b_vec`.
    pub fn b_vec_len(&self) -> usize {
        self.b_vec.len()
    }

    /// Returns the length of the vector `b_vec`.
    pub fn cplx_b_vec_len(&self) -> usize {
        self.cplx_b_vec.len()
    }

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat(&self) -> &HashMap<(usize, usize), Numeric> {
        &self.a_mat
    }

    /// Returns a reference to the matrix `a_mat`.
    pub fn a_mat_mut(&mut self) -> &mut HashMap<(usize, usize), Numeric> {
        &mut self.a_mat
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn b_vec(&self) -> &Vec<Numeric> {
        &self.b_vec
    }

    /// Returns a reference to the vector `b_vec`.
    pub fn x_vec(&self) -> &Vec<Numeric> {
        &self.x_vec
    }

    /// Returns a reference to the matrix `cplx_a_mat`.
    pub fn cplx_a_mat(&self) -> &HashMap<(usize, usize), ComplexNumeric> {
        &self.cplx_a_mat
    }

    /// Returns a reference to the vector `cplx_b_vec`.
    pub fn cplx_b_vec(&self) -> &Vec<Numeric> {
        &self.cplx_b_vec
    }

    /// Returns a reference to the vector `cplx_b_vec`.
    pub fn sprs(&self) -> &Sprs<Numeric> {
        &self.sprs
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
                warn!("Warning: Index out of bounds detected for element at index {k}. Skipping.",);
            }
        }

        // Print the matrix in a formatted way.
        for (row, _) in matrix.iter().enumerate().take(m) {
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
