// Implementation of current-controlled current sources for SPICE simulation
// This element models F sources where output current is proportional to controlling current
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e
=======
// Current-Controlled Current Source (CCCS) - F source
// Implementation of current-controlled current sources for SPICE simulation
// This element models F sources where output current is proportional to controlling current
=======
// Implementation of current-controlled current sources for SPICE simulation
// This element models F sources where output current is proportional to controlling current
>>>>>>> 25bca9d83d58b511eb2e0eadfa6fe1ecd3e23f1e

use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::models::{Pairs, TripleIdx, Triples, Variable};
use crate::spot::Numeric;
use crate::{Element, FrontendError};
use pest::iterators::Pair;
use std::sync::Arc;

use num::Complex;

type ComplexNumeric = Complex<Numeric>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CCCSOptions {
    pub gain: Numeric,
}

impl Default for CCCSOptions {
    fn default() -> Self {
        Self { gain: 1.0 }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CCCSBundle {
    name: Arc<str>,
    positive: Option<Variable>,
    negative: Option<Variable>,
    controlling_branch: Option<Variable>,
    options: CCCSOptions,
}

impl CCCSBundle {
    pub fn new(
        name: Arc<str>,
        positive: Option<Variable>,
        negative: Option<Variable>,
        controlling_branch: Option<Variable>,
        options: Option<CCCSOptions>,
    ) -> Self {
        Self {
            name,
            positive,
            negative,
            controlling_branch,
            options: options.unwrap_or_default(),
        }
    }

    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn triples(&self) -> Triples<Numeric, 4> {
        // CCCS contributes to the MNA matrix as a transconductance element
        // For DC/OP analysis, we need to handle the controlling current
        // In MNA, we represent this as: I_out = gain * I_controlling
        // Since we don't have direct access to branch currents in basic MNA,
        // we estimate the controlling current from the voltage across a small resistor
        
        if let (Some(_pos_idx), Some(_neg_idx), Some(_ctrl_pos_idx), Some(_ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
        ) {
            // For CCCS, we contribute to the transconductance matrix
            // This is a simplified representation - a full implementation would need
            // to handle the controlling branch current more accurately
            Triples::new(&[])
        } else {
            Triples::new(&[])
        }
    }

    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_pos_idx), Some(ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
        ) {
            Some(TripleIdx::new(&[
                (pos_idx, ctrl_pos_idx),
                (pos_idx, ctrl_neg_idx),
                (neg_idx, ctrl_pos_idx),
                (neg_idx, ctrl_neg_idx),
            ]))
        } else {
            None
        }
    }

    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        // For AC analysis, CCCS behaves like a transconductance
        // I_out = gain * I_controlling
        // In AC, we can represent this as a complex transconductance
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_pos_idx), Some(ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
        ) {
            let gain = Complex::new(self.options.gain, 0.0);
            // CCCS contributes to the transconductance matrix in AC analysis
            // This creates a relationship between controlling voltage and output current
            Triples::new(&[
                (pos_idx, ctrl_pos_idx, gain),
                (pos_idx, ctrl_neg_idx, -gain),
                (neg_idx, ctrl_pos_idx, -gain),
                (neg_idx, ctrl_neg_idx, gain),
            ])
        } else {
            Triples::new(&[])
        }
    }

    /// Calculate output current for transient analysis
    /// I_out = gain * I_controlling
    /// For MNA, we estimate controlling current from voltage across controlling branch
    pub fn calculate_output_current(&self, controlling_voltage: Numeric, controlling_resistance: Numeric) -> Numeric {
        // Estimate controlling current: I_controlling ≈ V_controlling / R_controlling
        let controlling_current = if controlling_resistance.abs() > 1e-12 {
            controlling_voltage / controlling_resistance
        } else {
            0.0 // Avoid division by zero for very small resistances
        };
        // Output current: I_out = gain * I_controlling
        self.options.gain * controlling_current
    }

    /// Get pairs for current contributions in transient analysis
    pub fn get_pairs(&self, output_current: Numeric) -> Pairs<Numeric, 2> {
        if let (Some(pos_idx), Some(neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
        ) {
            Pairs::new(&[
                (pos_idx, -output_current), // Current flows out of positive node
                (neg_idx, output_current),  // Current flows into negative node
            ])
        } else {
            Pairs::new(&[])
        }
    }
}

impl ProcessSpiceElement for CCCSBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Delegate to the module-level function
        super::spice::process_cccs(element, variables, elements, var_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_cccs_creation() {
        let cccs = CCCSBundle::new(Arc::from("F1"), None, None, None, None);
        assert_eq!(cccs.name(), Arc::from("F1"));
    }
}
