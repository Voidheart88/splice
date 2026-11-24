// Error benchmark for Faer solver

use std::sync::Arc;

use splice::frontends::create_simulation_from_spice;
use splice::run_sim_with_results;
use splice::sim::simulation_result::{Sim, SimulationResults};
use splice::solver::FaerSolver;

/// Calculate the analytical solution for RC step response using higher precision golden sample.
/// Uses f64 with the standard library's exp() function which provides ~15-17 decimal digits
/// of precision - sufficient as a reference for f64 numerical results.
/// V(t) = V_final * (1 - exp(-t/RC))
/// Note: Constants are defined in error.rs (RESISTANCE=1000, CAPACITANCE=100e-9, V_FINAL=10)
pub fn rc_analytical_solution_golden(t: f64, r: f64, c: f64, v_final: f64) -> f64 {
    let tau = r * c;
    v_final * (1.0 - (-t / tau).exp())
}

/// Calculate Sum of Squared Errors (SSE) between numerical and analytical solution
/// over the entire time series
/// Note: This measures the error at node 2 (capacitor voltage), not node 1 (input voltage)
fn calculate_sse(results: &SimulationResults, r: f64, c: f64, v_final: f64) -> Option<f64> {
    for result in &results.results {
        match result {
            Sim::Tran(data) => {
                let mut sse = 0.0;
                let mut count = 0;
                
                for (t, vars) in data {
                    // Calculate analytical solution at this time
                    let analytical = rc_analytical_solution_golden(*t, r, c, v_final);
                    
                    // Find voltage at node 2 (capacitor voltage)
                    for (var, val) in vars {
                        if var.name() == Arc::from("2") && var.unit() == splice::models::Unit::Volt {
                            let error = val - analytical;
                            sse += error * error;
                            count += 1;
                            break;
                        }
                    }
                }
                
                if count > 0 {
                    return Some(sse);
                }
                return None;
            }
            _ => {}
        }
    }
    None
}

/// Generate SPICE code for a simple RC circuit with step response
/// This circuit has a known analytical solution
/// Using STEP modifier for voltage source (matches test_step.cir format)
/// Note: Constants are defined in error.rs (RESISTANCE=1000, CAPACITANCE=100e-9, V_FINAL=10)
#[allow(dead_code)]
pub fn generate_rc_step_circuit(resistance: f64, capacitance: f64, timestep: f64, stop_time: f64, v_final: f64) -> String {
    // Create an RC circuit with a step input
    // V1 0 1 STEP 0 V_FINAL 1e-9  (step from 0V to V_FINAL with 1ns step time)
    // R1 1 2 {resistance}        (resistor from node 1 to node 2)
    // C1 2 0 {capacitance}       (capacitor from node 2 to ground)
    // .tran {timestep} {stop_time}  (transient analysis)
    
    let spice_code = format!(
        r#"
V1 0 1 STEP 0 {v_final} 1e-9
R1 1 2 {resistance}
C1 2 0 {capacitance}
.tran {timestep} {stop_time}
"#
    )
    .replace("{resistance}", &resistance.to_string())
    .replace("{capacitance}", &capacitance.to_string())
    .replace("{timestep}", &timestep.to_string())
    .replace("{stop_time}", &stop_time.to_string())
    .replace("{v_final}", &v_final.to_string());
    
    spice_code
}

/// Run exactness test for Faer solver
/// Returns the Sum of Squared Errors (SSE) between numerical and analytical solution
pub fn run_faer_error_test(spice_code: &str, r: f64, c: f64, v_final: f64) -> Result<f64, String> {
    let network = create_simulation_from_spice(spice_code)
        .map_err(|e| format!("Faer: Failed to parse RC circuit: {:?}", e))?;
    
    let results = run_sim_with_results::<FaerSolver>(network)
        .map_err(|e| format!("Faer: Simulation failed: {:?}", e))?;
    
    // Calculate SSE over the entire time series
    if let Some(sse) = calculate_sse(&results, r, c, v_final) {
        Ok(sse)
    } else {
        Err("Faer: Could not calculate SSE".to_string())
    }
}
