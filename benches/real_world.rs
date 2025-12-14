// Real-world circuit benchmarks
// This module contains performance benchmarks for realistic electronic circuits

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use splice::{
    frontends::create_simulation_from_spice,
    run_sim_for_benchmark,
    solver::{RSparseSolver, FaerSolver, NalgebraSolver},
};

/// Generate SPICE code for a resistor ladder circuit
fn generate_resistor_ladder(n_resistors: usize) -> String {
    let mut spice_code = String::from(".circuit\nV1 1 0 DC 10\n");
    
    // Create resistor chain: 1 -> 2 -> 3 -> ... -> n -> 0
    for i in 1..=n_resistors {
        spice_code.push_str(&format!("R{i} {} {} 1000", i, i + 1));
        if i < n_resistors {
            spice_code.push('\n');
        }
    }
    
    spice_code.push_str("\n.end");
    spice_code
}

/// Benchmark resistor ladder circuits with different sizes
pub fn bench_resistor_ladder(c: &mut Criterion) {
    let mut group = c.benchmark_group("Resistor Ladder");
    
    // Test different circuit sizes
    let sizes = [10, 100, 1000];
    
    for &size in &sizes {
        let spice_code = generate_resistor_ladder(size);
        
        // Parse the circuit once to avoid parsing overhead in benchmark
        let network = create_simulation_from_spice(&spice_code).expect("Failed to parse resistor ladder");
        
        group.bench_with_input(BenchmarkId::new("RSparse", size), &size, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<RSparseSolver>(network.clone()).expect("Simulation failed");
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

/// Generate SPICE code for a resistor network (grid)
fn generate_resistor_network(rows: usize, cols: usize) -> String {
    let mut spice_code = String::from(".circuit\nV1 1 0 DC 5\n");
    
    // Create a grid of resistors
    let mut node_counter = 2; // Start after voltage source
    
    // Create rows
    for row in 0..rows {
        // Horizontal resistors in row
        for col in 0..cols {
            let current_node = if row == 0 && col == 0 { 1 } else { node_counter - 1 };
            let next_node = if col == cols - 1 { 0 } else { node_counter };
            
            if next_node != 0 { // Don't connect to ground with resistor
                spice_code.push_str(&format!("R{}_{} {} {} 1k\n", row, col, current_node, next_node));
                node_counter += 1;
            }
        }
        
        // Vertical resistors between rows (except last row)
        if row < rows - 1 {
            let start_node = if row == 0 { 1 } else { node_counter - cols };
            let end_node = node_counter;
            spice_code.push_str(&format!("R_vert_{} {} {} 1k\n", row, start_node, end_node));
        }
    }
    
    spice_code.push_str("\n.end");
    spice_code
}

/// Benchmark resistor network (grid) circuits
pub fn bench_resistor_network(c: &mut Criterion) {
    let mut group = c.benchmark_group("Resistor Network");
    
    // Test different grid sizes
    let configurations = [(2, 2), (5, 5), (10, 10)];
    
    for &(rows, cols) in &configurations {
        let size_desc = format!("{rows}x{cols}");
        let spice_code = generate_resistor_network(rows, cols);
        
        // Parse the circuit once
        let network = create_simulation_from_spice(&spice_code).expect("Failed to parse resistor network");
        
        group.bench_with_input(BenchmarkId::new("RSparse", &size_desc), &size_desc, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<RSparseSolver>(network.clone()).expect("Simulation failed");
            });
        });
    }
    
    group.finish();
}

/// Generate SPICE code for a voltage divider
fn generate_voltage_divider(n_stages: usize) -> String {
    let mut spice_code = String::from(".circuit\nV1 1 0 DC 12\n");
    
    let mut current_node = 1;
    for i in 1..=n_stages {
        let next_node = i + 1;
        spice_code.push_str(&format!("R{i}_top {} {} 10k\n", current_node, next_node));
        spice_code.push_str(&format!("R{i}_bottom {} 0 10k\n", next_node));
        current_node = next_node;
    }
    
    spice_code.push_str("\n.end");
    spice_code
}

/// Benchmark voltage divider circuits
pub fn bench_voltage_divider(c: &mut Criterion) {
    let mut group = c.benchmark_group("Voltage Divider");
    
    let sizes = [5, 10, 20];
    
    for &size in &sizes {
        let spice_code = generate_voltage_divider(size);
        
        // Parse the circuit once
        let network = create_simulation_from_spice(&spice_code).expect("Failed to parse voltage divider");
        
        group.bench_with_input(BenchmarkId::new("RSparse", size), &size, |b, _| {
            b.iter(|| {
                run_sim_for_benchmark::<RSparseSolver>(network.clone()).expect("Simulation failed");
            });
        });
    }
    
    group.finish();
}


