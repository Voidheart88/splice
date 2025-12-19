use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;

use super::serde::ProcessSerdeElement;
use super::serde::SerdeCircuit;
use super::serde::SerdeElement;
use super::serde::SerdeSimulation;
use super::Element;
use super::Frontend;
use super::FrontendError;
use super::Simulation;
use super::Variable;
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

    fn convert_circuit(&self, circuit: SerdeCircuit) -> Result<Simulation, FrontendError> {
        let mut commands: Vec<SimulationCommand> = Vec::new();
        let mut options: Vec<SimulationOption> = Vec::new();
        let mut elements: Vec<Element> = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();

        // Process elements (similar to SerdeFrontend)
        // Check: „similar“ sounds like there is code duplication. Check if this can be refactored in a helper fn
        for element in circuit.elements {
            match element {
                SerdeElement::Resistor(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::Inductor(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::Capacitor(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::VSource(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::VSourceSin(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::VSourceStep(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::ISource(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::Diode(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::Mosfet(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::Gain(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::VCVS(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::VCCS(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::CCCS(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::CCVS(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
                SerdeElement::CoupledInductors(ele) => {
                    ProcessSerdeElement::process(&ele, &mut variables, &mut elements, &mut var_map);
                }
            }
        }

        // Process simulation commands
        // TODO: Consider refactoring this simulation processing logic to reduce nesting
        for simulation in circuit.simulations {
            match simulation {
                SerdeSimulation::OP => {
                    commands.push(SimulationCommand::Op);
                }
                SerdeSimulation::DC(dc) => {
                    commands.push(SimulationCommand::Dc(
                        Arc::from(dc.source()),
                        dc.vstart(),
                        dc.vstop(),
                        dc.vstep(),
                        None,
                    ));
                }
                SerdeSimulation::AC(ac) => {
                    commands.push(SimulationCommand::Ac(
                        ac.fstart(),
                        ac.fstop(),
                        ac.fstep(),
                        ACMode::default(),
                    ));
                }
                SerdeSimulation::Tran(tran) => {
                    commands.push(SimulationCommand::Tran(tran.tstep(), tran.tend()));
                }
            }
        }

        // Process options
        for option in circuit.options {
            options.push(SimulationOption::Out(vec![Arc::from(option.out)]));
        }

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
