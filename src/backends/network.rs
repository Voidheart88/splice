use std::net::TcpStream;

use super::Backend;
use crate::{sim::simulation_result::SimulationResults, BackendError};
use rmp_serde::encode::write as msgpack_write;

/// Network backend for remote simulation using MessagePack protocol.
///
/// This backend allows Splice to run as a server, accepting simulation requests
/// over TCP and returning results in a structured MessagePack format.
/// Clients can connect to the server and receive simulation results for
/// remote processing or visualization.
///
/// Response structure sent to the network client.
///
/// Contains the simulation status, results, and any errors that occurred.
#[derive(serde::Serialize)]
struct NetworkResponse {
    status: String,
    results: Vec<NetworkSimulationResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Individual simulation result for a specific analysis type.
///
/// Each result contains the type of simulation (OP, DC, AC, TRAN)
/// and the associated variables with their values.
#[derive(serde::Serialize)]
struct NetworkSimulationResult {
    r#type: String,
    variables: Vec<NetworkVariable>,
}

/// Variable data structure containing name, unit, and value.
///
/// Represents a single circuit variable (node voltage, current, etc.)
/// with its physical unit and numerical value.
#[derive(serde::Serialize)]
struct NetworkVariable {
    name: String,
    unit: String,
    value: f64,
}

/// Network backend implementation using TCP streaming.
///
/// This backend sends simulation results over a TCP connection using
/// MessagePack serialization for efficient data transfer.
pub(crate) struct NetworkBackend {
    stream: TcpStream,
}

impl Backend for NetworkBackend {
    /// Outputs simulation results to the network client.
    ///
    /// Converts the simulation results to network format and sends them
    /// over the TCP connection using MessagePack serialization.
    ///
    /// # Arguments
    ///
    /// * `res` - The simulation results to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the results were successfully sent
    /// * `Err(BackendError)` - If there was an error during serialization or transmission
    fn output(&self, res: SimulationResults) -> Result<(), BackendError> {
        let response = self.convert_results(res)?;
        let mut stream = self.stream.try_clone()?;
        msgpack_write(&mut stream, &response)?;
        Ok(())
    }
}

impl NetworkBackend {
    /// Creates a new NetworkBackend instance.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream to use for communication
    ///
    /// # Returns
    ///
    /// A new NetworkBackend instance ready to send results
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Converts simulation results to network response format.
    ///
    /// This method transforms the internal simulation results into a format
    /// suitable for network transmission, handling all supported simulation types
    /// (OP, DC, AC, TRAN) and converting them to a standardized structure.
    ///
    /// # Arguments
    ///
    /// * `results` - The simulation results to convert
    ///
    /// # Returns
    ///
    /// * `Ok(NetworkResponse)` - The converted results ready for transmission
    /// * `Err(BackendError)` - If there was an error during conversion
    fn convert_results(&self, results: SimulationResults) -> Result<NetworkResponse, BackendError> {
        let mut network_results = Vec::new();

        for sim_result in results.results {
            match sim_result {
                crate::sim::simulation_result::Sim::Op(vars) => {
                    // Convert operating point results
                    let variables = vars
                        .into_iter()
                        .map(|(var, val)| NetworkVariable {
                            name: var.name().to_string(),
                            unit: var.unit().to_string(),
                            value: val,
                        })
                        .collect();

                    network_results.push(NetworkSimulationResult {
                        r#type: "op".to_string(),
                        variables,
                    });
                }
                crate::sim::simulation_result::Sim::Dc(dc_results) => {
                    // DC results are Vec<Vec<(Variable, Numeric)>> where each inner Vec is a sweep step
                    for (step_idx, variables) in dc_results.iter().enumerate() {
                        let vars_converted = variables
                            .iter()
                            .map(|(var, val)| NetworkVariable {
                                name: var.name().to_string(),
                                unit: var.unit().to_string(),
                                value: *val,
                            })
                            .collect();

                        network_results.push(NetworkSimulationResult {
                            r#type: format!("dc_step_{}", step_idx),
                            variables: vars_converted,
                        });
                    }
                }
                crate::sim::simulation_result::Sim::Ac(vars) => {
                    // Convert AC analysis results (frequency domain)
                    for (freq, variables) in vars {
                        let vars_converted = variables
                            .into_iter()
                            .map(|(var, val)| NetworkVariable {
                                name: var.name().to_string(),
                                unit: var.unit().to_string(),
                                value: val.re, // Real part for AC analysis
                            })
                            .collect();

                        network_results.push(NetworkSimulationResult {
                            r#type: format!("ac_{}", freq),
                            variables: vars_converted,
                        });
                    }
                }
                crate::sim::simulation_result::Sim::Tran(vars) => {
                    // Convert transient analysis results (time domain)
                    for (time, variables) in vars {
                        let vars_converted = variables
                            .into_iter()
                            .map(|(var, val)| NetworkVariable {
                                name: var.name().to_string(),
                                unit: var.unit().to_string(),
                                value: val,
                            })
                            .collect();

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
