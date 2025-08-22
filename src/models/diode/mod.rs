/// The Diode Module. As every module this module encapsulates exerything regarding a resistor bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub(crate) mod spice;
pub(crate) mod yaml;

use std::sync::Arc;

use num::{One, Zero};

use super::*;
use crate::spot::*;

/// A structure representing a Diode with all their options.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DiodeBundle {
    name: Arc<str>,
    anode: Option<Variable>,
    cathode: Option<Variable>,
    value: DiodeOptions,
}

/// An struct representing possible Diode options.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DiodeOptions {
    is: Numeric,
    n: Numeric,
    rs: Option<Numeric>,
}

impl Default for DiodeOptions {
    fn default() -> Self {
        Self {
            is: 1e-14,
            n: 1.0,
            rs: None,
        }
    }
}

impl DiodeBundle {
    /// Creates a new `DiodeBundle` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the diode bundle.
    /// * `node0` - The name of the first node.
    /// * `node1` - The name of the second node.
    /// * `a` - The triples representing matrix A.
    /// * `b` - The pairs representing vector b.
    /// * `value` - The value of the diode.
    ///
    /// # Returns
    ///
    /// A new `DiodeBundle` object.
    pub fn new(
        name: Arc<str>,
        anode: Option<Variable>,
        cathode: Option<Variable>,
        value: Option<DiodeOptions>,
    ) -> DiodeBundle {
        let value = match value {
            Some(v) => v,
            None => DiodeOptions::default(),
        };

        DiodeBundle {
            name,
            anode,
            cathode,
            value,
        }
    }

    /// Returns the name of the diode bundle.
    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self, x_vec: &Vec<Numeric>) -> Triples<Numeric, 4> {
        let a_voltage = match self.a_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };
        let c_voltage = match self.c_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };
        let v_diode = a_voltage - c_voltage;

        // Conductance of this diode - Schokley equation
        let cond = (self.value.is * (Numeric::exp(v_diode / (self.value.n * UT)) - Numeric::one()))
            / (self.value.n * UT);

        let a_idx = if let Some(idx) = self.a_idx() {
            idx
        } else {
            return Triples::new(&[(self.c_idx().unwrap(), self.c_idx().unwrap(), cond)]);
        };
        let c_idx = if let Some(idx) = self.c_idx() {
            idx
        } else {
            return Triples::new(&[(self.a_idx().unwrap(), self.a_idx().unwrap(), cond)]);
        };

        Triples::new(&[
            (a_idx, a_idx, cond),
            (c_idx, c_idx, cond),
            (a_idx, c_idx, -cond),
            (c_idx, a_idx, -cond),
        ])
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triple_idx(&self) -> Option<TripleIdx<4>> {
        match (self.a_idx(), self.c_idx()) {
            (None, None) => None,
            (None, Some(idx_1)) => Some(TripleIdx::new(&[(idx_1, idx_1)])),
            (Some(idx_0), None) => Some(TripleIdx::new(&[(idx_0, idx_0)])),
            (Some(idx_0), Some(idx_1)) => Some(TripleIdx::new(&[
                (idx_0, idx_0),
                (idx_1, idx_1),
                (idx_0, idx_1),
                (idx_1, idx_0),
            ])),
        }
    }

    /// Returns a reference to the pairs representing vector b.
    pub fn pairs(&self, x_vec: &Vec<Numeric>) -> Pairs<Numeric, 2> {
        let a_voltage = match self.a_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };
        let c_voltage = match self.c_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };
        let v_diode = a_voltage - c_voltage;

        // Conductance of this diode - Schokley equation
        let cond = (self.value.is * (Numeric::exp(v_diode / (self.value.n * UT)) - Numeric::one()))
            / (self.value.n * UT);

        let ca = cond * v_diode
            - self.value.is * (Numeric::exp(v_diode / (self.value.n * UT)) - Numeric::one());
        let cc = -cond * v_diode
            - self.value.is * (Numeric::exp(v_diode / (self.value.n * UT)) - Numeric::one());

        let a_idx = if let Some(idx) = self.a_idx() {
            idx
        } else {
            return Pairs::new(&[(self.c_idx().unwrap(), cc)]);
        };

        let c_idx = if let Some(idx) = self.c_idx() {
            idx
        } else {
            return Pairs::new(&[(self.a_idx().unwrap(), ca)]);
        };

        Pairs::new(&[(a_idx, ca), (c_idx, cc)])
    }

    pub fn a_idx(&self) -> Option<usize> {
        match &self.anode {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }

    pub fn c_idx(&self) -> Option<usize> {
        match &self.cathode {
            Some(v) => Some(v.idx()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
