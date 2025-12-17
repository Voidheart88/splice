use std::sync::Arc;
use crate::spot::Numeric;
use crate::models::{Variable, Triples, TripleIdx};
use crate::frontends::spice::{Rule, ProcessSpiceElement};
use crate::{Element, FrontendError};
use pest::iterators::Pair;

use num::Complex;

type ComplexNumeric = Complex<Numeric>;

/// Options for a voltage-controlled voltage source (VCVS)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VCVSOptions {
    /// Voltage gain (output voltage = gain * control voltage)
    pub gain: Numeric,
}

impl Default for VCVSOptions {
    fn default() -> Self {
        Self { gain: 1.0 }
    }
}

/// Voltage-Controlled Voltage Source (VCVS) - E source
/// This represents a voltage source whose output voltage is proportional
/// to the voltage across a controlling pair of nodes.
/// SPICE syntax: E{name} {pos} {neg} {ctrl_pos} {ctrl_neg} {gain}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VCVSBundle {
    name: Arc<str>,
    /// Positive output node
    positive: Option<Variable>,
    /// Negative output node  
    negative: Option<Variable>,
    /// Positive controlling node
    controlling_positive: Option<Variable>,
    /// Negative controlling node
    controlling_negative: Option<Variable>,
    /// Source options including gain
    options: VCVSOptions,
}

impl VCVSBundle {
    /// Creates a new VCVS bundle
    pub fn new(
        name: Arc<str>,
        positive: Option<Variable>,
        negative: Option<Variable>,
        controlling_positive: Option<Variable>,
        controlling_negative: Option<Variable>,
        options: Option<VCVSOptions>,
    ) -> Self {
        Self {
            name,
            positive,
            negative,
            controlling_positive,
            controlling_negative,
            options: options.unwrap_or_default(),
        }
    }

    /// Returns the name of the VCVS
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the constant triples for the VCVS
    /// VCVS equation: V(out) = gain * (V(ctrl_pos) - V(ctrl_neg))
    /// This contributes to the matrix as:
    /// pos * gain * ctrl_pos
    /// pos * -gain * ctrl_neg
    /// neg * -gain * ctrl_pos
    /// neg * gain * ctrl_neg
    pub fn triples(&self) -> Triples<Numeric, 4> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_pos_idx), Some(ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_positive.as_ref().map(|v| v.idx()),
            self.controlling_negative.as_ref().map(|v| v.idx()),
        ) {
            // VCVS matrix contributions
            Triples::new(&[
                (pos_idx, ctrl_pos_idx, self.options.gain),
                (pos_idx, ctrl_neg_idx, -self.options.gain),
                (neg_idx, ctrl_pos_idx, -self.options.gain),
                (neg_idx, ctrl_neg_idx, self.options.gain),
            ])
        } else {
            Triples::new(&[])
        }
    }

    /// Returns the triple indices for the VCVS
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_pos_idx), Some(ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_positive.as_ref().map(|v| v.idx()),
            self.controlling_negative.as_ref().map(|v| v.idx()),
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

    /// Returns the AC triples for the VCVS
    /// For AC analysis, the VCVS behaves the same as in DC since it's a linear element
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_pos_idx), Some(ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_positive.as_ref().map(|v| v.idx()),
            self.controlling_negative.as_ref().map(|v| v.idx()),
        ) {
            // VCVS AC contributions (same as DC since it's linear and frequency-independent)
            Triples::new(&[
                (pos_idx, ctrl_pos_idx, Complex::new(self.options.gain, 0.0)),
                (pos_idx, ctrl_neg_idx, Complex::new(-self.options.gain, 0.0)),
                (neg_idx, ctrl_pos_idx, Complex::new(-self.options.gain, 0.0)),
                (neg_idx, ctrl_neg_idx, Complex::new(self.options.gain, 0.0)),
            ])
        } else {
            Triples::new(&[])
        }
    }

    /// Returns the node indices for the VCVS
    pub fn node_indices(&self) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
        (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_positive.as_ref().map(|v| v.idx()),
            self.controlling_negative.as_ref().map(|v| v.idx()),
        )
    }
}

impl ProcessSpiceElement for VCVSBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Delegate to the module-level function
        super::spice::process_vcvs(element, variables, elements, var_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::models::Unit;

    fn create_var(name: &str, idx: usize) -> Variable {
        Variable::new(Arc::from(name), Unit::Volt, idx)
    }

    #[test]
    fn test_vcvs_creation() {
        let vcvs = VCVSBundle::new(
            Arc::from("E1"),
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(vcvs.name(), Arc::from("E1"));
        assert_eq!(vcvs.options.gain, 1.0);
    }

    #[test]
    fn test_vcvs_with_options() {
        let options = VCVSOptions { gain: 10.0 };
        let vcvs = VCVSBundle::new(
            Arc::from("E1"),
            None,
            None,
            None,
            None,
            Some(options),
        );
        assert_eq!(vcvs.options.gain, 10.0);
    }

    #[test]
    fn test_vcvs_triples() {
        let v1 = create_var("1", 0);
        let v2 = create_var("2", 1);
        let v3 = create_var("3", 2);
        let v4 = create_var("4", 3);

        let vcvs = VCVSBundle::new(
            Arc::from("E1"),
            Some(v3.clone()),
            Some(v4.clone()),
            Some(v1.clone()),
            Some(v2.clone()),
            Some(VCVSOptions { gain: 2.0 }),
        );

        let triples = vcvs.triples();
        assert_eq!(triples.len(), 4);

        // Check specific triple values
        let data = triples.data();
        assert_eq!(data[0], (2, 0, 2.0));  // pos-ctrl_pos with gain
        assert_eq!(data[1], (2, 1, -2.0)); // pos-ctrl_neg with -gain
        assert_eq!(data[2], (3, 0, -2.0)); // neg-ctrl_pos with -gain
        assert_eq!(data[3], (3, 1, 2.0));  // neg-ctrl_neg with gain
    }

    #[test]
    fn test_vcvs_triple_indices() {
        let v1 = create_var("1", 0);
        let v2 = create_var("2", 1);
        let v3 = create_var("3", 2);
        let v4 = create_var("4", 3);

        let vcvs = VCVSBundle::new(
            Arc::from("E1"),
            Some(v3.clone()),
            Some(v4.clone()),
            Some(v1.clone()),
            Some(v2.clone()),
            None,
        );

        let indices = vcvs.triple_idx().unwrap();
        let data = indices.data();
        assert_eq!(data[0], (2, 0));
        assert_eq!(data[1], (2, 1));
        assert_eq!(data[2], (3, 0));
        assert_eq!(data[3], (3, 1));
    }
}