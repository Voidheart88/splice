// Serde implementation for coupled inductors

use serde::{Deserialize, Serialize};

use crate::spot::Numeric;

#[derive(Debug, Serialize, Deserialize)]
pub struct SerdeCoupledInductors {
    pub name: String,
    pub inductor1: String,
    pub inductor2: String,
    pub coupling_factor: Numeric,
}
