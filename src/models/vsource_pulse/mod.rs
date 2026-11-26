/// The VSourcePulse Module. This module encapsulates everything regarding a pulse voltage source bundle.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use num::Zero;
use std::sync::Arc;

/// A structure representing a pulse voltage source.
///
/// The pulse voltage source generates a pulsed voltage waveform with the following parameters:
/// - Starts at initial_value
/// - After delay_time, rises to pulsed_value over rise_time
/// - Stays at pulsed_value for pulse_width
/// - Falls back to initial_value over fall_time
/// - Repeats every period
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VSourcePulseBundle {
    name: Arc<str>,
    branch: Variable,
    node0: Option<Variable>,
    node1: Option<Variable>,
    initial_value: Numeric,
    pulsed_value: Numeric,
    delay_time: Numeric,
    rise_time: Numeric,
    fall_time: Numeric,
    pulse_width: Numeric,
    period: Numeric,
    ac_value: Option<Numeric>,
}

impl VSourcePulseBundle {
    /// Creates a new `VSourcePulseBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the pulse voltage source.
    /// * `branch` - The branch variable for MNA.
    /// * `node0` - The first node.
    /// * `node1` - The second node.
    /// * `initial_value` - The initial voltage value (before pulse).
    /// * `pulsed_value` - The voltage value during the pulse.
    /// * `delay_time` - The time before the pulse starts.
    /// * `rise_time` - The time to rise from initial to pulsed value.
    /// * `fall_time` - The time to fall from pulsed to initial value.
    /// * `pulse_width` - The duration of the pulse at pulsed_value.
    /// * `period` - The total period of the pulse (0 for single pulse).
    /// * `ac_value` - The AC value for small-signal analysis.
    ///
    /// # Returns
    ///
    /// A new `VSourcePulseBundle` object.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Arc<str>,
        branch: Variable,
        node0: Option<Variable>,
        node1: Option<Variable>,
        initial_value: Numeric,
        pulsed_value: Numeric,
        delay_time: Numeric,
        rise_time: Numeric,
        fall_time: Numeric,
        pulse_width: Numeric,
        period: Numeric,
        ac_value: Option<Numeric>,
    ) -> Self {
        VSourcePulseBundle {
            name,
            branch,
            node0,
            node1,
            initial_value,
            pulsed_value,
            delay_time,
            rise_time,
            fall_time,
            pulse_width,
            period,
            ac_value,
        }
    }

    /// Returns the name of the pulse voltage source.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the index of the branch.
    pub fn branch_idx(&self) -> usize {
        self.branch.idx()
    }

    /// Returns the index of node0 if it exists.
    pub fn node0_idx(&self) -> Option<usize> {
        self.node0.as_ref().map(|v| v.idx())
    }

    /// Returns the index of node1 if it exists.
    pub fn node1_idx(&self) -> Option<usize> {
        self.node1.as_ref().map(|v| v.idx())
    }

    /// Returns the voltage value at a given time for transient analysis.
    ///
    /// The pulse waveform:
    /// - t < delay_time: initial_value
    /// - delay_time <= t < delay_time + rise_time: linear rise to pulsed_value
    /// - delay_time + rise_time <= t < delay_time + rise_time + pulse_width: pulsed_value
    /// - delay_time + rise_time + pulse_width <= t < delay_time + rise_time + pulse_width + fall_time: linear fall to initial_value
    /// - t >= delay_time + rise_time + pulse_width + fall_time: initial_value (or repeats if period > 0)
    pub fn voltage_at_time(&self, time: Numeric) -> Numeric {
        // Handle periodic pulses
        let effective_time = if self.period > Numeric::zero() {
            // For periodic pulses, find the time within the current period
            let time_since_delay = time - self.delay_time;
            if time_since_delay < Numeric::zero() {
                return self.initial_value;
            }
            time_since_delay % self.period
        } else {
            // Single pulse - use absolute time
            if time < self.delay_time {
                return self.initial_value;
            }
            time - self.delay_time
        };

        let rise_end = self.rise_time;
        let pulse_end = rise_end + self.pulse_width;
        let fall_end = pulse_end + self.fall_time;

        if effective_time < rise_end {
            // Rising edge - linear interpolation
            if self.rise_time > Numeric::zero() {
                self.initial_value
                    + (self.pulsed_value - self.initial_value) * (effective_time / self.rise_time)
            } else {
                self.pulsed_value
            }
        } else if effective_time < pulse_end {
            // At pulsed value
            self.pulsed_value
        } else if effective_time < fall_end {
            // Falling edge - linear interpolation
            if self.fall_time > Numeric::zero() {
                let fall_progress = (effective_time - pulse_end) / self.fall_time;
                self.pulsed_value - (self.pulsed_value - self.initial_value) * fall_progress
            } else {
                self.initial_value
            }
        } else {
            // After pulse - back to initial value
            self.initial_value
        }
    }

    /// Returns the AC value for small-signal analysis.
    pub fn ac_value(&self) -> Option<Numeric> {
        self.ac_value
    }

    /// Returns the triples representing matrix A.
    pub fn triples(&self) -> Triples<Numeric, 4> {
        let branch_idx = self.branch.idx();
        let node0_idx = self.node0.as_ref().map(|v| v.idx());
        let node1_idx = self.node1.as_ref().map(|v| v.idx());

        // Handle different connection cases using early returns
        if let (None, Some(node1_idx)) = (node0_idx, node1_idx) {
            // Voltage source connected to ground through node1
            return Triples::new(&[
                (branch_idx, node1_idx, Numeric::one()),
                (node1_idx, branch_idx, Numeric::one()),
            ]);
        }
        if let (Some(node0_idx), None) = (node0_idx, node1_idx) {
            // Voltage source connected to ground through node0
            return Triples::new(&[
                (branch_idx, node0_idx, -Numeric::one()),
                (node0_idx, branch_idx, -Numeric::one()),
            ]);
        }
        if let (Some(node0_idx), Some(node1_idx)) = (node0_idx, node1_idx) {
            // Voltage source connected between two nodes
            return Triples::new(&[
                (branch_idx, node0_idx, Numeric::one()),
                (node0_idx, branch_idx, Numeric::one()),
                (branch_idx, node1_idx, -Numeric::one()),
                (node1_idx, branch_idx, -Numeric::one()),
            ]);
        }
        // This should not happen as voltage sources must have at least one connection
        Triples::new(&[])
    }

    /// Returns the triples indices.
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        match (
            self.node0.as_ref().map(|v| v.idx()),
            self.node1.as_ref().map(|v| v.idx()),
        ) {
            (None, None) => None,
            (None, Some(idx_1)) => Some(TripleIdx::new(&[(idx_1, idx_1)])),
            (Some(idx_0), None) => Some(TripleIdx::new(&[(idx_0, idx_0)])),
            (Some(idx_0), Some(idx_1)) => Some(TripleIdx::new(&[
                (idx_0, idx_0),
                (idx_1, idx_1),
                (idx_0, idx_1),
                (idx_1, idx_0),
            ])),
        }
    }

    /// Returns the pairs representing vector b.
    pub fn pairs(&self, time: Option<&Numeric>) -> Pairs<Numeric, 2> {
        let value = match time {
            Some(t) => self.voltage_at_time(*t),
            None => self.initial_value,
        };
        Pairs::new(&[(self.branch.idx(), value)])
    }

    /// Returns the AC triples.
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        let branch_idx = self.branch.idx();
        let node0_idx = self.node0.as_ref().map(|v| v.idx());
        let node1_idx = self.node1.as_ref().map(|v| v.idx());

        if let (Some(node0_idx), Some(node1_idx)) = (node0_idx, node1_idx) {
            Triples::new(&[
                (branch_idx, node0_idx, ComplexNumeric::one()),
                (node0_idx, branch_idx, ComplexNumeric::one()),
                (branch_idx, node1_idx, -ComplexNumeric::one()),
                (node1_idx, branch_idx, -ComplexNumeric::one()),
            ])
        } else if let (Some(node0_idx), None) = (node0_idx, node1_idx) {
            Triples::new(&[
                (branch_idx, node0_idx, -ComplexNumeric::one()),
                (node0_idx, branch_idx, -ComplexNumeric::one()),
            ])
        } else if let (None, Some(node1_idx)) = (node0_idx, node1_idx) {
            Triples::new(&[
                (branch_idx, node1_idx, ComplexNumeric::one()),
                (node1_idx, branch_idx, ComplexNumeric::one()),
            ])
        } else {
            Triples::new(&[])
        }
    }

    /// Returns the AC pairs.
    pub fn ac_pairs(&self) -> Pairs<ComplexNumeric, 2> {
        let Some(ac_val) = self.ac_value else {
            return Pairs::new(&[]);
        };
        Pairs::new(&[(
            self.branch.idx(),
            ComplexNumeric {
                re: ac_val,
                im: Numeric::zero(),
            },
        )])
    }
}

#[cfg(test)]
mod tests;
