/// The Summer Module. This module encapsulates everything regarding a summing block.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use crate::spot::*;
use std::sync::Arc;

/// A structure representing a summing block (adder).
///
/// The summer computes a weighted sum of multiple inputs:
/// output = gain[0] * input[0] + gain[1] * input[1] + ... + gain[n] * input[n]
///
/// This is useful for combining signals in analog circuit models.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SummerBundle {
    name: Arc<str>,
    inputs: Vec<Variable>,
    output: Option<Variable>,
    gains: Vec<Numeric>,
}

impl SummerBundle {
    /// Creates a new `SummerBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the summer block.
    /// * `inputs` - The input variables.
    /// * `output` - The output variable.
    /// * `gains` - The gain for each input (default 1.0 if not specified).
    ///
    /// # Returns
    ///
    /// A new `SummerBundle` object.
    pub fn new(
        name: Arc<str>,
        inputs: Vec<Variable>,
        output: Option<Variable>,
        gains: Option<Vec<Numeric>>,
    ) -> Self {
        let gains = gains.unwrap_or_else(|| vec![Numeric::one(); inputs.len()]);

        SummerBundle {
            name,
            inputs,
            output,
            gains,
        }
    }

    /// Returns the name of the summer block.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the number of inputs.
    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    /// Returns the indices of the input variables.
    pub fn input_indices(&self) -> Vec<usize> {
        self.inputs.iter().map(|v| v.idx()).collect()
    }

    /// Returns the index of the output variable.
    pub fn output_idx(&self) -> Option<usize> {
        self.output.as_ref().map(|v| v.idx())
    }

    /// Returns the indices for the triples in matrix A.
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        let output_idx = self.output_idx()?;

        if self.inputs.is_empty() {
            return None;
        }

        // Create indices for output->output and all input->output relationships
        let mut indices = vec![(output_idx, output_idx)];
        indices.extend(self.inputs.iter().map(|input| (output_idx, input.idx())));

        Some(TripleIdx::new(&indices))
    }

    /// Returns the triples representing the matrix A for the summer.
    ///
    /// For a summer: output - sum(gain[i] * input[i]) = 0
    ///
    /// Matrix form:
    /// [output_idx, output_idx] = 1
    /// [output_idx, input_idx[i]] = -gain[i] for each input
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let output_idx = match self.output_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };

        if self.inputs.is_empty() {
            return Triples::new(&[]);
        }

        // Build triples: output - sum(gain[i] * input[i]) = 0
        let mut triples_data = Vec::new();

        // output coefficient = 1
        triples_data.push((output_idx, output_idx, Numeric::one()));

        // input coefficients = -gain[i]
        for (input, &gain) in self.inputs.iter().zip(self.gains.iter()) {
            triples_data.push((output_idx, input.idx(), -gain));
        }

        Triples::new(&triples_data)
    }

    /// Returns the pairs representing the vector b (RHS) for the summer.
    ///
    /// For a pure summer, there is no constant term, so b is zero.
    pub fn pairs(&self, _x_vec: &[Numeric]) -> Pairs<Numeric, 0> {
        Pairs::new(&[])
    }

    /// Returns the AC triples for small-signal analysis.
    ///
    /// The summer is linear and time-invariant, so AC triples are the same as DC triples.
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        let output_idx = match self.output_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };

        if self.inputs.is_empty() {
            return Triples::new(&[]);
        }

        // Build complex triples
        let mut triples_data = Vec::new();

        // output coefficient = 1
        triples_data.push((output_idx, output_idx, ComplexNumeric::one()));

        // input coefficients = -gain[i] (as complex)
        for (input, &gain) in self.inputs.iter().zip(self.gains.iter()) {
            triples_data.push((
                output_idx,
                input.idx(),
                ComplexNumeric { re: -gain, im: 0.0 },
            ));
        }

        Triples::new(&triples_data)
    }

    /// Returns the gains for each input.
    pub fn gains(&self) -> &[Numeric] {
        &self.gains
    }

    /// Returns the inputs.
    pub fn inputs(&self) -> &[Variable] {
        &self.inputs
    }

    /// Adds an input with a specified gain.
    pub fn add_input(&mut self, input: Variable, gain: Numeric) {
        self.inputs.push(input);
        self.gains.push(gain);
    }

    /// Adds an input with unit gain (1.0).
    pub fn add_input_unit_gain(&mut self, input: Variable) {
        self.inputs.push(input);
        self.gains.push(Numeric::one());
    }
}

#[cfg(test)]
mod tests;
