use super::{Frontend, FrontendError, Simulation, Element};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct YamlElement {
    
}


pub struct YamlFrontend {
    
}

impl YamlFrontend {
    pub fn new_from_path(path: String) -> Self {
        Self {}
    }
    
    pub fn new_from_string(yaml_string: String) -> Self {
        Self {}
    }

    fn parse_yaml(&self) -> Result<Vec<Element>, FrontendError> {
        Err(FrontendError::Unimplemented)
    }
}

impl Frontend for YamlFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        let elements = self.parse_yaml()?;
        Ok(Simulation {
            commands: Vec::new(),
            options: Vec::new(),
            elements,
            variables: Vec::new(),
        })
    }
}
