use std::sync::Arc;

use crate::frontends::serde::{SerdeCircuit, SerdeElement, SerdeSimulation, SerdeOption};
use crate::models::resistor::serde::SerdeResistor;

#[test]
fn test_network_frontend_basic() {
    // Create a simple test circuit
    let circuit = SerdeCircuit {
        elements: vec![
            SerdeElement::Resistor(SerdeResistor {
                name: "R1".to_string(),
                node0: "n1".to_string(),
                node1: "0".to_string(),
                value: 10.0,
            }),
            SerdeElement::VSource(crate::models::vsource::serde::SerdeVSource {
                name: "V1".to_string(),
                node0: "0".to_string(),
                node1: "n1".to_string(),
                value: 5.0,
                ac_value: None,
            }),
        ],
        simulations: vec![
            SerdeSimulation::OP,
        ],
        options: vec![
            SerdeOption {
                out: "n1".to_string(),
            },
        ],
    };

    // Test MessagePack serialization/deserialization
    let serialized = rmp_serde::to_vec(&circuit).unwrap();
    let deserialized: SerdeCircuit = rmp_serde::from_slice(&serialized).unwrap();
    
    // Verify the circuit was correctly serialized/deserialized
    assert_eq!(deserialized.elements.len(), 2);
    assert_eq!(deserialized.simulations.len(), 1);
    assert_eq!(deserialized.options.len(), 1);
}

#[test]
fn test_network_backend_basic() {
    use crate::sim::simulation_result::Sim;
    use crate::sim::simulation_result::SimulationResults;
    use crate::models::Variable;
    use std::sync::Arc;
    let results = SimulationResults {
        options: vec![],
        results: vec![
            Sim::Op(vec![
                (Variable::new(Arc::from("n1"), crate::models::Unit::Volt, 0), 5.0),
            ]),
        ],
    };

    let serialized = rmp_serde::to_vec(&results).unwrap();
    assert!(!serialized.is_empty());
}

#[test]
fn test_network_integration() {
    let circuit = SerdeCircuit {
        elements: vec![
            SerdeElement::Resistor(SerdeResistor {
                name: "R1".to_string(),
                node0: "n1".to_string(),
                node1: "0".to_string(),
                value: 100.0,
            }),
            SerdeElement::VSource(crate::models::vsource::serde::SerdeVSource {
                name: "V1".to_string(),
                node0: "0".to_string(),
                node1: "n1".to_string(),
                value: 10.0,
                ac_value: None,
            }),
        ],
        simulations: vec![
            SerdeSimulation::OP,
        ],
        options: vec![
            SerdeOption {
                out: "n1".to_string(),
            },
        ],
    };

    // Test that the circuit can be processed through the Serde frontend
    let json_string = serde_json::to_string(&circuit).unwrap();
    let frontend_result = crate::frontends::serde::SerdeFrontend::try_new_from_string(
        json_string,
        crate::frontends::serde::SerdeFormat::Json,
    );
    
    match frontend_result {
        Ok(frontend) => {
            // Use the Frontend trait method
            let sim_result = crate::frontends::Frontend::simulation(&frontend);
            let sim = sim_result.unwrap();
            
            // Verify the simulation was created correctly
            assert_eq!(sim.commands.len(), 1);
            assert_eq!(sim.elements.len(), 2);
            assert_eq!(sim.options.len(), 1);
            
            // Check that we have the right variables
            assert!(sim.variables.iter().any(|v| v.name() == Arc::from("n1")));
        }
        Err(e) => panic!("Failed to create frontend: {}", e),
    }
}

#[test]
fn test_network_dc_analysis() {
    // Create a DC sweep test circuit
    let circuit = SerdeCircuit {
        elements: vec![
            SerdeElement::Resistor(SerdeResistor {
                name: "R1".to_string(),
                node0: "n1".to_string(),
                node1: "0".to_string(),
                value: 100.0,
            }),
            SerdeElement::VSource(crate::models::vsource::serde::SerdeVSource {
                name: "V1".to_string(),
                node0: "0".to_string(),
                node1: "n1".to_string(),
                value: 0.0, // Initial value
                ac_value: None,
            }),
        ],
        simulations: vec![
            crate::frontends::serde::SerdeSimulation::DC(crate::frontends::serde::SerdeDC::new(
                "V1".to_string(),
                0.0,
                10.0,
                2.5,
            )),
        ],
        options: vec![
            SerdeOption {
                out: "n1".to_string(),
            },
        ],
    };

    // Test that the DC circuit can be processed through the Serde frontend
    let json_string = serde_json::to_string(&circuit).unwrap();
    let frontend_result = crate::frontends::serde::SerdeFrontend::try_new_from_string(
        json_string,
        crate::frontends::serde::SerdeFormat::Json,
    );
    
    match frontend_result {
        Ok(frontend) => {
            // Use the Frontend trait method
            let sim_result = crate::frontends::Frontend::simulation(&frontend);
            let sim = sim_result.unwrap();
            
            // Verify the simulation was created correctly
            assert_eq!(sim.commands.len(), 1);
            assert_eq!(sim.elements.len(), 2);
            assert_eq!(sim.options.len(), 1);
            
            // Check that we have DC command with correct parameters
            if let crate::sim::commands::SimulationCommand::Dc(source, start, stop, step, _) = &sim.commands[0] {
                assert_eq!(source.as_ref(), "V1");
                assert!((start - 0.0).abs() < 1e-10);
                assert!((stop - 10.0).abs() < 1e-10);
                assert!((step - 2.5).abs() < 1e-10);
            } else {
                panic!("Expected DC command");
            }
        }
        Err(e) => panic!("Failed to create frontend: {}", e),
    }
}