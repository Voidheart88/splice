#![allow(unused)]
use std::sync::Arc;

use crate::consts::UT;

use super::super::*;

/// A structure representing a Mos0 Mosfet.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Mos0Bundle {
    name: Arc<str>,
    gate: Option<Variable>,
    drain: Option<Variable>,
    source: Option<Variable>,
    options: Mos0Options,
}

/// An enum representing possible Mosfet MOS0 options.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Mos0Options {
    /// Vt0 threshold Voltage
    vt0: f64,
    /// Tranceconductance
    kp: f64,
}

impl Default for Mos0Options {
    fn default() -> Self {
        Self {
            vt0: 1.0,
            kp: 2.0e-5,
        }
    }
}

impl Mos0Bundle {
    pub fn new(
        name: Arc<str>,
        gate: Option<Variable>,
        drain: Option<Variable>,
        source: Option<Variable>,
        options: Option<Mos0Options>,
    ) -> Mos0Bundle {
        let options = match options {
            Some(v) => v,
            None => Mos0Options::default(),
        };

        Mos0Bundle {
            name,
            gate,
            drain,
            source,
            options,
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

    pub fn g_idx(&self) -> Option<usize> {
        match &self.gate {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }

    pub fn d_idx(&self) -> Option<usize> {
        match &self.drain {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }

    pub fn s_idx(&self) -> Option<usize> {
        match &self.source {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }
}
