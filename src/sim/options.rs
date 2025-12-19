use serde::Serialize;
use std::sync::Arc;

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
                // Local serializer struct for output variables
                // Note: Defined inside serialize fn for encapsulation
                #[derive(Serialize)]
                struct OutWrapper {
                    r#type: &'static str,
                    variables: Vec<String>,
                }
                OutWrapper {
                    r#type: "out",
                    variables: vars.iter().map(|v| v.to_string()).collect(),
                }
                .serialize(serializer)
            }
            SimulationOption::IntegrationMethod(method) => {
                // Local serializer struct for integration method
                // Note: Defined inside serialize fn for encapsulation
                #[derive(Serialize)]
                struct IntegrationMethodWrapper {
                    r#type: &'static str,
                    method: IntegrationMethod,
                }
                IntegrationMethodWrapper {
                    r#type: "integration",
                    method: method.clone(),
                }
                .serialize(serializer)
            }
        }
    }
}
