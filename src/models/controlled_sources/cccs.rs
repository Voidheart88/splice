// Current-Controlled Current Source (CCCS) - F source

use crate::frontends::spice::{ProcessSpiceElement, Rule};
use crate::models::{TripleIdx, Triples, Variable};
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

    /// Returns the constant triples for the CCCS
    /// CCCS equation: I(out) = gain * I(control)
    /// This contributes to the matrix as:
    /// pos * gain * ctrl_branch
    /// neg * -gain * ctrl_branch
    pub fn triples(&self) -> Triples<Numeric, 4> {
        if let (Some(pos_idx), Some(neg_idx), Some(ctrl_branch_idx)) = (
            self.positive.as_ref().map(|v| v.idx()),
            self.negative.as_ref().map(|v| v.idx()),
            self.controlling_branch.as_ref().map(|v| v.idx()),
        ) {
            Triples::new(&[
                (pos_idx, ctrl_branch_idx, self.options.gain),
                (neg_idx, ctrl_branch_idx, -self.options.gain),
            ])
        } else {
            Triples::new(&[])
        }
    }

    /// Returns the triple indices for the CCCS
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

    /// Returns the AC triples for the CCCS
    /// For AC analysis, the CCCS behaves the same as in DC since it's a linear element
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

impl ProcessSpiceElement for CCCSBundle {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        // Delegate to the module-level function
        crate::models::controlled_sources::spice::process_cccs(
            element, variables, elements, var_map,
        )
    }
}
