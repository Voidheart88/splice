pub(crate) mod nalgebra;
pub(crate) mod rsparse;

use clap::ValueEnum;
use derive_more::Deref;
use miette::Diagnostic;
use thiserror::Error;

pub(crate) use nalgebra::NalgebraBackend;
pub(crate) use rsparse::RSparseBackend;

use crate::models::{Doubles, Triples};

/// Errors that can occur in the backend.
#[derive(Debug, Error, Diagnostic, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackendError {
    /// Error indicating that the conductance matrix is not invertible.
    #[error("The conductance matrix is not invertible")]
    #[diagnostic(help(
        "This is the case when the Matrix is singular,\n
        which happens when, for example, two ideal voltage sources short each other."
    ))]
    MatrixNonInvertible,

    #[error("The Faer Backend is currently not implemented.")]
    #[diagnostic(help("Help by implementing this backend."))]
    Unimplemented,
}

#[derive(Copy, Clone, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Backends {
    RSparse,
    Nalgebra,
}

/// Type representing a row index.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Deref, Hash)]
pub struct Row(pub usize);

/// Type representing a column index.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Deref, Hash)]
pub struct Col(pub usize);

/// Trait defining the backend interface.
/// (unstable)
pub trait Backend {
    /// Creates a new instance of the backend with the given number of variables.
    fn new(vars: usize) -> Result<Self, BackendError>
    where
        Self: Sized;

    /// Sets the conductance matrix (`a`) into the backend.
    /// Set sets a Value to the given matrix i,j
    fn set_a(&mut self, a_mat: &Triples);

    /// Sets the known values vector (`b`) into the backend.
    /// Set sets a Value to the given vector i
    fn set_b(&mut self, b_vec: &Doubles);

    /// Inserts the conductance matrix (`a`) into the backend.
    /// Insert adds a Value to the given matrix i,j
    fn insert_a(&mut self, a_mat: &Triples);

    /// Inserts the known values vector (`b`) into the backend.
    /// Insert adds a Value to the given vector i
    fn insert_b(&mut self, b_vec: &Doubles);

    /// Solves the system of equations (Ax = B for x) and returns a referenze to the solution.
    fn solve(&mut self) -> Result<&Vec<f64>, BackendError>;
}

#[cfg(test)]
mod tests;
