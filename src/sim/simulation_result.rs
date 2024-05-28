use derive_more::Deref;

use crate::models::Variable;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deref)]
pub(crate) struct SimulationResults(pub Vec<Sim>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Sim {
    Op(Vec<(Variable, f64)>),
    Dc(Vec<Vec<(Variable, f64)>>),
}
