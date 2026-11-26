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

#[derive(Serialize)]
struct OpWrapper<'a> {
    r#type: &'static str,
    variables: &'a [(Variable, Numeric)],
}

#[derive(Serialize)]
struct DcWrapper<'a> {
    r#type: &'static str,
    steps: &'a [Vec<(Variable, Numeric)>],
}

#[derive(Serialize)]
struct TranWrapper<'a> {
    r#type: &'static str,
    points: &'a [(Numeric, Vec<(Variable, Numeric)>)],
}

type AcSerializedResult<'a> = Vec<(Numeric, Vec<(&'a Variable, (Numeric, Numeric))>)>;

#[derive(Serialize)]
struct AcWrapper<'a> {
    r#type: &'static str,
    bode_values: AcSerializedResult<'a>,
}

impl Serialize for Sim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Sim::Op(vars) => OpWrapper {
                r#type: "op",
                variables: vars,
            }
            .serialize(serializer),
            Sim::Dc(steps) => DcWrapper {
                r#type: "dc",
                steps,
            }
            .serialize(serializer),
            Sim::Tran(points) => TranWrapper {
                r#type: "tran",
                points,
            }
            .serialize(serializer),
            Sim::Ac(bode_values) => {
                let bode_values = bode_values
                    .iter()
                    .map(|(freq, vars)| {
                        let converted_vars =
                            vars.iter().map(|(var, c)| (var, (c.re, c.im))).collect();
                        (*freq, converted_vars)
                    })
                    .collect();

                AcWrapper {
                    r#type: "ac",
                    bode_values,
                }
                .serialize(serializer)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct SimulationResults {
    pub options: Vec<SimulationOption>,
    pub results: Vec<Sim>,
}
