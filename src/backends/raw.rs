use super::Backend;
use crate::sim::simulation_result::SimulationResults;
use crate::BackendError;

pub struct RawBackend {}

impl Backend for RawBackend {
    fn output(&self, _res: SimulationResults) -> Result<(), BackendError> {
        Err(BackendError::Unimplemented)
    }
}

impl RawBackend {
    pub fn new() -> Self {
        Self {}
    }
}
