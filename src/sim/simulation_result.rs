use derive_more::Deref;
use num::Complex;

use crate::models::Variable;

#[derive(Debug, Clone, PartialEq, Deref)]
pub(crate) struct SimulationResults(pub Vec<Sim>);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Sim {
    Op(Vec<(Variable, f64)>),
    Dc(Vec<Vec<(Variable, f64)>>),
    Ac(Vec<(f64, Vec<(Variable, Complex<f64>)>)>),
}
