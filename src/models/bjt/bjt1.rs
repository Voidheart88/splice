//! Extended BJT model (placeholder for future Gummel‑Poon implementation).
//! Currently forwards all calculations to the simple `Bjt0Bundle` implementation,
//! providing a compatible API while allowing later extensions.

use std::sync::Arc;
use super::super::*; // bring common types (`Variable`, `Numeric`, `Triples`, `Pairs`, etc.)
use crate::spot::*; // for UT constant

/// Options for the extended BJT model. At the moment they are identical to `Bjt0Options`
/// but can be extended with additional parameters (Early voltage, base resistance, ...).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Bjt1Options {
    pub is: Numeric,   // Saturation current
    pub n: Numeric,    // Emission coefficient
    pub beta: Numeric, // Forward current gain
    // Placeholder for future parameters
    // pub va: Numeric, // Early voltage
    // pub rb: Numeric, // Base resistance
}

impl Default for Bjt1Options {
    fn default() -> Self {
        Self {
            is: 1e-14,
            n: 1.0,
            beta: 100.0,
        }
    }
}

/// Wrapper around the simple `Bjt0Bundle`.
/// All methods delegate to the inner bundle, guaranteeing identical behaviour.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) struct Bjt1Bundle {
    inner: super::bjt0::Bjt0Bundle,
    // future fields can be added here
}

impl Bjt1Bundle {
    /// Create a new `Bjt1Bundle`. `value` may be `None` to use defaults.
    pub fn new(
        name: Arc<str>,
        base: Option<Variable>,
        collector: Option<Variable>,
        emitter: Option<Variable>,
        value: Option<Bjt1Options>,
    ) -> Bjt1Bundle {
        // Convert Bjt1Options to Bjt0Options (they have the same fields).
        let opts0 = value.map(|v| super::bjt0::Bjt0Options {
            is: v.is,
            n: v.n,
            beta: v.beta,
        });
        let inner = super::bjt0::Bjt0Bundle::new(name, base, collector, emitter, opts0);
        Bjt1Bundle { inner }
    }

    // ---------------------------------------------------------------------
    // Simple forwarding API – matches the signature of `Bjt0Bundle`.
    // ---------------------------------------------------------------------
    pub fn name(&self) -> Arc<str> {
        self.inner.name()
    }
    pub fn b_idx(&self) -> Option<usize> {
        self.inner.b_idx()
    }
    pub fn c_idx(&self) -> Option<usize> {
        self.inner.c_idx()
    }
    pub fn e_idx(&self) -> Option<usize> {
        self.inner.e_idx()
    }
    pub fn triple_idx(&self) -> Option<TripleIdx<12>> {
        self.inner.triple_idx()
    }
    pub fn triples(&self, x_vec: &[Numeric]) -> Triples<Numeric, 16> {
        // Forward to inner implementation – still returns 16 entries as in Bjt0.
        self.inner.triples(x_vec)
    }
    pub fn pairs(&self, x_vec: &[Numeric]) -> Pairs<Numeric, 3> {
        self.inner.pairs(x_vec)
    }
}

// Export the struct for the public module.

