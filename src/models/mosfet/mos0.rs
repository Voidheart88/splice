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
    pub fn triples(&self, x_vec: &Vec<f64>) -> Triples {
        let kp= self.options.kp;
        let vt0 = self.options.vt0;
        let g_voltage = match self.g_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let d_voltage = match self.d_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let s_voltage = match self.s_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let v_gs = g_voltage - s_voltage;
        let v_gs_eff = v_gs-vt0;

        let g0 = kp*v_gs_eff*v_gs_eff;

        match (self.d_idx(),self.s_idx()) {
            (None, None) => Triples::Empty,
            (None, Some(s_idx)) => Triples::Single((s_idx,s_idx,g0)),
            (Some(d_idx), None) => Triples::Single((d_idx,d_idx,g0)),
            (Some(d_idx), Some(s_idx)) => Triples::Quad([
                (d_idx,d_idx,g0),
                (s_idx,s_idx,g0),
                (d_idx,s_idx,-g0),
                (s_idx,d_idx,-g0),
            ]),
        }
    }

    /// Returns a reference to the pairs representing vector b.
    pub fn pairs(&self, x_vec: &Vec<f64>) -> Pairs {
        let kp= self.options.kp;
        let vt0 = self.options.vt0;
        let g_voltage = match self.g_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let d_voltage = match self.d_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let s_voltage = match self.s_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let v_gs = g_voltage - s_voltage;
        let v_gs_eff = v_gs-vt0;
        let i_ds = kp*v_gs_eff;

        match (self.d_idx(),self.s_idx()) {
            (None, None) => Pairs::Empty,
            (None, Some(s_idx)) => Pairs::Single((s_idx,-i_ds)),
            (Some(d_idx), None) => Pairs::Single((d_idx,i_ds)),
            (Some(d_idx), Some(s_idx)) => Pairs::Double([(d_idx,i_ds),(s_idx,-i_ds)]),
        }
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
