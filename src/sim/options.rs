use serde::Serialize;
use std::sync::Arc;

/// Helper struct for serializing Out simulation option
#[derive(Serialize)]
struct OutWrapper {
    r#type: &'static str,
    variables: Vec<String>,
}

/// Helper struct for serializing IntegrationMethod simulation option
#[derive(Serialize)]
struct IntegrationMethodWrapper {
    r#type: &'static str,
    method: IntegrationMethod,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimulationOption {
    Out(Vec<Arc<str>>),
    IntegrationMethod(IntegrationMethod),
}

impl SimulationOption {
    /// Returns the integration method if this option is an IntegrationMethod variant
    pub fn get_integration_method(&self) -> Option<IntegrationMethod> {
        if let SimulationOption::IntegrationMethod(method) = self {
            Some(method.clone())
        } else {
            None
        }
    }
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
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
                OutWrapper {
                    r#type: "out",
                    variables: vars.iter().map(|v| v.to_string()).collect(),
                }
                .serialize(serializer)
            }
            SimulationOption::IntegrationMethod(method) => {
<<<<<<< HEAD
                // Local serializer struct for integration method
                // Note: Defined inside serialize fn for encapsulation
                #[derive(Serialize)]
                struct IntegrationMethodWrapper {
                    r#type: &'static str,
                    method: IntegrationMethod,
                }
=======
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
                IntegrationMethodWrapper {
                    r#type: "integration",
                    method: method.clone(),
                }
                .serialize(serializer)
            }
=======
            SimulationOption::Out(vars) => {
                OutWrapper {
                    r#type: "out",
                    variables: vars.iter().map(|v| v.to_string()).collect(),
                }
                .serialize(serializer)
            }
            SimulationOption::IntegrationMethod(method) => {
                IntegrationMethodWrapper {
                    r#type: "integration",
                    method: method.clone(),
                }
                .serialize(serializer)
            }
=======
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
                OutWrapper {
                    r#type: "out",
                    variables: vars.iter().map(|v| v.to_string()).collect(),
                }
                .serialize(serializer)
            }
            SimulationOption::IntegrationMethod(method) => {
<<<<<<< HEAD
                // Local serializer struct for integration method
                // Note: Defined inside serialize fn for encapsulation
                #[derive(Serialize)]
                struct IntegrationMethodWrapper {
                    r#type: &'static str,
                    method: IntegrationMethod,
                }
=======
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
                IntegrationMethodWrapper {
                    r#type: "integration",
                    method: method.clone(),
                }
                .serialize(serializer)
            }
        }
    }
}
