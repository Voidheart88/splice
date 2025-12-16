use num::Complex;
use serde::Serialize;

use super::options::SimulationOption;
use crate::models::Variable;
use crate::spot::Numeric;

type BodeValue = (Numeric, Vec<(Variable, Complex<Numeric>)>);

#[derive(Debug, Clone, PartialEq)]
pub enum Sim {
    /// Operating Point Analysis Results
    Op(Vec<(Variable, Numeric)>),
    /// DC Analysis Results
    Dc(Vec<Vec<(Variable, Numeric)>>),
    /// Transient Analysis Results (current Timestep,Vec with <(Variable,Value)>)
    Tran(Vec<(Numeric, Vec<(Variable, Numeric)>)>),
    /// AC Analysis Results
    Ac(Vec<BodeValue>),
}

impl Serialize for Sim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Sim::Op(vars) => {
                #[derive(Serialize)]
                struct OpWrapper {
                    r#type: &'static str,
                    variables: Vec<(Variable, Numeric)>
                }
                OpWrapper { r#type: "op", variables: vars.clone() }.serialize(serializer)
            },
            Sim::Dc(steps) => {
                #[derive(Serialize)]
                struct DcWrapper {
                    r#type: &'static str,
                    steps: Vec<Vec<(Variable, Numeric)>>
                }
                DcWrapper { r#type: "dc", steps: steps.clone() }.serialize(serializer)
            },
            Sim::Tran(points) => {
                #[derive(Serialize)]
                struct TranWrapper {
                    r#type: &'static str,
                    points: Vec<(Numeric, Vec<(Variable, Numeric)>)>
                }
                TranWrapper { r#type: "tran", points: points.clone() }.serialize(serializer)
            },
            Sim::Ac(bode_values) => {
                /// Type alias for AC analysis results: (Frequency, Variables)
                type AcResult = Vec<(Numeric, Vec<(Variable, (Numeric, Numeric))>)>;
                
                #[derive(Serialize)]
                struct AcWrapper {
                    r#type: &'static str,
                    bode_values: AcResult
                }
                
                let converted = bode_values.iter().map(|(freq, vars)| {
                    let converted_vars = vars.iter().map(|(var, complex)| {
                        (var.clone(), (complex.re, complex.im))
                    }).collect();
                    (*freq, converted_vars)
                }).collect();
                
                AcWrapper { r#type: "ac", bode_values: converted }.serialize(serializer)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct SimulationResults {
    pub options: Vec<SimulationOption>,
    pub results: Vec<Sim>,
}
