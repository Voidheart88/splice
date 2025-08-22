
use serde::Deserialize;

#[derive(Deserialize)]
pub struct YamlResistor {
    pub name: String,
    pub node0: String,
    pub node1: String,
    pub value: f64,
}