use std::sync::Arc;

use crate::consts::UT;

use super::*;

/// A structure representing a bundle of diodes.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct DiodeBundle {
    name: Arc<String>,
    anode: Option<Variable>,
    cathode: Option<Variable>,
    value: DiodeOptions,
}

/// An enum representing possible Diode options.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct DiodeOptions {
    is: f64,
    n: f64,
    rs: Option<f64>,
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
        name: Arc<String>,
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
    pub fn name(&self) -> Arc<String> {
        self.name.clone()
    }

    /// Returns a reference to the triples representing matrix A.
    pub fn triples(&self, x_vec: &Vec<f64>) -> Triples {
        let a_voltage = match self.a_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let c_voltage = match self.c_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let v_diode = a_voltage - c_voltage;

        // Conductance of this diode - Schokley equation
        let cond =
            (self.value.is * (f64::exp(v_diode / (self.value.n * UT)) - 1.0)) / (self.value.n * UT);

        let a_idx = if let Some(idx) = self.a_idx() {
            idx
        } else {
            return Triples::Single((self.c_idx().unwrap(), self.c_idx().unwrap(), cond));
        };
        let c_idx = if let Some(idx) = self.c_idx() {
            idx
        } else {
            return Triples::Single((self.a_idx().unwrap(), self.a_idx().unwrap(), cond));
        };

        Triples::Quad([
            (a_idx, a_idx, cond),
            (c_idx, c_idx, cond),
            (a_idx, c_idx, -cond),
            (c_idx, a_idx, -cond),
        ])
    }

    /// Returns a reference to the pairs representing vector b.
    pub fn pairs(&self, x_vec: &Vec<f64>) -> Pairs {
        let a_voltage = match self.a_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let c_voltage = match self.c_idx() {
            Some(idx) => x_vec[idx],
            None => 0.0,
        };
        let v_diode = a_voltage - c_voltage;

        // Conductance of this diode - Schokley equation
        let cond =
            (self.value.is * (f64::exp(v_diode / (self.value.n * UT)) - 1.0)) / (self.value.n * UT);

        let ca = cond * v_diode - self.value.is * (f64::exp(v_diode / (self.value.n * UT)) - 1.0);
        let cc = -cond * v_diode - self.value.is * (f64::exp(v_diode / (self.value.n * UT)) - 1.0);

        let a_idx = if let Some(idx) = self.a_idx() {
            idx
        } else {
            return Pairs::Single((self.c_idx().unwrap(), cc));
        };

        let c_idx = if let Some(idx) = self.c_idx() {
            idx
        } else {
            return Pairs::Single((self.a_idx().unwrap(), ca));
        };

        Pairs::Double([(a_idx, ca), (c_idx, cc)])
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
