// Solver backends - public for benchmarks with bench-internals feature
#[cfg(not(feature = "bench-internals"))]
pub mod faer;
#[cfg(not(feature = "bench-internals"))]
pub mod faer_sparse;
#[cfg(not(feature = "bench-internals"))]
pub mod nalgebra;
#[cfg(not(feature = "bench-internals"))]
pub mod rsparse;
#[cfg(not(feature = "bench-internals"))]
pub mod selector;

#[cfg(feature = "bench-internals")]
pub(crate) mod faer;
#[cfg(feature = "bench-internals")]
pub(crate) mod faer_sparse;
#[cfg(feature = "bench-internals")]
pub(crate) mod nalgebra;
#[cfg(feature = "bench-internals")]
pub(crate) mod rsparse;
#[cfg(feature = "bench-internals")]
pub(crate) mod selector;

use clap::ValueEnum;
use miette::Diagnostic;
use thiserror::Error;

// Re-exports gated by bench-internals feature
#[cfg(not(feature = "bench-internals"))]
pub use faer::FaerSolver;
#[cfg(not(feature = "bench-internals"))]
pub use faer_sparse::FaerSparseSolver;
#[cfg(not(feature = "bench-internals"))]
pub use nalgebra::NalgebraSolver;
#[cfg(not(feature = "bench-internals"))]
pub use rsparse::RSparseSolver;
#[cfg(not(feature = "bench-internals"))]
pub use selector::{SolverSelectionStrategy, SolverSelector};

#[cfg(feature = "bench-internals")]
pub(crate) use faer::FaerSolver;
#[cfg(feature = "bench-internals")]
pub(crate) use faer_sparse::FaerSparseSolver;
#[cfg(feature = "bench-internals")]
pub(crate) use nalgebra::NalgebraSolver;
#[cfg(feature = "bench-internals")]
pub(crate) use rsparse::RSparseSolver;
#[cfg(feature = "bench-internals")]
pub(crate) use selector::{SolverSelectionStrategy, SolverSelector};

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
    FaerSparse,
}

/// Trait defining the Solver interface.
pub trait Solver {
    /// Creates a new instance of the Solver with the given number of variables.
    fn new(vars: usize) -> Result<Self, SolverError>
    where
        Self: Sized;

    /// Initialize the matrices and vectors to get the sparsity pattern.
    fn init(&mut self, a_matrix: Vec<(usize, usize)>, cplx_a_matrix: Vec<(usize, usize)>);

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

    /// Resets the solver to a clean state, clearing all matrices and vectors.
    fn reset(&mut self);
}

#[cfg(test)]
mod tests;
