use num::Complex;

use super::options::SimulationOption;
use crate::models::Variable;
use crate::spot::Numeric;

type BodeValue = (Numeric, Vec<(Variable, Complex<Numeric>)>);

#[derive(Debug, Clone, PartialEq)]
pub enum Sim {
    Op(Vec<(Variable, Numeric)>),
    Dc(Vec<Vec<(Variable, Numeric)>>),
    Ac(Vec<BodeValue>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SimulationResults {
    pub options: Vec<SimulationOption>,
    pub results: Vec<Sim>,
}
