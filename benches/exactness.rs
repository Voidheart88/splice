// Exactness benchmark for integrators
// This benchmark tests the numerical exactness of different integration methods
// against a golden sample (reference solution) rather than measuring performance
//
// The benchmark uses a simple RC circuit with known analytical solution:
// - RC circuit with step input
// - Analytical solution: V(t) = V_final * (1 - exp(-t/RC))
// - Compares numerical results from different integrators against analytical solution
//
// Important: The step size (dt) should be chosen based on the system's dynamics
// For an RC circuit, the time constant tau = RC determines the system behavior.
// The step size should be a fraction of tau (typically dt < 0.1*tau for stability).
// For this circuit: tau = 1000 Ohm * 1e-9 F = 1e-6 s (1 microsecond)
//
// Run with: cargo bench --bench exactness

use std::sync::Arc;

use splice::frontends::create_simulation_from_spice;
use splice::run_sim_with_results;
use splice::sim::simulation_result::{Sim, SimulationResults};
use splice::solver::{FaerSolver, FaerSparseSolver, NalgebraSolver, RSparseSolver};

/// Calculate the analytical solution for RC step response using higher precision golden sample.
/// Uses f64 with the standard library's exp() function which provides ~15-17 decimal digits
/// of precision - sufficient as a reference for f64 numerical results.
/// V(t) = V_final * (1 - exp(-t/RC))
fn rc_analytical_solution_golden(t: f64, r: f64, c: f64, v_final: f64) -> f64 {
    let tau = r * c;
    v_final * (1.0 - (-t / tau).exp())
}

/// Extract voltage at a specific time from simulation results
fn extract_voltage_at_time(results: &SimulationResults) -> Option<f64> {
    for result in &results.results {
        match result {
            Sim::Tran(data) => {
                // Find the result closest to the requested time (10us - final time)
                let target_time = 10e-6;
                let mut best_voltage: Option<f64> = None;
                let mut best_error = f64::MAX;
                
                for (t, vars) in data {
                    let error = (*t - target_time).abs();
                    if error < best_error {
                        best_error = error;
                        // Look for V(1) - the voltage at node 1
                        for (var, val) in vars {
                            if var.name() == Arc::from("1") && var.unit() == splice::models::Unit::Volt {
                                best_voltage = Some(*val);
                                break;
                            }
                        }
                    }
                }
                if best_voltage.is_some() {
                    return best_voltage;
                }
            }
            _ => {}
        }
    }
    None
}

/// Generate SPICE code for a simple RC circuit with step response
/// This circuit has a known analytical solution
/// Using STEP modifier for voltage source (matches test_step.cir format)
fn generate_rc_step_circuit(resistance: f64, capacitance: f64, timestep: f64, stop_time: f64) -> String {
    // Create an RC circuit with a step input
    // V1 0 1 STEP 0 5 1e-9  (step from 0V to 5V with 1ns step time)
    // R1 1 2 {resistance}
    // C1 2 0 {capacitance}
    // .tran {timestep} {stop_time}  (transient analysis)
    
    let spice_code = format!(
        r#"
V1 0 1 STEP 0 5 1e-9
R1 1 2 {resistance}
C1 2 0 {capacitance}
.tran {timestep} {stop_time}
"#
    )
    .replace("{resistance}", &resistance.to_string())
    .replace("{capacitance}", &capacitance.to_string())
    .replace("{timestep}", &timestep.to_string())
    .replace("{stop_time}", &stop_time.to_string());
    
    spice_code
}

/// Run exactness test for a single solver
/// Returns the error between numerical and analytical solution
fn run_exactness_test(
    spice_code: &str,
    solver_type: &str,
    expected_voltage: f64,
) -> Result<f64, String> {
    let network = create_simulation_from_spice(spice_code)
        .map_err(|e| format!("{}: Failed to parse RC circuit: {:?}", solver_type, e))?;
    
    let results = match solver_type {
        "Nalgebra" => run_sim_with_results::<NalgebraSolver>(network),
        "Faer" => run_sim_with_results::<FaerSolver>(network),
        "FaerSparse" => run_sim_with_results::<FaerSparseSolver>(network),
        "RSparse" => run_sim_with_results::<RSparseSolver>(network),
        _ => return Err(format!("{}: Unknown solver", solver_type)),
    };
    
    let results = results?;
    
    // Extract the voltage at t=stop_time (last time point)
    if let Some(actual_voltage) = extract_voltage_at_time(&results) {
        let error = (actual_voltage - expected_voltage).abs();
        Ok(error)
    } else {
        Err(format!("{}: Could not extract voltage", solver_type))
    }
}

/// Main function for exactness benchmark
/// This runs all solver/integration method combinations and reports results
fn main() {
    println!("=== Exactness Benchmark for Integrators ===\n");
    println!("Testing numerical accuracy of splice integrators against known solutions");
    println!("Benchmark configuration: RC circuit with step response\n");

    let resistance = 1000.0;  // 1 kOhm
    let capacitance = 1e-9;   // 1 nF
    let tau = resistance * capacitance;  // 1 microsecond
    
    // System analysis:
    // - Time constant tau = R * C = 1000 * 1e-9 = 1e-6 s (1 microsecond)
    // - The step response time constant determines the required resolution
    // - For accurate integration: dt < 0.1 * tau is recommended
    // - This gives dt < 100 ns for this circuit
    // - We test multiple step sizes to show the accuracy vs dt tradeoff
    
    // Expected final voltage at t=10e-6 (many time constants - steady state)
    let v_final = 5.0;
    let t_test = 10e-6;
    
    // Calculate expected voltage using golden sample (f64 exp() provides ~15-17 decimal digits)
    let expected_voltage_golden = rc_analytical_solution_golden(t_test, resistance, capacitance, v_final);
    
    println!("Circuit parameters:");
    println!("  R = {} Ohm", resistance);
    println!("  C = {} F", capacitance);
    println!("  Time constant tau = R*C = {} s", tau);
    println!("\nRecommended step size: dt < 0.1 * tau = {} s", 0.1 * tau);
    println!("  (Step size should be a fraction of the system's time constant)");
    
    println!("\nExpected voltage at t={} s:", t_test);
    println!("  golden (f64 exp, ~17 digits): {:.17} V", expected_voltage_golden);
    
    // Test different step sizes to show how accuracy changes
    // dt values relative to tau (1e-6 s):
    // - dt = 0.01 * tau = 10 ns (very fine)
    // - dt = 0.1 * tau = 100 ns (reasonable)
    // - dt = 1.0 * tau = 1 us (coarse)
    // - dt = 10 * tau = 10 us (very coarse - may be unstable/expensive)
    
    let step_sizes = [
        (10e-9, "dt = 0.01 * tau (10 ns) - very fine"),
        (100e-9, "dt = 0.1 * tau (100 ns) - reasonable"),
        (1e-6, "dt = 1.0 * tau (1 us) - coarse"),
        (10e-6, "dt = 10 * tau (10 us) - very coarse"),
    ];
    
    let solvers = [
        "Nalgebra",
        "Faer",
        "FaerSparse",
        "RSparse",
    ];
    
    println!("\nRunning exactness tests with different step sizes...\n");
    
    for &(tstep, description) in &step_sizes {
        println!("Step size: {} ({})", tstep, description);
        let spice_code = generate_rc_step_circuit(resistance, capacitance, tstep, t_test);
        
        for solver in &solvers {
            match run_exactness_test(&spice_code, solver, expected_voltage_golden) {
                Ok(error) => {
                    println!("  [OK] {}-BackwardEuler: Error = {:.17e} V", solver, error);
                }
                Err(e) => {
                    println!("  [FAIL] {}-BackwardEuler: {}", solver, e);
                }
            }
        }
        println!();
    }

    println!("\nExactness is verified by comparing against analytical RC response:");
    println!("  V(t) = V_final * (1 - exp(-t/RC))");
}
