pub(crate) mod serde;
pub(crate) mod spice;

use std::sync::Arc;
use super::*;
use crate::spot::*;

/// A structure representing a Gain with all its options.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GainBundle {
    name: Arc<str>,
    input: Option<Variable>,
    output: Option<Variable>,
    value: Numeric,
}

impl GainBundle {
    /// Creates a new `GainBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the gain bundle.
    /// * `input` - The input variable.
    /// * `output` - The output variable.
    /// * `value` - The gain value.
    ///
    /// # Returns
    ///
    /// A new `GainBundle` object.
    pub fn new(
        name: Arc<str>,
        input: Option<Variable>,
        output: Option<Variable>,
        value: Numeric,
    ) -> Self {
        GainBundle {
            name,
            input,
            output,
            value,
        }
    }

    /// Returns the name of the gain bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the triples representing the matrix A for the gain.
    /// For a gain, the matrix A is typically a simple linear relationship:
    /// output = gain * input
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let input_idx = match self.input_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };
        let output_idx = match self.output_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };
        Triples::new(&[(output_idx, input_idx, -self.value)])
    }

    /// Returns the indices for the triples in matrix A.
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        match (self.input_idx(), self.output_idx()) {
            (Some(input_idx), Some(output_idx)) => {
                Some(TripleIdx::new(&[(output_idx, input_idx)]))
            }
            _ => None,
        }
    }
    
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        let input_idx = match self.input_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };
        let output_idx = match self.output_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };
        Triples::new(&[(output_idx, input_idx, ComplexNumeric{re: -self.value, im: 0.0})])
    }

    /// Returns the pairs representing the vector b for the gain.
    /// For a pure gain, there is no constant term, so b is zero.
    pub fn pairs(&self, _x_vec: &[Numeric]) -> Pairs<Numeric, 0> {
        Pairs::new(&[])
    }

    /// Returns the index of the input variable.
    pub fn input_idx(&self) -> Option<usize> {
        self.input.as_ref().map(|v| v.idx())
    }

    /// Returns the index of the output variable.
    pub fn output_idx(&self) -> Option<usize> {
        self.output.as_ref().map(|v| v.idx())
    }
}

#[cfg(test)]
mod tests;
