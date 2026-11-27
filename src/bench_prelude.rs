//! Benchmark prelude module - common imports and utilities for benchmarks
//!
//! This module is feature-gated with `bench-internals` and should not be
//! included in public documentation.

pub use crate::frontends::create_simulation_from_spice;
pub use crate::frontends::serde::{SerdeCircuit, SerdeElement, SerdeOption, SerdeSimulation};
pub use crate::models::{Unit, Variable};
pub use crate::run_sim_for_benchmark;
pub use crate::run_sim_with_results;
pub use crate::solver::{Solver, FaerSolver, FaerSparseSolver, NalgebraSolver, RSparseSolver};
pub use crate::spot::Numeric;

use num::{One, Zero};
use rand::RngExt;
use rand::rng;
use std::collections::HashMap;

/// Generate a solvable linear system Ax = b for benchmarking solvers
///
/// Returns (A, b, x_true) where:
/// - A is a positive definite matrix constructed from L * L^T
/// - b = A * x_true
/// - x_true is the known solution
///
/// # Arguments
///
/// * `n` - Size of the system (n x n matrix)
/// * `density` - Sparsity density of the lower triangular matrix L (0.0 to 1.0)
///
/// # Example
///
/// ```ignore
/// let (a, b, x_true) = generate_solvable_system(10, 0.5);
/// ```
pub fn generate_solvable_system(
    n: usize,
    density: Numeric,
) -> (Vec<Vec<Numeric>>, Vec<Numeric>, Vec<Numeric>) {
    let mut rng = rng();

    let mut l_entries: HashMap<(usize, usize), f64> = HashMap::new();
    for i in 0..n {
        let diag_val = rng.random_range(0.5..2.0);
        l_entries.insert((i, i), diag_val);

        for j in 0..i {
            if rng.random::<Numeric>() < density {
                let val = rng.random_range(-Numeric::one()..Numeric::one());
                l_entries.insert((i, j), val);
            }
        }
    }

    let mut l = vec![vec![Numeric::zero(); n]; n];
    for (&(i, j), &val) in l_entries.iter() {
        l[i][j] = val;
    }

    let mut a = vec![vec![Numeric::zero(); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                a[i][j] += l[i][k] * l[j][k];
            }
        }
    }

    let x_true: Vec<Numeric> = (0..n).map(|_| rng.random_range(-5.0..5.0)).collect();

    let mut b = vec![Numeric::zero(); n];
    for i in 0..n {
        for (j, val) in x_true.iter().enumerate().take(n) {
            b[i] += a[i][j] * val;
        }
    }

    (a, b, x_true)
}

/// Generate a simple RC circuit for benchmarking
///
/// Returns a SerdeCircuit with:
/// - One voltage source (5V DC)
/// - One resistor (1kOhm)
/// - OP simulation
pub fn create_simple_rc_circuit() -> SerdeCircuit {
    use crate::models::resistor::serde::SerdeResistor;
    use crate::models::vsource::serde::SerdeVSource;

    SerdeCircuit {
        elements: vec![
            SerdeElement::Resistor(SerdeResistor {
                name: "R1".to_string(),
                node0: "n1".to_string(),
                node1: "0".to_string(),
                value: 1000.0,
            }),
            SerdeElement::VSource(SerdeVSource {
                name: "V1".to_string(),
                node0: "0".to_string(),
                node1: "n1".to_string(),
                value: 5.0,
                ac_value: None,
            }),
        ],
        simulations: vec![SerdeSimulation::OP],
        options: vec![SerdeOption {
            out: Some("n1".to_string()),
            integration: None,
            gmin: None,
            rshunt: None,
        }],
    }
}

/// Generate a medium-sized circuit with multiple components
///
/// # Arguments
///
/// * `size` - Number of resistors in the series chain
///
/// Returns a SerdeCircuit with:
/// - One voltage source (5V DC)
/// - `size` resistors in series (1kOhm each)
/// - One end resistor to ground
/// - OP simulation
pub fn create_medium_circuit(size: usize) -> SerdeCircuit {
    use crate::models::resistor::serde::SerdeResistor;
    use crate::models::vsource::serde::SerdeVSource;

    let mut elements = Vec::new();

    // Add voltage source
    elements.push(SerdeElement::VSource(SerdeVSource {
        name: "V1".to_string(),
        node0: "0".to_string(),
        node1: "n1".to_string(),
        value: 5.0,
        ac_value: None,
    }));

    // Add resistors in series
    for i in 1..=size {
        elements.push(SerdeElement::Resistor(SerdeResistor {
            name: format!("R{i}"),
            node0: if i == 1 {
                "n1".to_string()
            } else {
                format!("n{i}")
            },
            node1: format!("n{}", i + 1),
            value: 1000.0,
        }));
    }

    // Add end resistor to ground
    elements.push(SerdeElement::Resistor(SerdeResistor {
        name: "R_end".to_string(),
        node0: format!("n{}", size + 1),
        node1: "0".to_string(),
        value: 1000.0,
    }));

    SerdeCircuit {
        elements,
        simulations: vec![SerdeSimulation::OP],
        options: vec![SerdeOption {
            out: Some(format!("n{}", size + 1)),
            integration: None,
            gmin: None,
            rshunt: None,
        }],
    }
}

/// Generate a large resistor grid circuit for stress testing
///
/// # Arguments
///
/// * `rows` - Number of rows in the grid
/// * `cols` - Number of columns in the grid
///
/// Returns a SerdeCircuit with a resistor grid pattern
pub fn create_large_circuit(rows: usize, cols: usize) -> SerdeCircuit {
    use crate::models::resistor::serde::SerdeResistor;
    use crate::models::vsource::serde::SerdeVSource;

    let mut elements = Vec::new();

    // Add voltage source
    elements.push(SerdeElement::VSource(SerdeVSource {
        name: "V1".to_string(),
        node0: "0".to_string(),
        node1: "n1".to_string(),
        value: 5.0,
        ac_value: None,
    }));

    // Create resistor grid
    let mut node_counter = 2;
    for row in 0..rows {
        for col in 0..cols {
            let current_node = if row == 0 && col == 0 {
                1
            } else {
                node_counter - 1
            };
            let next_node = if col == cols - 1 { 0 } else { node_counter };

            if next_node != 0 {
                elements.push(SerdeElement::Resistor(SerdeResistor {
                    name: format!("R{}_{}", row, col),
                    node0: current_node.to_string(),
                    node1: next_node.to_string(),
                    value: 1000.0,
                }));
                node_counter += 1;
            }
        }

        // Vertical connection
        if row < rows - 1 {
            let start_node = if row == 0 { 1 } else { node_counter - cols };
            let end_node = node_counter;
            elements.push(SerdeElement::Resistor(SerdeResistor {
                name: format!("Rvert_{}", row),
                node0: start_node.to_string(),
                node1: end_node.to_string(),
                value: 1000.0,
            }));
        }
    }

    SerdeCircuit {
        elements,
        simulations: vec![SerdeSimulation::OP],
        options: vec![SerdeOption {
            out: Some("n1".to_string()),
            integration: None,
            gmin: None,
            rshunt: None,
        }],
    }
}

/// Generate an RC step response circuit for error benchmarking
///
/// Returns a SPICE circuit description with:
/// - One voltage source (10V DC step)
/// - One resistor (1kOhm)
/// - One capacitor (100nF)
///
/// # Arguments
///
/// * `r` - Resistance in Ohms (default: 1000)
/// * `c` - Capacitance in Farads (default: 100e-9)
/// * `v_final` - Final voltage in Volts (default: 10)
pub fn generate_rc_step_circuit(r: f64, c: f64, v_final: f64) -> String {
    format!(
        "V1 1 0 DC {}\nR1 1 2 {}\nC1 2 0 {}\n.tran 0.01 0.01",
        v_final, r, c
    )
}

/// Calculate the analytical solution for RC step response
///
/// Uses the standard formula: V(t) = V_final * (1 - exp(-t/RC))
///
/// # Arguments
///
/// * `t` - Time in seconds
/// * `r` - Resistance in Ohms
/// * `c` - Capacitance in Farads
/// * `v_final` - Final voltage in Volts
pub fn rc_analytical_solution_golden(t: f64, r: f64, c: f64, v_final: f64) -> f64 {
    let tau = r * c;
    v_final * (1.0 - (-t / tau).exp())
}
