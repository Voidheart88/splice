use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::models::{TripleIdx, Triples, Variable};
use crate::spot::Numeric;
use crate::{Element, FrontendError};
use pest::iterators::Pair;
use std::sync::Arc;

use num::Complex;

type ComplexNumeric = Complex<Numeric>;

/// Options for a voltage-controlled current source (VCCS)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VCCSOptions {
    /// Transconductance (output current = transconductance * control voltage)
    pub transconductance: Numeric,
}

impl Default for VCCSOptions {
    fn default() -> Self {
        Self {
            transconductance: 1.0,
        }
    }
}

/// Voltage-Controlled Current Source (VCCS) - G source
/// This represents a current source whose output current is proportional
/// to the voltage across a controlling pair of nodes.
/// SPICE syntax: G{name} {pos} {neg} {ctrl_pos} {ctrl_neg} {transconductance}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VCCSBundle {
    name: Arc<str>,
    /// Positive output node
    positive: Option<Variable>,
    /// Negative output node  
    negative: Option<Variable>,
    /// Positive controlling node
    controlling_positive: Option<Variable>,
    /// Negative controlling node
    controlling_negative: Option<Variable>,
    /// Source options including transconductance
    options: VCCSOptions,
}

impl VCCSBundle {
    /// Creates a new VCCS bundle
    pub fn new(
        name: Arc<str>,
        positive: Option<Variable>,
        negative: Option<Variable>,
        controlling_positive: Option<Variable>,
        controlling_negative: Option<Variable>,
        options: Option<VCCSOptions>,
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

    /// Returns the name of the VCCS
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns the constant triples for the VCCS
    /// VCCS equation: I(out) = transconductance * (V(ctrl_pos) - V(ctrl_neg))
    /// This contributes to the matrix as:
    /// pos * trans * ctrl_pos
    /// pos * -trans * ctrl_neg
    /// neg * -trans * ctrl_pos
    /// neg * trans * ctrl_neg
    pub fn triples(&self) -> Triples<Numeric, 4> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_pos_idx), Some(ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_positive.as_ref().map(|v| v.idx()),
            self.controlling_negative.as_ref().map(|v| v.idx()),
        ) {
            // VCCS matrix contributions
            Triples::new(&[
                (pos_idx, ctrl_pos_idx, self.options.transconductance),
                (pos_idx, ctrl_neg_idx, -self.options.transconductance),
                (neg_idx, ctrl_pos_idx, -self.options.transconductance),
                (neg_idx, ctrl_neg_idx, self.options.transconductance),
            ])
        } else {
            Triples::new(&[])
        }
    }

    /// Returns the triple indices for the VCCS
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

    /// Returns the node indices for the VCCS
    pub fn node_indices(&self) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
        (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_positive.as_ref().map(|v| v.idx()),
            self.controlling_negative.as_ref().map(|v| v.idx()),
        )
    }

    /// Returns the AC triples for the VCCS
    /// For AC analysis, the VCCS behaves the same as in DC since it's a linear element
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_pos_idx), Some(ctrl_neg_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_positive.as_ref().map(|v| v.idx()),
            self.controlling_negative.as_ref().map(|v| v.idx()),
        ) {
            // VCCS AC contributions (same as DC since it's linear and frequency-independent)
            // FIXME: This nests too deep and needs a refactor
            Triples::new(&[
                (
                    pos_idx,
                    ctrl_pos_idx,
                    Complex::new(self.options.transconductance, 0.0),
                ),
                (
                    pos_idx,
                    ctrl_neg_idx,
                    Complex::new(-self.options.transconductance, 0.0),
                ),
                (
                    neg_idx,
                    ctrl_pos_idx,
                    Complex::new(-self.options.transconductance, 0.0),
                ),
                (
                    neg_idx,
                    ctrl_neg_idx,
                    Complex::new(self.options.transconductance, 0.0),
                ),
            ])
        } else {
            Triples::new(&[])
        }
    }
}

impl ProcessSpiceElement for VCCSBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Delegate to the module-level function
        super::spice::process_vccs(element, variables, elements, var_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Unit;
    use std::sync::Arc;

    fn create_var(name: &str, idx: usize) -> Variable {
        Variable::new(Arc::from(name), Unit::Volt, idx)
    }

    #[test]
    fn test_vccs_creation() {
        let vccs = VCCSBundle::new(Arc::from("G1"), None, None, None, None, None);
        assert_eq!(vccs.name(), Arc::from("G1"));
        assert_eq!(vccs.options.transconductance, 1.0);
    }

    #[test]
    fn test_vccs_with_options() {
        let options = VCCSOptions {
            transconductance: 0.1,
        };
        let vccs = VCCSBundle::new(Arc::from("G1"), None, None, None, None, Some(options));
        assert_eq!(vccs.options.transconductance, 0.1);
    }

    #[test]
    fn test_vccs_triples() {
        let v1 = create_var("1", 0);
        let v2 = create_var("2", 1);
        let v3 = create_var("3", 2);
        let v4 = create_var("4", 3);

        let vccs = VCCSBundle::new(
            Arc::from("G1"),
            Some(v3.clone()),
            Some(v4.clone()),
            Some(v1.clone()),
            Some(v2.clone()),
            Some(VCCSOptions {
                transconductance: 0.05,
            }),
        );

        let triples = vccs.triples();
        assert_eq!(triples.len(), 4);

        // Check specific triple values
        let data = triples.data();
        assert_eq!(data[0], (2, 0, 0.05)); // pos-ctrl_pos with transconductance
        assert_eq!(data[1], (2, 1, -0.05)); // pos-ctrl_neg with -transconductance
        assert_eq!(data[2], (3, 0, -0.05)); // neg-ctrl_pos with -transconductance
        assert_eq!(data[3], (3, 1, 0.05)); // neg-ctrl_neg with transconductance
    }
}
