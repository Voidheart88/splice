// Error benchmark runner
// This benchmark tests the numerical accuracy of different integration methods
// against a golden sample (reference solution) rather than measuring performance
//
// The benchmark uses a simple RC circuit with known analytical solution:
// - RC circuit with step input
// - Analytical solution: V(t) = V_final * (1 - exp(-t/RC))
// - Compares numerical results from different integrators against analytical solution
// - Uses Sum of Squared Errors (SSE) over the entire time series
//
// Important: The step size (dt) should be chosen based on the system's dynamics
// For an RC circuit, the time constant tau = RC determines the system behavior.
// The step size should be a fraction of tau (typically dt < 0.1*tau for stability).
// For this circuit: tau = 1000 Ohm * 100e-9 F = 100e-6 s (100 microseconds)
//
// Run with: cargo bench --bench error

mod nalgebra_error;
mod faer_error;
mod faer_sparse_error;
mod rsparse_error;

use nalgebra_error::{run_nalgebra_error_test, rc_analytical_solution_golden, generate_rc_step_circuit};
use faer_error::run_faer_error_test;
use faer_sparse_error::run_faer_sparse_error_test;
use rsparse_error::run_rsparse_error_test;

/// Benchmark configuration parameters
/// These match test_step.cir: R=1kOhm, C=100nF, V=10V step
/// Time constant tau = R * C = 1000 * 100e-9 = 100e-6 = 100 microseconds
const RESISTANCE: f64 = 1000.0;  // 1 kOhm
const CAPACITANCE: f64 = 100e-9; // 100 nF
const V_FINAL: f64 = 10.0;       // 10V step (matches test_step.cir)
const T_TEST: f64 = 10e-3;       // 10ms (many time constants - steady state)

/// Calculate Sum of Squared Errors (SSE) for a solver
/// This provides a better measure of overall accuracy than a single-point error
fn run_solver_test_sse(solver_name: &str, runner: impl Fn(&str, f64, f64, f64) -> Result<f64, String>, timestep: f64) {
    let spice_code = generate_rc_step_circuit(RESISTANCE, CAPACITANCE, timestep, T_TEST, V_FINAL);
    
    match runner(&spice_code, RESISTANCE, CAPACITANCE, V_FINAL) {
        Ok(sse) => {
            // Convert SSE to Root Mean Squared Error (RMSE) for easier interpretation
            // RMSE = sqrt(SSE / n) where n is number of time steps
            // We estimate n as T_TEST / timestep
            let n = T_TEST / timestep;
            let rmse = (sse / n).sqrt();
            println!("  [OK] {}-BackwardEuler: SSE = {:.3e}, RMSE = {:.3e} V", solver_name, sse, rmse);
        }
        Err(e) => {
            println!("  [FAIL] {}-BackwardEuler: {}", solver_name, e);
        }
    }
}

/// Main function for error benchmark
/// This runs all solver/integration method combinations and reports results
fn main() {
    println!("=== Error Benchmark for Integrators ===\n");
    println!("Testing numerical accuracy of splice integrators against known solutions");
    println!("Benchmark configuration: RC circuit with step response\n");
    println!("Uses Sum of Squared Errors (SSE) over entire time series\n");

    let tau = RESISTANCE * CAPACITANCE;  // 100 microseconds (0.0001 s)
    
    // System analysis:
    // - Time constant tau = R * C = 1000 * 100e-9 = 100e-6 s (100 microseconds)
    // - The step response time constant determines the required resolution
    // - For accurate integration: dt < 0.1 * tau is recommended
    // - This gives dt < 10 us for this circuit
    // - We test multiple step sizes to show the accuracy vs dt tradeoff
    
    // Expected final voltage at t=10ms (many time constants - steady state)
    let expected = rc_analytical_solution_golden(T_TEST, RESISTANCE, CAPACITANCE, V_FINAL);
    
    println!("Circuit parameters:");
    println!("  R = {} Ohm", RESISTANCE);
    println!("  C = {} F", CAPACITANCE);
    println!("  Time constant tau = R*C = {} s", tau);
    println!("\nRecommended step size: dt < 0.1 * tau = {} s", 0.1 * tau);
    println!("  (Step size should be a fraction of the system's time constant)");
    
    println!("\nExpected voltage at t={} s:", T_TEST);
    println!("  golden (f64 exp, ~17 digits): {:.17} V", expected);
    
    // Test different step sizes to show how accuracy changes
    // dt values relative to tau (100e-6 s = 100 us):
    // - dt = 0.01 * tau = 1 us (very fine)
    // - dt = 0.1 * tau = 10 us (reasonable)
    // - dt = 1.0 * tau = 100 us (coarse)
    // - dt = 10 * tau = 1 ms (very coarse - may be unstable/expensive)
    
    let step_sizes = [
        (1e-6, "dt = 0.01 * tau (1 us) - very fine"),
        (10e-6, "dt = 0.1 * tau (10 us) - reasonable"),
        (100e-6, "dt = 1.0 * tau (100 us) - coarse"),
        (1e-3, "dt = 10 * tau (1 ms) - very coarse"),
    ];
    
    println!("\nStep sizes relative to tau ({} s):", tau);
    
    println!("\nRunning error tests with different step sizes...\n");
    
    for &(tstep, description) in &step_sizes {
        println!("Step size: {} ({})", tstep, description);
        
        // Run each solver with the current step size
        run_solver_test_sse("Nalgebra", run_nalgebra_error_test, tstep);
        run_solver_test_sse("Faer", run_faer_error_test, tstep);
        run_solver_test_sse("FaerSparse", run_faer_sparse_error_test, tstep);
        run_solver_test_sse("RSparse", run_rsparse_error_test, tstep);
        
        println!();
    }

    println!("Error is verified by comparing against analytical RC response:");
    println!("  V(t) = V_final * (1 - exp(-t/RC))");
}
