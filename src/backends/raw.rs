use super::Backend;
use crate::BackendError;
use crate::sim::simulation_result::SimulationResults;

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
