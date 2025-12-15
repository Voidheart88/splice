use std::sync::Arc;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimulationOption {
    Out(Vec<Arc<str>>),
    IntegrationMethod(IntegrationMethod),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum IntegrationMethod {
    #[serde(rename = "be")]
    BackwardEuler,
    #[serde(rename = "trapezoidal")]
    Trapezoidal,
}



impl Serialize for SimulationOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SimulationOption::Out(vars) => {
                #[derive(Serialize)]
                struct OutWrapper {
                    r#type: &'static str,
                    variables: Vec<String>,
                }
                OutWrapper {
                    r#type: "out",
                    variables: vars.iter().map(|v| v.to_string()).collect(),
                }.serialize(serializer)
            }
            SimulationOption::IntegrationMethod(method) => {
                #[derive(Serialize)]
                struct IntegrationMethodWrapper {
                    r#type: &'static str,
                    method: IntegrationMethod,
                }
                IntegrationMethodWrapper {
                    r#type: "integration",
                    method: method.clone(),
                }.serialize(serializer)
            }
        }
    }
}
