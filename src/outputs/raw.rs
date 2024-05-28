use super::Output;
use crate::sim::simulation_result::SimulationResults;
use crate::OutputError;

pub struct RawOutput {}

impl Output for RawOutput {
    fn output(&self, _res: SimulationResults) -> Result<(), OutputError> {
        Err(OutputError::Unimplemented)
    }
}

impl RawOutput {
    pub fn new() -> Self {
        Self {}
    }
}
