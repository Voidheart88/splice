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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Unit;
    use num::Complex;
    use std::sync::Arc;

    #[test]
    fn test_convert_sim_result_op() {
        let var = Variable::new(Arc::from("v1"), Unit::Volt, 0);
        let sim = Sim::Op(vec![(var, 3.3)]);
        let results = NetworkBackend::convert_sim_result(sim);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].r#type, "op");
        assert_eq!(results[0].variables.len(), 1);
        assert_eq!(results[0].variables[0].name, "v1");
        assert_eq!(results[0].variables[0].unit, "V");
        assert_eq!(results[0].variables[0].value, 3.3);
    }

    #[test]
    fn test_convert_sim_result_dc() {
        let var = Variable::new(Arc::from("i1"), Unit::Ampere, 0);
        let sim = Sim::Dc(vec![vec![(var.clone(), 0.1)], vec![(var, 0.2)]]);
        let results = NetworkBackend::convert_sim_result(sim);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].r#type, "dc_step_0");
        assert_eq!(results[0].variables[0].value, 0.1);
        assert_eq!(results[1].r#type, "dc_step_1");
        assert_eq!(results[1].variables[0].value, 0.2);
    }

    #[test]
    fn test_convert_sim_result_ac() {
        let var = Variable::new(Arc::from("out"), Unit::Volt, 0);
        let sim = Sim::Ac(vec![(1000.0, vec![(var, Complex::new(2.5, 1.0))])]);
        let results = NetworkBackend::convert_sim_result(sim);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].r#type, "ac_1000");
        assert_eq!(results[0].variables[0].value, 2.5);
    }

    #[test]
    fn test_convert_sim_result_tran() {
        let var = Variable::new(Arc::from("node1"), Unit::Volt, 0);
        let sim = Sim::Tran(vec![(0.01, vec![(var, 4.2)])]);
        let results = NetworkBackend::convert_sim_result(sim);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].r#type, "tran_0.01");
        assert_eq!(results[0].variables[0].value, 4.2);
    }
}
