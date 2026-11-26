use std::net::TcpStream;

use super::Backend;
use crate::{
    BackendError,
    models::Variable,
    sim::simulation_result::{Sim, SimulationResults},
};
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

impl NetworkVariable {
    fn new(var: &Variable, value: f64) -> Self {
        Self {
            name: var.name().to_string(),
            unit: var.unit().to_string(),
            value,
        }
    }
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

    fn convert_sim_result(sim_result: Sim) -> Vec<NetworkSimulationResult> {
        match sim_result {
            Sim::Op(vars) => {
                let variables = vars
                    .into_iter()
                    .map(|(var, val)| NetworkVariable::new(&var, val))
                    .collect();
                vec![NetworkSimulationResult {
                    r#type: "op".to_string(),
                    variables,
                }]
            }
            Sim::Dc(dc_results) => dc_results
                .into_iter()
                .enumerate()
                .map(|(step_idx, variables)| {
                    let vars_converted = variables
                        .into_iter()
                        .map(|(var, val)| NetworkVariable::new(&var, val))
                        .collect();
                    NetworkSimulationResult {
                        r#type: format!("dc_step_{step_idx}"),
                        variables: vars_converted,
                    }
                })
                .collect(),
            Sim::Ac(vars) => vars
                .into_iter()
                .map(|(freq, variables)| {
                    let vars_converted = variables
                        .into_iter()
                        .map(|(var, val)| NetworkVariable::new(&var, val.re))
                        .collect();
                    NetworkSimulationResult {
                        r#type: format!("ac_{freq}"),
                        variables: vars_converted,
                    }
                })
                .collect(),
            Sim::Tran(vars) => vars
                .into_iter()
                .map(|(time, variables)| {
                    let vars_converted = variables
                        .into_iter()
                        .map(|(var, val)| NetworkVariable::new(&var, val))
                        .collect();
                    NetworkSimulationResult {
                        r#type: format!("tran_{time}"),
                        variables: vars_converted,
                    }
                })
                .collect(),
        }
    }

    fn convert_results(&self, results: SimulationResults) -> Result<NetworkResponse, BackendError> {
        let network_results = results
            .results
            .into_iter()
            .flat_map(Self::convert_sim_result)
            .collect();

        Ok(NetworkResponse {
            status: "success".to_string(),
            results: network_results,
            error: None,
        })
    }
}
