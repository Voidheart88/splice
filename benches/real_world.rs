// Real-world circuit benchmarks
// This module contains performance benchmarks for realistic electronic circuits
// The Benchmarks are ran in the performance.rs module

use criterion::{Criterion, BenchmarkId};
use splice::{
    frontends::create_simulation_from_spice,
    run_sim_for_benchmark,
    solver::{FaerSolver, FaerSparseSolver, NalgebraSolver},
};

/// Generate SPICE code for a resistor ladder circuit
fn generate_resistor_ladder(n_resistors: usize) -> String {
    let mut spice_code = String::from("V1 1 0 10\n");

    for i in 1..=n_resistors {
        spice_code.push_str(&format!("R{i} {} {} 1000\n", i, i + 1));
    }
    spice_code.push_str(&format!("R{} {} 0 1000\n",n_resistors+1,n_resistors+1));
    
    spice_code.push_str(".op");
    spice_code
}

/// Generate SPICE code for a resistor network (grid)
fn generate_resistor_network(rows: usize, cols: usize) -> String {
    let mut spice_code = String::from("V1 1 0 5\n");
    
    // Create a simple grid: connect resistors between nodes
    let mut node_counter = 2; // Start after voltage source
    
    // Create rows
    for row in 0..rows {
        // Horizontal resistors in row
        for col in 0..cols {
            let current_node = if row == 0 && col == 0 { 1 } else { node_counter - 1 };
            let next_node = if col == cols - 1 { 0 } else { node_counter };
            
            if next_node != 0 { // Don't connect to ground with resistor
                spice_code.push_str(&format!("R{}x{} {} {} 1000\n", row, col, current_node, next_node));
                node_counter += 1;
            }
        }
        
        // Vertical resistors between rows (except last row)
        if row < rows - 1 {
            let start_node = if row == 0 { 1 } else { node_counter - cols };
            let end_node = node_counter;
            spice_code.push_str(&format!("Rxxvertxx{} {} {} 1000\n", row, start_node, end_node));
        }
    }
    
    spice_code.push_str(".op");
    spice_code
}

/// Benchmark resistor ladder circuits with different sizes
pub fn bench_resistor_ladder(c: &mut Criterion) {
    let mut group = c.benchmark_group("Resistor Ladder");
    let sizes = [10, 100, 1000];
    
    for &size in &sizes {
        let spice_code = generate_resistor_ladder(size);
        let network = create_simulation_from_spice(&spice_code).expect("Failed to parse resistor ladder");
        
        group.bench_with_input(BenchmarkId::new("Faer Sparse", size), &size, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<FaerSparseSolver>(network.clone()).expect("Simulation failed");
            });
        });
        
        group.bench_with_input(BenchmarkId::new("Faer", size), &size, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<FaerSolver>(network.clone()).expect("Simulation failed");
            });
        });
        
        group.bench_with_input(BenchmarkId::new("Nalgebra", size), &size, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<NalgebraSolver>(network.clone()).expect("Simulation failed");
            });
        });
    }
    
    group.finish();
}

/// Benchmark resistor network (grid) circuits
pub fn bench_resistor_network(c: &mut Criterion) {
    let mut group = c.benchmark_group("Resistor Network");
    
    // Test different grid sizes - keep them small for reasonable benchmark times
    let configurations = [(5, 5), (10, 10), (50, 50)];
    
    for &(rows, cols) in &configurations {
        let size_desc = format!("{rows}x{cols}");
        let spice_code = generate_resistor_network(rows, cols);
        
        // Parse the circuit once
        let network = create_simulation_from_spice(&spice_code).expect("Failed to parse resistor network");
        
        group.bench_with_input(BenchmarkId::new("Faer Sparse", &size_desc), &size_desc, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<FaerSparseSolver>(network.clone()).expect("Simulation failed");
            });
        });
        
        group.bench_with_input(BenchmarkId::new("Faer", &size_desc), &size_desc, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<FaerSolver>(network.clone()).expect("Simulation failed");
            });
        });
        
        group.bench_with_input(BenchmarkId::new("Nalgebra", &size_desc), &size_desc, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<NalgebraSolver>(network.clone()).expect("Simulation failed");
            });
        });
    }
    
    group.finish();
}