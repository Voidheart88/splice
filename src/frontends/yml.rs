use crate::FrontendError;

use super::Frontend;
use super::Simulation;

pub struct YmlFrontend {}

impl Frontend for YmlFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}

impl YmlFrontend {
    pub fn new(_: String) -> Self {
        Self {}
    }
}
