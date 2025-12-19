/// Intelligent solver selection based on circuit characteristics
use super::{FaerSparseSolver, NalgebraSolver, Solver, SolverError};
use crate::spot::{DEFAULT_SOLVER_THRESHOLD, MEDIUM_CIRCUIT_THRESHOLD, SMALL_CIRCUIT_THRESHOLD};

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
            circuit_size_threshold: DEFAULT_SOLVER_THRESHOLD,
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
            SolverSelectionStrategy::Automatic => {
                // Automatic selection based on performance characteristics
                self.select_best_solver(vars)
            }
            SolverSelectionStrategy::Hybrid => {
                // Hybrid approach: sparse for large, dense for small
                if vars <= self.circuit_size_threshold {
                    // For small circuits, use Nalgebra (fastest for small systems)
                    Ok(Box::new(NalgebraSolver::new(vars)?))
                } else {
                    // For large circuits, use FaerSparse (best scaling)
                    Ok(Box::new(FaerSparseSolver::new(vars)?))
                }
            }
        }
    }

    /// Select the best solver based on performance data
    fn select_best_solver(&self, vars: usize) -> Result<Box<dyn Solver>, SolverError> {
        // Based on our performance testing:
        // - Small circuits (<10 vars): Nalgebra is fastest
        // - Medium circuits (10-100 vars): FaerSparse is best
        // - Large circuits (>100 vars): FaerSparse dominates

        if vars < SMALL_CIRCUIT_THRESHOLD {
            // Very small circuits: Nalgebra is fastest
            Ok(Box::new(NalgebraSolver::new(vars)?))
        } else if vars < MEDIUM_CIRCUIT_THRESHOLD {
            // Small to medium circuits: FaerSparse is best
            Ok(Box::new(FaerSparseSolver::new(vars)?))
        } else {
            // Large circuits: FaerSparse is significantly better
            Ok(Box::new(FaerSparseSolver::new(vars)?))
        }
    }

    /// Get a recommendation for solver selection
    pub fn recommend_solver(&self, vars: usize) -> &'static str {
        match self.strategy {
            SolverSelectionStrategy::Automatic => {
                if vars < SMALL_CIRCUIT_THRESHOLD {
                    "NalgebraSolver (best for very small circuits)"
                } else if vars < MEDIUM_CIRCUIT_THRESHOLD {
                    "FaerSparseSolver (best for small-medium circuits)"
                } else {
                    "FaerSparseSolver (best for large circuits)"
                }
            }
            SolverSelectionStrategy::Hybrid => {
                if vars <= self.circuit_size_threshold {
                    "NalgebraSolver (hybrid strategy: dense for small)"
                } else {
                    "FaerSparseSolver (hybrid strategy: sparse for large)"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automatic_solver_selection() {
        let selector = SolverSelector::new(SolverSelectionStrategy::Automatic);

        // Test that we can create solvers for different sizes
        // We can't easily check the type, but we can verify the selector works
        assert!(selector.select_solver(5).is_ok());
        assert!(selector.select_solver(50).is_ok());
        assert!(selector.select_solver(200).is_ok());
    }

    #[test]
    fn test_hybrid_solver_selection() {
        let selector = SolverSelector::new(SolverSelectionStrategy::Hybrid);

        // Test that we can create solvers for different sizes
        assert!(selector.select_solver(5).is_ok());
        assert!(selector.select_solver(200).is_ok());
    }

    #[test]
    fn test_custom_threshold() {
        let selector = SolverSelector::with_threshold(
            SolverSelectionStrategy::Hybrid,
            20, // Custom threshold
        );

        // Test that we can create solvers with custom threshold
        assert!(selector.select_solver(10).is_ok());
        assert!(selector.select_solver(30).is_ok());
    }

    #[test]
    fn test_solver_recommendation() {
        let selector = SolverSelector::new(SolverSelectionStrategy::Automatic);

        assert_eq!(
            selector.recommend_solver(5),
            "NalgebraSolver (best for very small circuits)"
        );

        assert_eq!(
            selector.recommend_solver(50),
            "FaerSparseSolver (best for small-medium circuits)"
        );

        assert_eq!(
            selector.recommend_solver(200),
            "FaerSparseSolver (best for large circuits)"
        );
    }
}
