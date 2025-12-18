// Current-Controlled Voltage Source (CCVS) - H source
// FIXME: Placeholder implementation - Complete this

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

    pub fn triples(&self) -> Triples<Numeric, 4> {
        Triples::new(&[])
    }

    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        None
    }

    pub fn ac_triples(&self) -> Triples<ComplexNumeric, 4> {
        Triples::new(&[])
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
        super::spice::process_ccvs(element, variables, elements, var_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_ccvs_creation() {
        let ccvs = CCVSBundle::new(Arc::from("H1"), None, None, None, None);
        assert_eq!(ccvs.name(), Arc::from("H1"));
    }
}
