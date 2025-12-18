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
        let value = value.unwrap_or_default();

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
    /// Implements the Ebers-Moll model for BJT
    pub fn triples(&self, x_vec: &[Numeric]) -> Triples<Numeric, 16> {
        use num::{One, Zero};

        // Get voltages at each terminal
        let v_base = match self.b_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };

        let v_collector = match self.c_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };

        let v_emitter = match self.e_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };

        // Calculate junction voltages
        let v_be = v_base - v_emitter; // Base-Emitter voltage
        let v_bc = v_base - v_collector; // Base-Collector voltage

        // Ebers-Moll model parameters
        let is = self.value.is; // Saturation current
        let n = self.value.n; // Emission coefficient
        let beta = self.value.beta; // Current gain

        // Calculate diode conductances for BE and BC junctions
        let g_be = if v_be > -100.0 {
            // Avoid numerical overflow
            (is * (Numeric::exp(v_be / (n * UT)) - Numeric::one())) / (n * UT)
        } else {
            Numeric::zero()
        };

        let g_bc = if v_bc > -100.0 {
            // Avoid numerical overflow
            (is * (Numeric::exp(v_bc / (n * UT)) - Numeric::one())) / (n * UT)
        } else {
            Numeric::zero()
        };

        // Calculate transistor action components
        let g_m = if v_be > -100.0 {
            (is * Numeric::exp(v_be / (n * UT))) / (n * UT) * beta
        } else {
            Numeric::zero()
        };

        // Get indices for each terminal
        let b_idx = self.b_idx();
        let c_idx = self.c_idx();
        let e_idx = self.e_idx();

        // Build the conductance matrix based on which terminals are connected
        let mut triples = Vec::new();

        match (b_idx, c_idx, e_idx) {
            // All three terminals connected
            (Some(b), Some(c), Some(e)) => {
                // Base-Emitter diode conductance
                triples.push((b, b, g_be));
                triples.push((e, e, g_be));
                triples.push((b, e, -g_be));
                triples.push((e, b, -g_be));

                // Base-Collector diode conductance
                triples.push((b, b, g_bc));
                triples.push((c, c, g_bc));
                triples.push((b, c, -g_bc));
                triples.push((c, b, -g_bc));

                // Transistor action (current gain)
                triples.push((c, b, g_m));
                triples.push((b, c, -g_m));

                // Collector-Emitter path
                triples.push((c, e, -g_m));
                triples.push((e, c, g_m));
            }

            // Base and Collector connected, Emitter grounded
            (Some(b), Some(c), None) => {
                triples.push((b, b, g_be + g_bc));
                triples.push((c, c, g_bc));
                triples.push((b, c, -g_bc));
                triples.push((c, b, -g_bc - g_m));
            }

            // Base and Emitter connected, Collector grounded
            (Some(b), None, Some(e)) => {
                triples.push((b, b, g_be));
                triples.push((e, e, g_be));
                triples.push((b, e, -g_be));
                triples.push((e, b, -g_be));
            }

            // Collector and Emitter connected, Base grounded
            (None, Some(c), Some(e)) => {
                // This configuration doesn't make much sense for BJT
                // but we handle it for completeness
                triples.push((c, c, Numeric::zero()));
                triples.push((e, e, Numeric::zero()));
            }

            // Only one terminal connected (not useful, but handle gracefully)
            (Some(b), None, None) => {
                triples.push((b, b, Numeric::zero()));
            }
            (None, Some(c), None) => {
                triples.push((c, c, Numeric::zero()));
            }
            (None, None, Some(e)) => {
                triples.push((e, e, Numeric::zero()));
            }

            // No terminals connected
            (None, None, None) => {}
        }

        Triples::new(&triples)
    }

    /// Returns a reference to the pairs representing vector b.
    /// Implements the Ebers-Moll model for BJT
    pub fn pairs(&self, x_vec: &[Numeric]) -> Pairs<Numeric, 3> {
        use num::{One, Zero};

        // Get voltages at each terminal
        let v_base = match self.b_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };

        let v_collector = match self.c_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };

        let v_emitter = match self.e_idx() {
            Some(idx) => x_vec[idx],
            None => Numeric::zero(),
        };

        // Calculate junction voltages
        let v_be = v_base - v_emitter; // Base-Emitter voltage
        let v_bc = v_base - v_collector; // Base-Collector voltage

        // Ebers-Moll model parameters
        let is = self.value.is; // Saturation current
        let n = self.value.n; // Emission coefficient
        let beta = self.value.beta; // Current gain

        // Calculate diode currents for BE and BC junctions
        let i_be = if v_be > -100.0 {
            // Avoid numerical overflow
            is * (Numeric::exp(v_be / (n * UT)) - Numeric::one())
        } else {
            Numeric::zero()
        };

        let i_bc = if v_bc > -100.0 {
            // Avoid numerical overflow
            is * (Numeric::exp(v_bc / (n * UT)) - Numeric::one())
        } else {
            Numeric::zero()
        };

        // Calculate transistor action current
        let i_c = if v_be > -100.0 {
            is * Numeric::exp(v_be / (n * UT)) * beta
        } else {
            Numeric::zero()
        };

        // Get indices for each terminal
        let b_idx = self.b_idx();
        let c_idx = self.c_idx();
        let e_idx = self.e_idx();

        // Build the current vector based on which terminals are connected
        let mut pairs = Vec::new();

        match (b_idx, c_idx, e_idx) {
            // All three terminals connected
            (Some(b), Some(c), Some(e)) => {
                let collector_current = beta * i_be - i_bc; // This flows OUT of collector
                let base_current = i_be + i_bc - collector_current / beta; // This flows INTO base
                let emitter_current = i_be + collector_current / beta; // This flows INTO emitter

                // In MNA, currents flowing INTO nodes are positive
                pairs.push((b, base_current)); // Positive: flows into base
                pairs.push((c, -collector_current)); // Negative: flows out of collector
                pairs.push((e, emitter_current)); // Positive: flows into emitter
            }

            // Base and Collector connected, Emitter grounded
            (Some(b), Some(c), None) => {
                let i_b = i_be + i_bc - i_c / beta;
                let i_c = -i_bc - i_c;
                pairs.push((b, i_b));
                pairs.push((c, i_c));
            }

            // Base and Emitter connected, Collector grounded
            (Some(b), None, Some(e)) => {
                let i_b = i_be;
                let i_e = -i_be;
                pairs.push((b, i_b));
                pairs.push((e, i_e));
            }

            // Collector and Emitter connected, Base grounded
            (None, Some(c), Some(e)) => {
                // This configuration doesn't make much sense for BJT
                pairs.push((c, Numeric::zero()));
                pairs.push((e, Numeric::zero()));
            }

            // Only one terminal connected
            (Some(b), None, None) => {
                pairs.push((b, Numeric::zero()));
            }
            (None, Some(c), None) => {
                pairs.push((c, Numeric::zero()));
            }
            (None, None, Some(e)) => {
                pairs.push((e, Numeric::zero()));
            }

            // No terminals connected
            (None, None, None) => {}
        }

        Pairs::new(&pairs)
    }

    pub fn b_idx(&self) -> Option<usize> {
        self.base.as_ref().map(|v| v.idx())
    }

    pub fn c_idx(&self) -> Option<usize> {
        self.collector.as_ref().map(|v| v.idx())
    }

    pub fn e_idx(&self) -> Option<usize> {
        self.emitter.as_ref().map(|v| v.idx())
    }

    /// Returns the indices for the triples representing matrix A.
    /// Used for building the Jacobian matrix.
    pub fn triple_idx(&self) -> Option<TripleIdx<12>> {
        let b_idx = self.b_idx();
        let c_idx = self.c_idx();
        let e_idx = self.e_idx();

        match (b_idx, c_idx, e_idx) {
            // All three terminals connected - full BJT model
            (Some(b), Some(c), Some(e)) => Some(TripleIdx::new(&[
                (b, b),
                (e, e),
                (b, e),
                (e, b), // BE diode
                (b, b),
                (c, c),
                (b, c),
                (c, b), // BC diode
                (c, b),
                (b, c), // Transistor action
                (c, e),
                (e, c), // CE path
            ])),

            // Base and Collector connected, Emitter grounded
            (Some(b), Some(c), None) => Some(TripleIdx::new(&[(b, b), (c, c), (b, c), (c, b)])),

            // Base and Emitter connected, Collector grounded
            (Some(b), None, Some(e)) => Some(TripleIdx::new(&[(b, b), (e, e), (b, e), (e, b)])),

            // Collector and Emitter connected, Base grounded
            (None, Some(c), Some(e)) => Some(TripleIdx::new(&[(c, c), (e, e)])),

            // Only one terminal connected
            (Some(b), None, None) => Some(TripleIdx::new(&[(b, b)])),
            (None, Some(c), None) => Some(TripleIdx::new(&[(c, c)])),
            (None, None, Some(e)) => Some(TripleIdx::new(&[(e, e)])),

            // No terminals connected
            (None, None, None) => None,
        }
    }
}
