use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;

use super::Element;
use super::Frontend;
use super::FrontendError;
use super::Simulation;
use super::Variable;
use super::serde::ProcessSerdeElement;
use super::serde::SerdeCircuit;
use rmp_serde::decode::from_read;

pub(crate) struct NetworkFrontend {
    listener: TcpListener,
    port: u16,
}

impl Frontend for NetworkFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        // Accept incoming connection
        let (stream, _) = self.listener.accept()?;

        // Receive MessagePack data
        let circuit: SerdeCircuit = from_read(&stream)?;

        // Convert to internal simulation format
        self.convert_circuit(circuit)
    }
}

impl NetworkFrontend {
    pub fn new(port: u16) -> Result<Self, FrontendError> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
        Ok(Self { listener, port })
    }

    fn convert_circuit(&self, circuit: SerdeCircuit) -> Result<Simulation, FrontendError> {
        let mut elements: Vec<Element> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();

        circuit.elements.iter().for_each(|element| {
            element.process(&mut variables, &mut elements, &mut var_map);
        });

        let commands = circuit
            .simulations
            .into_iter()
            .map(Into::into)
            .collect();

        let options = circuit
            .options
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(Simulation {
            elements,
            commands,
            options,
            variables,
        })
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

#[cfg(test)]
mod tests {
    use crate::frontends::serde::{SerdeDC, SerdeSimulation};
    use crate::sim::commands::SimulationCommand;

    #[test]
    fn test_convert_simulation_command_op() {
        assert_eq!(
            SimulationCommand::from(SerdeSimulation::OP),
            SimulationCommand::Op
        );
    }

    #[test]
    fn test_convert_simulation_command_dc() {
        let dc = SerdeDC::new("V1".to_string(), 0.0, 5.0, 1.0);
        if let SimulationCommand::Dc(src, start, stop, step, _) =
            SimulationCommand::from(SerdeSimulation::DC(dc))
        {
            assert_eq!(src.as_ref(), "V1");
            assert_eq!(start, 0.0);
            assert_eq!(stop, 5.0);
            assert_eq!(step, 1.0);
        } else {
            panic!("Expected DC command");
        }
    }
}
