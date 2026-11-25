/// The ISourcePulse Module. This module encapsulates everything regarding a pulse current source bundle.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use std::sync::Arc;

/// A structure representing a pulse current source.
///
/// The pulse current source generates a pulsed current waveform with the following parameters:
/// - Starts at initial_value
/// - After delay_time, rises to pulsed_value over rise_time
/// - Stays at pulsed_value for pulse_width
/// - Falls back to initial_value over fall_time
/// - Repeats every period
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ISourcePulseBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    initial_value: Numeric,
    pulsed_value: Numeric,
    delay_time: Numeric,
    rise_time: Numeric,
    fall_time: Numeric,
    pulse_width: Numeric,
    period: Numeric,
}

impl ISourcePulseBundle {
    /// Creates a new `ISourcePulseBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the pulse current source.
    /// * `node0` - The first node.
    /// * `node1` - The second node.
    /// * `initial_value` - The initial current value (before pulse).
    /// * `pulsed_value` - The current value during the pulse.
    /// * `delay_time` - The time before the pulse starts.
    /// * `rise_time` - The time to rise from initial to pulsed value.
    /// * `fall_time` - The time to fall from pulsed to initial value.
    /// * `pulse_width` - The duration of the pulse at pulsed_value.
    /// * `period` - The total period of the pulse (0 for single pulse).
    ///
    /// # Returns
    ///
    /// A new `ISourcePulseBundle` object.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        initial_value: Numeric,
        pulsed_value: Numeric,
        delay_time: Numeric,
        rise_time: Numeric,
        fall_time: Numeric,
        pulse_width: Numeric,
        period: Numeric,
    ) -> Self {
        ISourcePulseBundle {
            name,
            node0,
            node1,
            initial_value,
            pulsed_value,
            delay_time,
            rise_time,
            fall_time,
            pulse_width,
            period,
        }
    }

    /// Returns the name of the pulse current source.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the pairs representing the current source contributions to the vector b.
    /// For DC/OP analysis, uses the initial_value.
    pub fn pairs(&self) -> Pairs<Numeric, 2> {
        let value = self.initial_value;
        match (&self.node0, &self.node1) {
            (None, None) => Pairs::new(&[]),
            (Some(node0), None) => Pairs::new(&[(node0.idx(), -value)]),
            (None, Some(node1)) => Pairs::new(&[(node1.idx(), value)]),
            (Some(node0), Some(node1)) => {
                Pairs::new(&[(node0.idx(), -value), (node1.idx(), value)])
            }
        }
    }

    /// Returns the current value at a given time for transient analysis.
    ///
    /// The pulse waveform:
    /// - t < delay_time: initial_value
    /// - delay_time <= t < delay_time + rise_time: linear rise to pulsed_value
    /// - delay_time + rise_time <= t < delay_time + rise_time + pulse_width: pulsed_value
    /// - delay_time + rise_time + pulse_width <= t < delay_time + rise_time + pulse_width + fall_time: linear fall to initial_value
    /// - t >= delay_time + rise_time + pulse_width + fall_time: initial_value (or repeats if period > 0)
    pub fn current_at_time(&self, time: Numeric) -> Numeric {
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
}

#[cfg(test)]
mod tests;
