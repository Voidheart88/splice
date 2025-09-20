pub mod bjt;
pub mod capacitor;
pub mod diode;
pub mod inductor;
pub mod isource;
pub mod mosfet;
pub mod pairs;
pub mod resistor;
pub mod triples;
pub mod vsource;

use core::fmt::Display;
use std::sync::Arc;

use crate::spot::*;

pub use self::capacitor::CapacitorBundle;
pub use self::diode::DiodeBundle;
pub use self::inductor::InductorBundle;
pub use self::isource::ISourceBundle;
pub use self::mosfet::Mos0Bundle;
pub use self::pairs::Pairs;
pub use self::resistor::ResistorBundle;
pub use self::triples::{TripleIdx, Triples};
pub use self::vsource::VSourceBundle;

/// An Enum representing the Unit of the Value - Nessecary for
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Unit {
    None,
    Volt,
    Ampere,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Volt => write!(f, "V"),
            Unit::Ampere => write!(f, "A"),
            Unit::None => write!(f, " "),
        }
    }
}

/// A structure representing the name and position of a Variable.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Variable(Arc<str>, Unit, usize);

impl Variable {
    /// Creates a new `Variable` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable.
    /// * `unit` - The unit of the variable.
    /// * `index` - The index of the variable.
    ///
    /// # Returns
    ///
    /// A new `Variable` object.
    pub fn new(name: Arc<str>, unit: Unit, index: usize) -> Self {
        Variable(name, unit, index)
    }

    pub fn name(&self) -> Arc<str> {
        self.0.clone()
    }

    pub fn unit(&self) -> Unit {
        self.1
    }

    pub fn idx(&self) -> usize {
        self.2
    }
}

impl From<(Arc<str>, Unit, usize)> for Variable {
    fn from(value: (Arc<str>, Unit, usize)) -> Self {
        let name = value.0;
        let unit = value.1;
        let idx = value.2;
        Self(name, unit, idx)
    }
}

/// An enum representing different types of circuit elements.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Element {
    Capacitor(CapacitorBundle),
    Inductor(InductorBundle),
    Resistor(ResistorBundle),
    Diode(DiodeBundle),
    Mos0(Mos0Bundle),
    VSource(VSourceBundle),
    ISource(ISourceBundle),
}

impl Element {
    pub(crate) fn get_constant_triples(&self) -> Option<Triples<Numeric, 4>> {
        match self {
            Element::VSource(ele) => Some(ele.triples()),
            Element::Resistor(ele) => Some(ele.triples()),
            _ => None,
        }
    }

    /// Returns the constant pairs of the element, if applicable.
    pub(crate) fn get_constant_pairs(&self) -> Option<Pairs<Numeric, 2>> {
        match self {
            Element::VSource(ele) => Some(ele.pairs()),
            Element::ISource(ele) => Some(ele.pairs()),
            _ => None,
        }
    }

    /// Returns the time variant triples of the element, if applicable.
    pub(crate) fn get_time_variant_triples(&self,delta_t:Option<&Numeric>) -> Option<Triples<Numeric, 4>> {
        match self {
            Element::Capacitor(ele) => Some(ele.triples(delta_t)),
            Element::Inductor(ele) => Some(ele.triples(delta_t)),
            _ => None,
        }
    }

    /// Returns the time variant pairs of the element, if applicable.
    pub(crate) fn get_time_variant_pairs(&self) -> Option<Pairs<Numeric, 2>> {
        None
    }

    /// Returns the nonlinear triples. Nonlinear Triples are Dependend on Vector x
    pub(crate) fn get_nonlinear_triples(&self, x_vec: &[Numeric]) -> Option<Triples<Numeric, 4>> {
        match self {
            Element::Diode(ele) => Some(ele.triples(x_vec)),
            Element::Mos0(ele) => Some(ele.triples(x_vec)),
            _ => None,
        }
    }

    /// Returns the nonlinear pairs of the element, if applicable.
    pub(crate) fn get_nonlinear_pairs(&self, x_vec: &[Numeric]) -> Option<Pairs<Numeric, 2>> {
        match self {
            Element::Diode(ele) => Some(ele.pairs(x_vec)),
            Element::Mos0(ele) => Some(ele.pairs(x_vec)),
            _ => None,
        }
    }

    /// Checks if the element is nonlinear.
    pub(crate) fn is_nonlinear(&self) -> bool {
        matches!(self, Element::Diode(_) | Element::Mos0(_))
    }

    /// Returns the ac triples. Ac Triples are dependend on f
    pub(crate) fn get_ac_triples(&self, freq: Numeric) -> Option<Triples<ComplexNumeric, 4>> {
        match self {
            Element::Diode(_) => None,
            Element::Mos0(_) => None,
            Element::Capacitor(cap) => Some(cap.ac_triples(freq)),
            Element::Inductor(ind) => Some(ind.ac_triples(freq)),
            Element::Resistor(red) => Some(red.ac_triples()),
            Element::VSource(vsource) => Some(vsource.ac_triples()),
            Element::ISource(_) => None,
        }
    }

    /// Returns the ac pairs of the element, if applicable.
    pub(crate) fn get_ac_pairs(&self, _freq: Numeric) -> Option<Pairs<ComplexNumeric, 2>> {
        match self {
            Element::Diode(_) => None,
            Element::Mos0(_) => None,
            Element::Capacitor(_) => None,
            Element::Inductor(_) => None,
            Element::Resistor(_) => None,
            Element::VSource(ele) => Some(ele.ac_pairs()),
            Element::ISource(_) => None,
        }
    }

    /// Returns the name of the element.
    pub(crate) fn name(&self) -> Arc<str> {
        match self {
            Element::Capacitor(ele) => ele.name(),
            Element::Inductor(ele) => ele.name(),
            Element::Resistor(ele) => ele.name(),
            Element::Diode(ele) => ele.name(),
            Element::Mos0(ele) => ele.name(),
            Element::VSource(ele) => ele.name(),
            Element::ISource(ele) => ele.name(),
        }
    }

    /// Returns the name of the element.
    pub(crate) fn get_triple_indices(&self) -> Option<TripleIdx<4>> {
        match self {
            Element::Capacitor(ele) => ele.triple_idx(),
            Element::Inductor(ele) => ele.triple_idx(),
            Element::Resistor(ele) => ele.triple_idx(),
            Element::Diode(ele) => ele.triple_idx(),
            Element::Mos0(ele) => ele.triple_idx(),
            Element::VSource(ele) => ele.triple_idx(),
            Element::ISource(_) => None,
        }
    }

    /// Returns the ac triples. Ac Triples are dependend on f
    pub(crate) fn get_cplx_triple_indices(&self) -> Option<TripleIdx<4>> {
        match self {
            Element::Diode(_) => None,
            Element::Mos0(_) => None,
            Element::Capacitor(cap) => cap.triple_idx(),
            Element::Inductor(ind) => ind.triple_idx(),
            Element::Resistor(res) => res.triple_idx(),
            Element::VSource(vsource) => vsource.triple_idx(),
            Element::ISource(_) => None,
        }
    }
}

#[cfg(test)]
mod tests;
