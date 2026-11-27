#![deny(unsafe_code)]

//! Splice - A blazingly fast circuit simulator
//!
//! This library provides circuit simulation capabilities with multiple solver backends.
//!
//! # Modules
//!
//! - `frontends` - Input parsers (SPICE, YAML, JSON)
//! - `models` - Circuit element implementations
//! - `sim` - Simulation engine and integration methods
//! - `solver` - Linear algebra solver backends
//! - `spot` - Type aliases and physical constants
//!
//! # Public API for Benchmarks
//!
//! The following functions are exposed for benchmarking (marked with `#[doc(hidden)]`):
//! - [`run_sim_for_benchmark()`] - Run simulation without returning results
//! - [`run_sim_with_results()`] - Run simulation and return results
//!
//! # Feature Flags
//!
//! - `bench-internals` - Enables benchmark utilities via [`bench_prelude`]

pub mod backends;
pub mod frontends;
pub(crate) mod guesser;
pub mod models;
pub mod sim;
pub mod solver;
pub mod spot;

// Re-exports for internal use across modules
pub use backends::{Backend, BackendError};
pub use frontends::{Frontend, FrontendError};
pub use models::{Element, Variable};

use frontends::Simulation;
use sim::simulation_result::SimulationResults;
use solver::Solver;

/// Run a simulation with a specific solver for benchmarking
///
/// This function is exposed for benchmarking purposes and returns a simple Result.
/// It runs the simulation to completion without returning detailed results.
///
/// # Type Parameters
///
/// * `T` - The solver type (must implement [`Solver`])
///
/// # Example
///
/// ```ignore
/// use splice::{run_sim_for_benchmark, solver::FaerSolver, frontends::Simulation};
///
/// let sim: Simulation = /* ... */;
/// run_sim_for_benchmark::<FaerSolver>(sim)?;
/// ```
#[doc(hidden)]
pub fn run_sim_for_benchmark<T: Solver>(sim: Simulation) -> Result<(), String> {
    let mut sim: Simulator<T> = Simulator::from(sim);
    match sim.run() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

/// Run a simulation with a specific solver and return results
///
/// This function is exposed for benchmarking purposes to get detailed simulation results.
/// It returns the complete [`SimulationResults`] for analysis.
///
/// # Type Parameters
///
/// * `T` - The solver type (must implement [`Solver`])
///
/// # Example
///
/// ```ignore
/// use splice::{run_sim_with_results, solver::FaerSolver, frontends::Simulation};
///
/// let sim: Simulation = /* ... */;
/// let results = run_sim_with_results::<FaerSolver>(sim)?;
/// ```
#[doc(hidden)]
pub fn run_sim_with_results<T: Solver>(sim: Simulation) -> Result<SimulationResults, String> {
    let mut sim: Simulator<T> = Simulator::from(sim);
    match sim.run() {
        Ok(results) => Ok(results),
        Err(e) => Err(format!("{:?}", e)),
    }
}

// Internal helper - not part of public API
use sim::Simulator;

#[cfg(feature = "bench-internals")]
#[doc(hidden)]
pub mod bench_prelude;
