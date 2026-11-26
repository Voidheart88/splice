/// The ISourceSin Module. This module encapsulates everything regarding a sinusoidal current source bundle.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use num::Complex;
use std::sync::Arc;

/// A structure representing a sinusoidal current source.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ISourceSinBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    dc_offset: Numeric,
    amplitude: Numeric,
    frequency: Numeric,
    phase: Numeric,
    ac_value: Option<Numeric>,
}

impl ISourceSinBundle {
    /// Creates a new `ISourceSinBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sinusoidal current source.
    /// * `node0` - The first node.
    /// * `node1` - The second node.
    /// * `dc_offset` - The DC offset of the sinusoidal current source.
    /// * `amplitude` - The amplitude of the sinusoidal current source.
    /// * `frequency` - The frequency of the sinusoidal current source.
    /// * `phase` - The phase of the sinusoidal current source.
    /// * `ac_value` - The AC value of the sinusoidal current source.
    ///
    /// # Returns
    ///
    /// A new `ISourceSinBundle` object.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        dc_offset: Numeric,
        amplitude: Numeric,
        frequency: Numeric,
        phase: Numeric,
        ac_value: Option<Numeric>,
    ) -> Self {
        ISourceSinBundle {
            name,
            node0,
            node1,
            dc_offset,
            amplitude,
            frequency,
            phase,
            ac_value,
        }
    }

    /// Returns the name of the sinusoidal current source.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the pairs representing the current source contributions to the vector b.
    /// For DC/OP analysis, uses the DC offset value.
    pub fn pairs(&self) -> Pairs<Numeric, 2> {
        match (&self.node0, &self.node1) {
            (None, None) => Pairs::new(&[]),
            (Some(node0), None) => Pairs::new(&[(node0.idx(), -self.dc_offset)]),
            (None, Some(node1)) => Pairs::new(&[(node1.idx(), self.dc_offset)]),
            (Some(node0), Some(node1)) => Pairs::new(&[
                (node0.idx(), -self.dc_offset),
                (node1.idx(), self.dc_offset),
            ]),
        }
    }

    /// Returns the AC pairs for AC analysis.
    /// Uses the AC value if provided, otherwise uses amplitude with phase.
    pub fn ac_pairs(&self) -> Pairs<ComplexNumeric, 2> {
        let ac_current = if let Some(ac) = self.ac_value {
            Complex::new(ac, 0.0)
        } else {
            // Convert amplitude and phase to complex phasor
            let phase_rad = self.phase.to_radians();
            Complex::from_polar(self.amplitude, phase_rad)
        };

        match (&self.node0, &self.node1) {
            (None, None) => Pairs::new(&[]),
            (Some(node0), None) => Pairs::new(&[(node0.idx(), -ac_current)]),
            (None, Some(node1)) => Pairs::new(&[(node1.idx(), ac_current)]),
            (Some(node0), Some(node1)) => {
                Pairs::new(&[(node0.idx(), -ac_current), (node1.idx(), ac_current)])
            }
        }
    }

    /// Returns the pairs representing the current source contributions to the vector b.
    /// For transient analysis, uses the sinusoidal current at the given time.
    pub fn pairs_at_time(&self, time: Option<&Numeric>) -> Pairs<Numeric, 2> {
        let value = match time {
            Some(t) => {
                // Calculate sinusoidal current: dc_offset + amplitude * sin(2*pi*frequency*t + phase)
                let angular_freq = 2.0 * std::f64::consts::PI * self.frequency;
                let phase_rad = self.phase.to_radians();
                self.dc_offset + self.amplitude * (angular_freq * *t + phase_rad).sin()
            }
            None => self.dc_offset,
        };
        match (&self.node0, &self.node1) {
            (None, None) => Pairs::new(&[]),
            (Some(node0), None) => Pairs::new(&[(node0.idx(), -value)]),
            (None, Some(node1)) => Pairs::new(&[(node1.idx(), value)]),
            (Some(node0), Some(node1)) => {
                Pairs::new(&[(node0.idx(), -value), (node1.idx(), value)])
            }
        }
    }
}

#[cfg(test)]
mod tests;
