// Network mode performance benchmarks
// This module benchmarks the MessagePack serialization/deserialization
// and network communication performance

use criterion::{Criterion, BenchmarkId, BatchSize};
use rmp_serde::{encode::write as msgpack_write, decode::from_read as msgpack_read};
use std::io::Cursor;
use splice::frontends::serde::{SerdeCircuit, SerdeElement, SerdeSimulation, SerdeOption, SerdeResistor, SerdeVSource, SerdeCapacitor, SerdeDC, SerdeAC, SerdeTran};
use splice::sim::simulation_result::SimulationResults;

/// Generate a simple RC circuit for benchmarking
fn create_simple_rc_circuit() -> SerdeCircuit {
    SerdeCircuit {
        elements: vec![
            SerdeElement::Resistor(SerdeResistor {
                name: "R1".to_string(),
                node0: "n1".to_string(),
                node1: "0".to_string(),
                value: 1000.0,
            }),
            SerdeElement::Capacitor(splice::models::capacitor::serde::SerdeCapacitor {
                name: "C1".to_string(),
                node0: "n1".to_string(),
                node1: "0".to_string(),
                value: 1e-6,
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
        options: vec![SerdeOption { out: "n1".to_string() }],
    }
}

/// Generate a medium-sized circuit with multiple components
fn create_medium_circuit(size: usize) -> SerdeCircuit {
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
            node0: if i == 1 { "n1".to_string() } else { format!("n{i}") },
            node1: format!("n{}", i + 1),
            value: 1000.0,
        }));
    }
    
    // Add capacitor to ground
    elements.push(SerdeElement::Capacitor(splice::models::capacitor::serde::SerdeCapacitor {
        name: "C1".to_string(),
        node0: format!("n{}", size + 1),
        node1: "0".to_string(),
        value: 1e-6,
    }));
    
    SerdeCircuit {
        elements,
        simulations: vec![SerdeSimulation::OP],
        options: vec![SerdeOption { out: format!("n{}", size + 1) }],
    }
}

/// Generate a large circuit for stress testing
fn create_large_circuit(rows: usize, cols: usize) -> SerdeCircuit {
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
            let current_node = if row == 0 && col == 0 { 1 } else { node_counter - 1 };
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
        options: vec![SerdeOption { out: "n1".to_string() }],
    }
}

/// Benchmark MessagePack serialization performance
pub fn bench_msgpack_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("MessagePack Serialization");
    
    // Small circuit (3 elements)
    let small_circuit = create_simple_rc_circuit();
    group.bench_function("Small circuit (3 elements)", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &small_circuit).unwrap();
            buf.len()
        });
    });
    
    // Medium circuits (10-100 elements)
    for size in [10, 50, 100] {
        let circuit = create_medium_circuit(size);
        group.bench_with_input(BenchmarkId::new("Medium circuit", size), &size, |b, _| {
            b.iter(|| {
                let mut buf = Vec::new();
                msgpack_write(&mut buf, &circuit).unwrap();
                buf.len()
            });
        });
    }
    
    // Large circuits (grid-based)
    for (rows, cols) in [(5, 5), (10, 10)] {
        let size_desc = format!("{rows}x{cols} grid");
        let circuit = create_large_circuit(rows, cols);
        group.bench_with_input(BenchmarkId::new("Large circuit", &size_desc), &size_desc, |b, _| {
            b.iter(|| {
                let mut buf = Vec::new();
                msgpack_write(&mut buf, &circuit).unwrap();
                buf.len()
            });
        });
    }
    
    group.finish();
}

/// Benchmark MessagePack deserialization performance
pub fn bench_msgpack_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("MessagePack Deserialization");
    
    // Prepare serialized data for different circuit sizes
    let small_circuit = create_simple_rc_circuit();
    let mut small_data = Vec::new();
    msgpack_write(&mut small_data, &small_circuit).unwrap();
    
    let medium_circuit_50 = create_medium_circuit(50);
    let mut medium_data_50 = Vec::new();
    msgpack_write(&mut medium_data_50, &medium_circuit_50).unwrap();
    
    let large_circuit = create_large_circuit(10, 10);
    let mut large_data = Vec::new();
    msgpack_write(&mut large_data, &large_circuit).unwrap();
    
    group.bench_function("Small circuit (3 elements)", |b| {
        b.iter(|| {
            let cursor = Cursor::new(&small_data);
            let _: SerdeCircuit = msgpack_read(cursor).unwrap();
        });
    });
    
    group.bench_function("Medium circuit (50 elements)", |b| {
        b.iter(|| {
            let cursor = Cursor::new(&medium_data_50);
            let _: SerdeCircuit = msgpack_read(cursor).unwrap();
        });
    });
    
    group.bench_function("Large circuit (10x10 grid)", |b| {
        b.iter(|| {
            let cursor = Cursor::new(&large_data);
            let _: SerdeCircuit = msgpack_read(cursor).unwrap();
        });
    });
    
    group.finish();
}

/// Benchmark round-trip serialization/deserialization
pub fn bench_msgpack_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("MessagePack Roundtrip");
    
    let circuits = vec![
        ("Small", create_simple_rc_circuit()),
        ("Medium", create_medium_circuit(50)),
        ("Large", create_large_circuit(10, 10)),
    ];
    
    for (name, circuit) in circuits {
        group.bench_function(name, |b| {
            b.iter(|| {
                let mut buf = Vec::new();
                msgpack_write(&mut buf, &circuit).unwrap();
                let cursor = Cursor::new(&buf);
                let _: SerdeCircuit = msgpack_read(cursor).unwrap();
                buf.len()
            });
        });
    }
    
    group.finish();
}

/// Benchmark different simulation types serialization
pub fn bench_simulation_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("Simulation Types");
    
    let base_circuit = create_simple_rc_circuit();
    
    // OP simulation
    let op_circuit = SerdeCircuit {
        simulations: vec![SerdeSimulation::OP],
        ..base_circuit.clone()
    };
    
    // DC simulation
    let dc_circuit = SerdeCircuit {
        simulations: vec![SerdeSimulation::DC(SerdeDC::new("V1".to_string(), 0.0, 5.0, 0.1))],
        ..base_circuit.clone()
    };
    
    // AC simulation
    let ac_circuit = SerdeCircuit {
        simulations: vec![SerdeSimulation::AC(splice::models::ac::serde::SerdeAC {
            fstart: 1.0,
            fstop: 1e6,
            fstep: 100,
        })],
        ..base_circuit.clone()
    };
    
    // Transient simulation
    let tran_circuit = SerdeCircuit {
        simulations: vec![SerdeSimulation::Tran(splice::models::tran::serde::SerdeTran {
            tstep: 1e-6,
            tend: 1e-3,
        })],
        ..base_circuit
    };
    
    group.bench_function("OP simulation", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &op_circuit).unwrap();
            buf.len()
        });
    });
    
    group.bench_function("DC simulation", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &dc_circuit).unwrap();
            buf.len()
        });
    });
    
    group.bench_function("AC simulation", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &ac_circuit).unwrap();
            buf.len()
        });
    });
    
    group.bench_function("Transient simulation", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &tran_circuit).unwrap();
            buf.len()
        });
    });
    
    group.finish();
}

/// Benchmark result serialization (simulated results)
pub fn bench_result_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("Result Serialization");
    
    // Create a mock result with different sizes
    let small_result = SimulationResults {
        options: vec![],
        results: vec![splice::sim::simulation_result::Sim::Op(vec![
            (splice::sim::simulation_result::Variable::Voltage("n1".to_string()), 5.0),
        ])],
    };
    
    let medium_result = SimulationResults {
        options: vec![],
        results: vec![splice::sim::simulation_result::Sim::Op(
            (0..50).map(|i| {
                (splice::sim::simulation_result::Variable::Voltage(format!("n{i}")), i as f64 * 0.1)
            }).collect()
        )],
    };
    
    let large_result = SimulationResults {
        options: vec![],
        results: vec![splice::sim::simulation_result::Sim::Dc(
            (0..100).map(|i| {
                (0..50).map(|j| {
                    (splice::sim::simulation_result::Variable::Voltage(format!("n{j}")), (i * j) as f64 * 0.01)
                }).collect()
            }).collect()
        )],
    };
    
    group.bench_function("Small result (1 variable)", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &small_result).unwrap();
            buf.len()
        });
    });
    
    group.bench_function("Medium result (50 variables)", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &medium_result).unwrap();
            buf.len()
        });
    });
    
    group.bench_function("Large result (100x50 DC sweep)", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            msgpack_write(&mut buf, &large_result).unwrap();
            buf.len()
        });
    });
    
    group.finish();
}

/// Benchmark payload size vs serialization time
pub fn bench_payload_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Payload Scaling");
    
    let sizes = [10, 50, 100, 200, 500];
    
    for size in sizes.iter() {
        let circuit = create_medium_circuit(*size);
        
        group.bench_with_input(BenchmarkId::new("Serialization", size), size, |b, _| {
            b.iter(|| {
                let mut buf = Vec::new();
                msgpack_write(&mut buf, &circuit).unwrap();
                buf.len()
            });
        });
        
        // Prepare data for deserialization benchmark
        let mut data = Vec::new();
        msgpack_write(&mut data, &circuit).unwrap();
        
        group.bench_with_input(BenchmarkId::new("Deserialization", size), size, |b, _| {
            b.iter(|| {
                let cursor = Cursor::new(&data);
                let _: SerdeCircuit = msgpack_read(cursor).unwrap();
            });
        });
    }
    
    group.finish();
}