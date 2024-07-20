#![allow(unused)]
use std::sync::Arc;

use crate::consts::UT;

use super::*;

/// A structure representing a BJT.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct BjtEMollBundle {
    name: Arc<str>,
    base: Option<Variable>,
    collector: Option<Variable>,
    emitter: Option<Variable>,
    value: BjtEMollOptions,
}

/// An enum representing possible Bjt Ebers Moll options.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct BjtEMollOptions {
    is: f64,
    n: f64,
    beta: f64
}

impl Default for BjtEMollOptions {
    fn default() -> Self {
        Self {
            is: 1e-14,
            n: 1.0,
            beta: 100.0
        }
    }
}

impl BjtEMollBundle {
    pub fn new(
        name: Arc<str>,
        base: Option<Variable>,
        collector: Option<Variable>,
        emitter: Option<Variable>,
        value: Option<BjtEMollOptions>,
    ) -> BjtEMollBundle {
        let value = match value {
            Some(v) => v,
            None => BjtEMollOptions::default(),
        };

        BjtEMollBundle {
            name,
            base,
            collector,
            emitter,
            value,
        }
    }

    /// Returns the name of the diode bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self, _x_vec: &Vec<f64>) -> Triples {
        todo!()
    }

    /// Returns a reference to the pairs representing vector b.
    pub fn pairs(&self, _x_vec: &Vec<f64>) -> Pairs {
        todo!()
    }

    pub fn b_idx(&self) -> Option<usize> {
        match &self.base {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }

    pub fn c_idx(&self) -> Option<usize> {
        match &self.collector {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }

    pub fn e_idx(&self) -> Option<usize> {
        match &self.emitter {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
