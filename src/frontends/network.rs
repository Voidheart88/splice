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
use super::serde::SerdeElement;
use super::serde::SerdeSimulation;
use crate::sim::commands::{ACMode, SimulationCommand};
use crate::sim::options::SimulationOption;
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

    fn process_element(
        element: &SerdeElement,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        match element {
            SerdeElement::Resistor(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::Inductor(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::Capacitor(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::VSource(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::VSourceSin(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::VSourceStep(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::ISource(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::Diode(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::Mosfet(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::Gain(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::VCVS(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::VCCS(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::CCCS(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::CCVS(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
            SerdeElement::CoupledInductors(ele) => {
                ProcessSerdeElement::process(ele, variables, elements, var_map);
            }
        }
    }

    fn convert_simulation_command(simulation: SerdeSimulation) -> SimulationCommand {
        match simulation {
            SerdeSimulation::OP => SimulationCommand::Op,
            SerdeSimulation::DC(dc) => SimulationCommand::Dc(
                Arc::from(dc.source()),
                dc.vstart(),
                dc.vstop(),
                dc.vstep(),
                None,
            ),
            SerdeSimulation::AC(ac) => SimulationCommand::Ac(
                ac.fstart(),
                ac.fstop(),
                ac.fstep(),
                ACMode::default(),
            ),
            SerdeSimulation::Tran(tran) => {
                SimulationCommand::Tran(tran.tstep(), tran.tend())
            }
        }
    }

    fn convert_circuit(&self, circuit: SerdeCircuit) -> Result<Simulation, FrontendError> {
        let mut elements: Vec<Element> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();

        circuit.elements.iter().for_each(|element| {
            Self::process_element(element, &mut variables, &mut elements, &mut var_map);
        });

        let commands = circuit
            .simulations
            .into_iter()
            .map(Self::convert_simulation_command)
            .collect();

        let options = circuit
            .options
            .into_iter()
            .map(|option| SimulationOption::Out(vec![Arc::from(option.out)]))
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
    use super::*;
    use crate::frontends::serde::SerdeDC;

    #[test]
    fn test_convert_simulation_command_op() {
        assert_eq!(
            NetworkFrontend::convert_simulation_command(SerdeSimulation::OP),
            SimulationCommand::Op
        );
    }

    #[test]
    fn test_convert_simulation_command_dc() {
        let dc = SerdeDC::new("V1".to_string(), 0.0, 5.0, 1.0);
        if let SimulationCommand::Dc(src, start, stop, step, _) =
            NetworkFrontend::convert_simulation_command(SerdeSimulation::DC(dc))
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
