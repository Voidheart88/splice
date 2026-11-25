/// The Integrator Module. This module encapsulates everything regarding an integrator block.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use crate::spot::*;
use std::sync::Arc;

/// A structure representing an integrator block.
///
/// The integrator performs time-domain integration:
/// output(t) = ∫ input(t) dt
///
/// In discrete time using backward Euler:
/// output[n] = output[n-1] + dt * input[n]
///
/// Rearranged for MNA stamp:
/// output[n] - dt * input[n] = output[n-1]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IntegratorBundle {
    name: Arc<str>,
    input: Option<Variable>,
    output: Option<Variable>,
    gain: Numeric,
    initial_condition: Numeric,
    previous_output: Numeric,
}

impl IntegratorBundle {
    /// Creates a new `IntegratorBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the integrator block.
    /// * `input` - The input variable.
    /// * `output` - The output variable.
    /// * `gain` - The integration gain (default 1.0).
    /// * `initial_condition` - The initial output value at t=0.
    ///
    /// # Returns
    ///
    /// A new `IntegratorBundle` object.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Arc<str>,
        input: Option<Variable>,
        output: Option<Variable>,
        gain: Numeric,
        initial_condition: Numeric,
    ) -> Self {
        IntegratorBundle {
            name,
            input,
            output,
            gain,
            initial_condition,
            previous_output: initial_condition,
        }
    }

    /// Returns the name of the integrator block.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the index of the input variable.
    pub fn input_idx(&self) -> Option<usize> {
        self.input.as_ref().map(|v| v.idx())
    }

    /// Returns the index of the output variable.
    pub fn output_idx(&self) -> Option<usize> {
        self.output.as_ref().map(|v| v.idx())
    }

    /// Returns the indices for the triples in matrix A.
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        match (self.input_idx(), self.output_idx()) {
            (Some(input_idx), Some(output_idx)) => Some(TripleIdx::new(&[
                (output_idx, output_idx),
                (output_idx, input_idx),
            ])),
            _ => None,
        }
    }

    /// Returns the triples representing the matrix A for the integrator.
    ///
    /// For backward Euler integration:
    /// output[n] - dt * gain * input[n] = output[n-1]
    ///
    /// Matrix form:
    /// [output_idx, output_idx] = 1
    /// [output_idx, input_idx] = -dt * gain
    pub fn triples(&self, delta_t: Option<&Numeric>) -> Triples<Numeric, 4> {
        let input_idx = match self.input_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };
        let output_idx = match self.output_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };

        let delta_t = match delta_t {
            Some(dt) => *dt,
            None => return Triples::new(&[]),
        };

        // output[n] - dt * gain * input[n] = output[n-1]
        // Matrix: [1, -dt*gain]
        Triples::new(&[
            (output_idx, output_idx, Numeric::one()),
            (output_idx, input_idx, -delta_t * self.gain),
        ])
    }

    /// Returns the pairs representing the vector b (RHS) for the integrator.
    ///
    /// The RHS contains the history term: output[n-1]
    pub fn pairs(&self, _x_vec: &[Numeric]) -> Pairs<Numeric, 2> {
        let output_idx = match self.output_idx() {
            Some(idx) => idx,
            None => return Pairs::new(&[]),
        };

        // RHS: output[n-1] (previous output value)
        Pairs::new(&[(output_idx, self.previous_output)])
    }

    /// Returns the AC triples for small-signal analysis.
    ///
    /// In the frequency domain, an integrator has transfer function H(s) = gain/s.
    /// For AC analysis at frequency f, s = j*2*pi*f.
    /// This is handled by the simulator, so we return a simple gain relationship.
    pub fn ac_triples(&self, _freq: Numeric) -> Triples<ComplexNumeric, 4> {
        // AC analysis for integrators is frequency-dependent
        // This is a placeholder - full implementation would need complex gain
        let input_idx = match self.input_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };
        let output_idx = match self.output_idx() {
            Some(idx) => idx,
            None => return Triples::new(&[]),
        };

        // For AC, we use a simplified model
        Triples::new(&[(
            output_idx,
            input_idx,
            ComplexNumeric {
                re: -self.gain,
                im: 0.0,
            },
        )])
    }

    /// Updates the previous output value for the next time step.
    ///
    /// # Arguments
    ///
    /// * `x_vec` - The current solution vector.
    pub fn update_previous_output(&mut self, x_vec: &[Numeric]) {
        if let Some(output_idx) = self.output_idx() {
            if output_idx < x_vec.len() {
                self.previous_output = x_vec[output_idx];
            }
        }
    }

    /// Resets the integrator to its initial condition.
    pub fn reset(&mut self) {
        self.previous_output = self.initial_condition;
    }

    /// Returns the current gain value.
    pub fn gain(&self) -> Numeric {
        self.gain
    }

    /// Returns the initial condition.
    pub fn initial_condition(&self) -> Numeric {
        self.initial_condition
    }

    /// Returns the previous output value (history term).
    pub fn previous_output(&self) -> Numeric {
        self.previous_output
    }
}

#[cfg(test)]
mod tests;
