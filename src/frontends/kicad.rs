use super::Frontend;
use super::FrontendError;
use super::Simulation;

pub struct KicadFrontend {}

impl Frontend for KicadFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}

impl KicadFrontend {
    pub fn new() -> Self {
        Self {}
    }

    pub fn try_new_from_path(_path: String) -> Result<Self, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}
