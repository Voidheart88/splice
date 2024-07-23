use num::Complex;

use crate::models::Variable;

use super::options::SimulationOption;


#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Sim {
    Op(Vec<(Variable, f64)>),
    Dc(Vec<Vec<(Variable, f64)>>),
    Ac(Vec<(f64, Vec<(Variable, Complex<f64>)>)>),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SimulationResults{
    pub options: Vec<SimulationOption>,
    pub results: Vec<Sim>
}

impl Default for SimulationResults {
    fn default() -> Self {
        Self { 
            options: Default::default(), 
            results: Default::default() }
    }
}