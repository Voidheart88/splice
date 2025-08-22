use serde::Deserialize;

use crate::models::Element;

#[derive(Debug, Deserialize)]
pub struct YamlResistor {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub value: f64,
}

impl Into<Element> for YamlResistor {
    fn into(self) -> Element {
        todo!()
    }
}
