use super::Backend;
use crate::{sim::simulation_result::SimulationResults, BackendError};

pub struct NetworkBackend {}

impl Backend for NetworkBackend {
    fn output(&self, _res: SimulationResults) -> Result<(), BackendError> {
        Err(BackendError::Unimplemented)
    }
}

impl NetworkBackend {
    pub fn new() -> Self {
        Self {}
    }
}
