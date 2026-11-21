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
    #[serde(rename = "gear")]
    GearBdf { order: usize },
}

#[derive(Serialize)]
struct OutWrapper<'a> {
    r#type: &'static str,
    variables: Vec<&'a str>,
}

#[derive(Serialize)]
struct IntegrationMethodWrapper<'a> {
    r#type: &'static str,
    method: &'a IntegrationMethod,
}

impl Serialize for SimulationOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SimulationOption::Out(vars) => OutWrapper {
                r#type: "out",
                variables: vars.iter().map(|v| v.as_ref()).collect(),
            }
            .serialize(serializer),
            SimulationOption::IntegrationMethod(method) => IntegrationMethodWrapper {
                r#type: "integration",
                method,
            }
            .serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_out() {
        let opt = SimulationOption::Out(vec![Arc::from("v1"), Arc::from("v2")]);
        let json = serde_json::to_string(&opt).unwrap();
        assert_eq!(json, r#"{"type":"out","variables":["v1","v2"]}"#);
    }

    #[test]
    fn test_serialize_integration_method() {
        let opt = SimulationOption::IntegrationMethod(IntegrationMethod::BackwardEuler);
        let json = serde_json::to_string(&opt).unwrap();
        assert_eq!(json, r#"{"type":"integration","method":"be"}"#);

        let opt_gear = SimulationOption::IntegrationMethod(IntegrationMethod::GearBdf { order: 2 });
        let json_gear = serde_json::to_string(&opt_gear).unwrap();
        assert_eq!(json_gear, r#"{"type":"integration","method":{"gear":{"order":2}}}"#);
    }
}
