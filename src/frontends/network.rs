use super::Frontend;
use super::FrontendError;
use super::Simulation;

pub struct NetworkFrontend {}

impl Frontend for NetworkFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}

impl NetworkFrontend {
    pub fn new() -> Self {
        Self {}
    }
}
