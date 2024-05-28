use super::Output;
use crate::{sim::simulation_result::SimulationResults, OutputError};

pub struct NetworkOutput {}

impl Output for NetworkOutput {
    fn output(&self, _res: SimulationResults) -> Result<(), OutputError> {
        Err(OutputError::Unimplemented)
    }
}

impl NetworkOutput {
    pub fn new() -> Self {
        Self {}
    }
}
