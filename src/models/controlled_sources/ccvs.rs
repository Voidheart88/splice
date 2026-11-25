// Current-Controlled Voltage Source (CCVS) - H source

use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::models::{TripleIdx, Triples, Variable};
use crate::spot::Numeric;
use crate::{Element, FrontendError};
use pest::iterators::Pair;
use std::sync::Arc;

use num::Complex;

type ComplexNumeric = Complex<Numeric>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CCVSOptions {
    pub gain: Numeric,
}

impl Default for CCVSOptions {
    fn default() -> Self {
        Self { gain: 1.0 }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CCVSBundle {
    name: Arc<str>,
    positive: Option<Variable>,
    negative: Option<Variable>,
    controlling_branch: Option<Variable>,
    options: CCVSOptions,
}

impl CCVSBundle {
    pub fn new(
        name: Arc<str>,
        positive: Option<Variable>,
        negative: Option<Variable>,
        controlling_branch: Option<Variable>,
        options: Option<CCVSOptions>,
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

    /// Returns the constant triples for the CCVS
    /// CCVS equation: V(out) = gain * I(control)
    /// Note: CCVS requires special MNA handling as it's a voltage source
    /// This provides the control relationship triples
    pub fn triples(&self) -> Triples<Numeric, 4> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_branch_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
        ) {
            // CCVS matrix contributions
            // The voltage across pos-neg is controlled by current through ctrl_branch
            Triples::new(&[
                (pos_idx, ctrl_branch_idx, self.options.gain),
                (neg_idx, ctrl_branch_idx, -self.options.gain),
            ])
        } else {
            Triples::new(&[])
        }
    }

    /// Returns the triple indices for the CCVS
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_branch_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
        ) {
            Some(TripleIdx::new(&[
                (pos_idx, ctrl_branch_idx),
                (neg_idx, ctrl_branch_idx),
            ]))
        } else {
            None
        }
    }

    /// Returns the AC triples for the CCVS
    /// For AC analysis, the CCVS behaves the same as in DC since it's a linear element
    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_branch_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
        ) {
            let gain = ComplexNumeric::new(self.options.gain, 0.0);
            Triples::new(&[
                (pos_idx, ctrl_branch_idx, gain),
                (neg_idx, ctrl_branch_idx, -gain),
            ])
        } else {
            Triples::new(&[])
        }
    }
}

impl ProcessSpiceElement for CCVSBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Delegate to the module-level function
        crate::models::controlled_sources::spice::process_ccvs(
            element, variables, elements, var_map,
        )
    }
}
