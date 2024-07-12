pub(crate) mod faer;
pub(crate) mod nalgebra;
pub(crate) mod rsparse;

use clap::ValueEnum;
use derive_more::Deref;
use miette::Diagnostic;
use thiserror::Error;

pub(crate) use faer::FaerSolver;
pub(crate) use nalgebra::NalgebraSolver;
pub(crate) use rsparse::RSparseSolver;

use crate::models::{Pairs, Triples};

/// Errors that can occur in the Solver.
#[derive(Debug, Error, Diagnostic, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SolverError {
    /// Error indicating that the conductance matrix is not invertible.
    #[error("The conductance matrix is not invertible")]
    #[diagnostic(help(
        "This is the case when the Matrix is singular,\n
        which happens when, for example, two ideal voltage sources short each other."
    ))]
    MatrixNonInvertible,
}

#[derive(Copy, Clone, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Solvers {
    Rsparse,
    Nalgebra,
    Faer,
}

/// Type representing a row index.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Deref, Hash)]
pub struct Row(pub usize);

/// Type representing a column index.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Deref, Hash)]
pub struct Col(pub usize);

/// Trait defining the Solver interface.
pub trait Solver {
    /// Creates a new instance of the Solver with the given number of variables.
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized;

    /// Sets the conductance matrix (`a`) into the Solver.
    /// Set sets a Value to the given matrix i,j
    fn set_a(&mut self, a_mat: &Triples);

    /// Sets the known values vector (`b`) into the Solver.
    /// Set sets a Value to the given vector i
    fn set_b(&mut self, b_vec: &Pairs);

    /// Inserts the conductance matrix (`a`) into the Solver.
    /// Insert adds a Value to the given matrix i,j
    fn insert_a(&mut self, a_mat: &Triples);

    /// Inserts the known values vector (`b`) into the Solver.
    /// Insert adds a Value to the given vector i
    fn insert_b(&mut self, b_vec: &Pairs);

    /// Solves the system of equations (Ax = B for x) and returns a referenze to the solution.
    /// Since there are solve in place Solvers, Ax = B can change b!
    ///
    fn solve(&mut self) -> Result<&Vec<f64>, SolverError>;
}

#[cfg(test)]
mod tests;
