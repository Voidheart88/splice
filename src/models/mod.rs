mod capacitor;
mod diode;
mod pairs;
mod inductor;
mod isource;
mod resistor;
mod triples;
mod vsource;

use core::fmt::Display;
use std::sync::Arc;

use crate::backends::{Col, Row};

use derive_more::{Deref, DerefMut};

pub(crate) use self::capacitor::CapacitorBundle;
pub(crate) use self::diode::DiodeBundle;
pub(crate) use self::pairs::Pairs; //Fixme: Find a better word for this
pub(crate) use self::inductor::InductorBundle;
pub(crate) use self::isource::ISourceBundle;
pub(crate) use self::resistor::ResistorBundle;
pub(crate) use self::triples::Triples;
pub(crate) use self::vsource::VSourceBundle;

/// An Enum representing the Unit of the Value - Nessecary for
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Unit {
    Volt,
    Ampere,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Volt => write!(f, "V"),
            Unit::Ampere => write!(f, "A"),
        }
    }
}

/// A structure representing the name of an element.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) struct Variable(Arc<String>, Unit, usize);

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
    pub fn new(name: Arc<String>, unit: Unit, index: usize) -> Self {
        Variable(name, unit, index)
    }

    pub fn name(&self) -> Arc<String> {
        return self.0.clone();
    }

    pub fn unit(&self) -> Unit {
        return self.1;
    }

    pub fn idx(&self) -> usize {
        self.2
    }
}

impl From<(Arc<String>, Unit, usize)> for Variable {
    fn from(value: (Arc<String>, Unit, usize)) -> Self {
        let name = value.0;
        let unit = value.1;
        let idx = value.2;
        Self(name, unit, idx)
    }
}

/// A structure representing the value of an element.
#[derive(DerefMut, Deref, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub(crate) struct Value(pub f64);
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

/// An enum representing different types of circuit elements.
#[derive(Debug)]
pub enum Element {
    Capacitor(CapacitorBundle),
    Inductor(InductorBundle),
    Resistor(ResistorBundle),
    Diode(DiodeBundle),
    VSource(VSourceBundle),
    ISource(ISourceBundle),
}

impl Element {
    #[allow(unused)]
    /// Returns the constant triples of the element, if applicable.
    pub(crate) fn get_triples(&self, x_vec: &Vec<f64>) -> Option<Triples> {
        match self {
            Element::VSource(ele) => Some(ele.triples()),
            Element::Resistor(ele) => Some(ele.triples()),
            Element::Capacitor(ele) => Some(ele.triples()),
            Element::Inductor(ele) => Some(ele.triples()),
            Element::Diode(ele) => Some(ele.triples(x_vec)),
            Element::ISource(_) => None,
        }
    }

    #[allow(unused)]
    /// Returns the time variant pairs of the element, if applicable.
    pub(crate) fn get_pairs(&self, x_vec: &Vec<f64>) -> Option<Pairs> {
        match self {
            Element::Diode(ele) => Some(ele.pairs(x_vec)),
            Element::VSource(ele) => Some(ele.pairs()),
            Element::ISource(ele) => Some(ele.pairs()),
            _ => None,
        }
    }

    pub(crate) fn get_constant_triples(&self) -> Option<Triples> {
        match self {
            Element::VSource(ele) => Some(ele.triples()),
            Element::Resistor(ele) => Some(ele.triples()),
            _ => None,
        }
    }

    /// Returns the constant pairs of the element, if applicable.
    pub(crate) fn get_constant_pairs(&self) -> Option<Pairs> {
        match self {
            Element::VSource(ele) => Some(ele.pairs()),
            Element::ISource(ele) => Some(ele.pairs()),
            _ => None,
        }
    }

    /// Returns the time variant triples of the element, if applicable.
    pub(crate) fn get_time_variant_triples(&self) -> Option<Triples> {
        match self {
            Element::Capacitor(ele) => Some(ele.triples()),
            Element::Inductor(ele) => Some(ele.triples()),
            _ => None,
        }
    }

    /// Returns the time variant pairs of the element, if applicable.
    pub(crate) fn get_time_variant_pairs(&self) -> Option<Pairs> {
        match self {
            _ => None,
        }
    }

    /// Returns the nonlinear triples. Nonlinear Triples are Dependend on Vector x
    pub(crate) fn get_nonlinear_triples(&self, x_vec: &Vec<f64>) -> Option<Triples> {
        match self {
            Element::Diode(ele) => Some(ele.triples(x_vec)),
            _ => None,
        }
    }

    /// Returns the nonlinear pairs of the element, if applicable.
    pub(crate) fn get_nonlinear_pairs(&self, x_vec: &Vec<f64>) -> Option<Pairs> {
        match self {
            Element::Diode(ele) => Some(ele.pairs(x_vec)),
            _ => None,
        }
    }

    /// Checks if the element is nonlinear.
    pub(crate) fn is_nonlinear(&self) -> bool {
        match self {
            Element::Diode(_) => true,
            _ => false,
        }
    }

    /// Returns the name of the element.
    pub(crate) fn name(&self) -> Arc<String> {
        match self {
            Element::Capacitor(ele) => ele.name(),
            Element::Inductor(ele) => ele.name(),
            Element::Resistor(ele) => ele.name(),
            Element::Diode(ele) => ele.name(),
            Element::VSource(ele) => ele.name(),
            Element::ISource(ele) => ele.name(),
        }
    }
}

#[cfg(test)]
mod tests;
