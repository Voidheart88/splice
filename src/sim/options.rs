use serde::Serialize;
use std::sync::Arc;

use crate::solver::integration::{BackwardEuler, ForwardEuler, IntegrationMethod, Trapezoidal, BDF};

/// Simulation options that can be set via .options command
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimulationOption {
    Out(Vec<Arc<str>>),
    IntegrationMethod(IntegrationMethodEnum),
}

/// Integration method enum - this wraps the trait objects
/// We use this to serialize/deserialize integration methods
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum IntegrationMethodEnum {
    #[serde(rename = "be")]
    BackwardEuler,
    #[serde(rename = "forward_euler")]
    ForwardEuler,
    #[serde(rename = "trapezoidal")]
    Trapezoidal,
    #[serde(rename = "gear")]
    GearBdf { order: usize },
}

impl IntegrationMethodEnum {
    /// Convert the enum to an integration method trait object
    pub fn to_integration_method(&self, delta_t: f64) -> Box<dyn IntegrationMethod> {
        match self {
            IntegrationMethodEnum::BackwardEuler => Box::new(BackwardEuler::new(delta_t)),
            IntegrationMethodEnum::ForwardEuler => Box::new(ForwardEuler::new(delta_t)),
            IntegrationMethodEnum::Trapezoidal => Box::new(Trapezoidal::new(delta_t)),
            IntegrationMethodEnum::GearBdf { order } => Box::new(BDF::new(delta_t, *order)),
        }
    }

    /// Get the name of the integration method
    pub fn name(&self) -> &'static str {
        match self {
            IntegrationMethodEnum::BackwardEuler => "Backward Euler",
            IntegrationMethodEnum::ForwardEuler => "Forward Euler",
            IntegrationMethodEnum::Trapezoidal => "Trapezoidal",
            IntegrationMethodEnum::GearBdf { order } => match order {
                1 => "BDF1",
                2 => "BDF2",
                3 => "BDF3",
                4 => "BDF4",
                5 => "BDF5",
                6 => "BDF6",
                _ => "BDF",
            },
        }
    }
}

#[derive(Serialize)]
struct OutWrapper<'a> {
    r#type: &'static str,
    variables: Vec<&'a str>,
}

#[derive(Serialize)]
struct IntegrationMethodWrapper<'a> {
    r#type: &'static str,
    method: &'a IntegrationMethodEnum,
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
        let opt = SimulationOption::IntegrationMethod(IntegrationMethodEnum::BackwardEuler);
        let json = serde_json::to_string(&opt).unwrap();
        assert_eq!(json, r#"{"type":"integration","method":"be"}"#);

        let opt_gear = SimulationOption::IntegrationMethod(IntegrationMethodEnum::GearBdf { order: 2 });
        let json_gear = serde_json::to_string(&opt_gear).unwrap();
        assert_eq!(json_gear, r#"{"type":"integration","method":{"gear":{"order":2}}}"#);
    }

    #[test]
    fn test_integration_method_to_object() {
        let be = IntegrationMethodEnum::BackwardEuler;
        let method = be.to_integration_method(1e-6);
        assert_eq!(method.name(), "Backward Euler");
        assert!(!method.is_multistep());
    }

    #[test]
    fn test_bdf_integration_method_to_object() {
        let bdf = IntegrationMethodEnum::GearBdf { order: 2 };
        let method = bdf.to_integration_method(1e-6);
        assert_eq!(method.name(), "BDF2");
        assert!(method.is_multistep());
    }
}
