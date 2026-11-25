/// Intelligent solver selection based on circuit characteristics
use super::{FaerSparseSolver, NalgebraSolver, Solver, SolverError};

/// Solver selection strategy based on circuit size and characteristics
#[derive(Debug, Clone, Copy)]
pub enum SolverSelectionStrategy {
    /// Automatically select the best solver based on circuit size
    Automatic,
    /// Use sparse solver for large circuits, dense for small
    Hybrid,
}

/// Intelligent solver selector
pub struct SolverSelector {
    strategy: SolverSelectionStrategy,
    circuit_size_threshold: usize,
}

impl SolverSelector {
    /// Create a new solver selector with default strategy
    pub fn new(strategy: SolverSelectionStrategy) -> Self {
        SolverSelector {
            strategy,
            circuit_size_threshold: crate::spot::SOLVER_SELECTION_THRESHOLD,
        }
    }

    /// Create a new solver selector with custom threshold
    pub fn with_threshold(strategy: SolverSelectionStrategy, threshold: usize) -> Self {
        SolverSelector {
            strategy,
            circuit_size_threshold: threshold,
        }
    }

    /// Select the appropriate solver based on circuit size
    pub fn select_solver(&self, vars: usize) -> Result<Box<dyn Solver>, SolverError> {
        match self.strategy {
            SolverSelectionStrategy::Automatic => self.select_best_solver(vars),
            SolverSelectionStrategy::Hybrid if vars <= self.circuit_size_threshold => {
                Ok(Box::new(NalgebraSolver::new(vars)?))
            }
            SolverSelectionStrategy::Hybrid => Ok(Box::new(FaerSparseSolver::new(vars)?)),
        }
    }

    /// Select the best solver based on performance data
    fn select_best_solver(&self, vars: usize) -> Result<Box<dyn Solver>, SolverError> {
        // Based on our performance testing:
        // - Small circuits (<10 vars): Nalgebra is fastest
        // - Medium to large circuits (>=10 vars): FaerSparse dominates
        if vars < 10 {
            Ok(Box::new(NalgebraSolver::new(vars)?))
        } else {
            Ok(Box::new(FaerSparseSolver::new(vars)?))
        }
    }

    /// Get a recommendation for solver selection
    pub fn recommend_solver(&self, vars: usize) -> &'static str {
        match self.strategy {
            SolverSelectionStrategy::Automatic if vars < 10 => {
                "NalgebraSolver (best for very small circuits)"
            }
            SolverSelectionStrategy::Automatic if vars < 100 => {
                "FaerSparseSolver (best for small-medium circuits)"
            }
            SolverSelectionStrategy::Automatic => "FaerSparseSolver (best for large circuits)",
            SolverSelectionStrategy::Hybrid if vars <= self.circuit_size_threshold => {
                "NalgebraSolver (hybrid strategy: dense for small)"
            }
            SolverSelectionStrategy::Hybrid => "FaerSparseSolver (hybrid strategy: sparse for large)",
        }
    }
}
