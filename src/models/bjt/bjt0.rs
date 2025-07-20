#![allow(unused)]
use std::sync::Arc;

use super::super::*;
use crate::spot::*;

/// A structure representing a BJT.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Bjt0Bundle {
    name: Arc<str>,
    base: Option<Variable>,
    collector: Option<Variable>,
    emitter: Option<Variable>,
    value: Bjt0Options,
}

/// An enum representing possible Bjt Ebers Moll options.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Bjt0Options {
    is: Numeric,
    n: Numeric,
    beta: Numeric,
}

impl Default for Bjt0Options {
    fn default() -> Self {
        Self {
            is: 1e-14,
            n: 1.0,
            beta: 100.0,
        }
    }
}

impl Bjt0Bundle {
    pub fn new(
        name: Arc<str>,
        base: Option<Variable>,
        collector: Option<Variable>,
        emitter: Option<Variable>,
        value: Option<Bjt0Options>,
    ) -> Bjt0Bundle {
        let value = match value {
            Some(v) => v,
            None => Bjt0Options::default(),
        };

        Bjt0Bundle {
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
    pub fn triples(&self, _x_vec: &Vec<Numeric>) -> Triples<Numeric, 4> {
        todo!()
    }

    /// Returns a reference to the pairs representing vector b.
    pub fn pairs(&self, _x_vec: &Vec<Numeric>) -> Pairs<Numeric, 2> {
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
