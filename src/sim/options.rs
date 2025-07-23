use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum SimulationOption {
    Out(Vec<Arc<str>>),
}
