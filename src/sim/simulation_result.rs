use num::Complex;

use crate::spot::Numeric;
use crate::models::Variable;
use super::options::SimulationOption;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Sim {
    Op(Vec<(Variable, Numeric)>),
    Dc(Vec<Vec<(Variable, Numeric)>>),
    Ac(Vec<(Numeric, Vec<(Variable, Complex<Numeric>)>)>),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SimulationResults {
    pub options: Vec<SimulationOption>,
    pub results: Vec<Sim>,
}

impl Default for SimulationResults {
    fn default() -> Self {
        Self {
            options: Default::default(),
            results: Default::default(),
        }
    }
}
