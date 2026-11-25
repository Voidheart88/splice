use crate::solver::{SolverSelectionStrategy, SolverSelector};

#[test]
fn test_automatic_solver_selection() {
    let selector = SolverSelector::new(SolverSelectionStrategy::Automatic);

    // Test that we can create solvers for different sizes
    // We can't easily check the type, but we can verify the selector works
    // CHECK: Do these thresholds should be located in spot.rs
    assert!(selector.select_solver(5).is_ok());
    assert!(selector.select_solver(50).is_ok());
    assert!(selector.select_solver(200).is_ok());
}

#[test]
fn test_hybrid_solver_selection() {
    let selector = SolverSelector::new(SolverSelectionStrategy::Hybrid);

    // Test that we can create solvers for different sizes
    // CHECK: Do these thresholds should be located in spot.rs
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
