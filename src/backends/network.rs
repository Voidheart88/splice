use std::net::TcpStream;

use super::Backend;
use crate::{sim::simulation_result::SimulationResults, BackendError};
use rmp_serde::encode::write as msgpack_write;

#[derive(serde::Serialize)]
struct NetworkResponse {
    status: String,
    results: Vec<NetworkSimulationResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(serde::Serialize)]
struct NetworkSimulationResult {
    r#type: String,
    variables: Vec<NetworkVariable>,
}

#[derive(serde::Serialize)]
struct NetworkVariable {
    name: String,
    unit: String,
    value: f64,
}

pub(crate) struct NetworkBackend {
    stream: TcpStream,
}

impl Backend for NetworkBackend {
    fn output(&self, res: SimulationResults) -> Result<(), BackendError> {
        let response = self.convert_results(res)?;
        let mut stream = self.stream.try_clone()?;
        msgpack_write(&mut stream, &response)?;
        Ok(())
    }
}

impl NetworkBackend {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    fn convert_results(&self, results: SimulationResults) -> Result<NetworkResponse, BackendError> {
        let mut network_results = Vec::new();
        
        for sim_result in results.results {
            match sim_result {
                crate::sim::simulation_result::Sim::Op(vars) => {
                    let variables = vars.into_iter().map(|(var, val)| NetworkVariable {
                        name: var.name().to_string(),
                        unit: var.unit().to_string(),
                        value: val,
                    }).collect();
                    
                    network_results.push(NetworkSimulationResult {
                        r#type: "op".to_string(),
                        variables,
                    });
                }
                crate::sim::simulation_result::Sim::Dc(dc_results) => {
                    // DC results are Vec<Vec<(Variable, Numeric)>> where each inner Vec is a sweep step
                    for (step_idx, variables) in dc_results.iter().enumerate() {
                        let vars_converted = variables.iter().map(|(var, val)| NetworkVariable {
                            name: var.name().to_string(),
                            unit: var.unit().to_string(),
                            value: *val,
                        }).collect();
                        
                        network_results.push(NetworkSimulationResult {
                            r#type: format!("dc_step_{}", step_idx),
                            variables: vars_converted,
                        });
                    }
                }
                crate::sim::simulation_result::Sim::Ac(vars) => {
                    for (freq, variables) in vars {
                        let vars_converted = variables.into_iter().map(|(var, val)| NetworkVariable {
                            name: var.name().to_string(),
                            unit: var.unit().to_string(),
                            value: val.re, // Real part for AC analysis
                        }).collect();
                        
                        network_results.push(NetworkSimulationResult {
                            r#type: format!("ac_{}", freq),
                            variables: vars_converted,
                        });
                    }
                }
                crate::sim::simulation_result::Sim::Tran(vars) => {
                    for (time, variables) in vars {
                        let vars_converted = variables.into_iter().map(|(var, val)| NetworkVariable {
                            name: var.name().to_string(),
                            unit: var.unit().to_string(),
                            value: val,
                        }).collect();
                        
                        network_results.push(NetworkSimulationResult {
                            r#type: format!("tran_{}", time),
                            variables: vars_converted,
                        });
                    }
                }
            }
        }
        
        Ok(NetworkResponse {
            status: "success".to_string(),
            results: network_results,
            error: None,
        })
    }
}
