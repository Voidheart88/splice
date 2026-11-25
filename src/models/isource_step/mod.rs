/// The ISourceStep Module. This module encapsulates everything regarding a step current source bundle.
pub(crate) mod serde;
pub(crate) mod spice;

use super::*;
use std::sync::Arc;

/// A structure representing a step current source.
/// The current steps from initial_value to final_value at delay_time.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ISourceStepBundle {
    name: Arc<str>,
    node0: Option<Variable>,
    node1: Option<Variable>,
    initial_value: Numeric,
    final_value: Numeric,
    delay_time: Numeric,
}

impl ISourceStepBundle {
    /// Creates a new `ISourceStepBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the step current source.
    /// * `node0` - The first node.
    /// * `node1` - The second node.
    /// * `initial_value` - The initial current value (before delay_time).
    /// * `final_value` - The final current value (after delay_time).
    /// * `delay_time` - The time at which the step occurs.
    ///
    /// # Returns
    ///
    /// A new `ISourceStepBundle` object.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        initial_value: Numeric,
        final_value: Numeric,
        delay_time: Numeric,
    ) -> Self {
        ISourceStepBundle {
            name,
            node0,
            node1,
            initial_value,
            final_value,
            delay_time,
        }
    }

    /// Returns the name of the step current source.
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

    /// Returns the current value at a given time.
    /// For transient analysis, returns initial_value before delay_time, final_value after.
    pub fn current_at_time(&self, time: Numeric) -> Numeric {
        if time < self.delay_time {
            self.initial_value
        } else {
            self.final_value
        }
    }
}

#[cfg(test)]
mod tests;
