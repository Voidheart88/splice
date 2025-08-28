use super::Frontend;
use super::FrontendError;
use super::Simulation;

pub struct JsonFrontend {}

impl Frontend for JsonFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}

impl JsonFrontend {
    pub fn new(_: String) -> Self {
        Self {}
    }

    pub fn try_new_from_path(_path: String) -> Result<Self, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}
