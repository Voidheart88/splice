use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimulationOption {
    Out(Vec<Arc<str>>),
}
