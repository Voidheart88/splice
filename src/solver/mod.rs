pub(crate) mod faer;
pub(crate) mod nalgebra;
pub(crate) mod rsparse;

use clap::ValueEnum;
use miette::Diagnostic;
use thiserror::Error;

pub(crate) use faer::FaerSolver;
pub(crate) use nalgebra::NalgebraSolver;
pub(crate) use rsparse::RSparseSolver;

use crate::spot::{ComplexNumeric, Numeric};

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

/// Trait defining the Solver interface.
pub trait Solver {
    /// Creates a new instance of the Solver with the given number of variables.
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized;

    /// Initialize the matrices and vectors to get the sparsity pattern.
    fn init(
        &mut self,
        a_matrix: Vec<(usize, usize)>,
        cplx_a_matrix: Vec<(usize, usize)>,
    );

    /// Inserts a Value into the conductance matrix (`a`) of the Solver.
    fn insert_a(&mut self, a_trpl: &(usize, usize, Numeric));

    /// Inserts a Value into the  known values Vector (`a`) of the Solver.
    fn insert_b(&mut self, b_pair: &(usize, Numeric));

    /// Inserts a Value into the conductance matrix (`a`) of the Solver.
    fn insert_cplx_a(&mut self, a_trpl: &(usize, usize, ComplexNumeric));

    /// Inserts a Value into the  known values Vector (`a`) of the Solver.
    fn insert_cplx_b(&mut self, b_pair: &(usize, ComplexNumeric));

    fn solve(&mut self) -> Result<&Vec<Numeric>, SolverError>;

    fn solve_cplx(&mut self) -> Result<&Vec<ComplexNumeric>, SolverError>;
}

#[cfg(test)]
mod tests;
