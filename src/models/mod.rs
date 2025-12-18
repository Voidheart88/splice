pub mod bjt;
pub mod capacitor;
pub mod controlled_sources;
pub mod coupled_inductors;
pub mod diode;
pub mod gain;
pub mod inductor;
pub mod integration;
pub mod isource;
pub mod mosfet;
pub mod pairs;
pub mod resistor;
pub mod triples;
pub mod vsource;
pub mod vsource_sine;
pub mod vsource_step;

use core::fmt::Display;
use std::collections::HashMap;
use std::sync::Arc;

use num::{One, Zero};

use crate::spot::*;
use serde::Serialize;

pub use self::capacitor::CapacitorBundle;
pub use self::controlled_sources::{CCCSBundle, CCVSBundle, VCCSBundle, VCVSBundle};
pub use self::coupled_inductors::serde::SerdeCoupledInductors;
pub use self::coupled_inductors::CoupledInductorsBundle;
pub use self::diode::DiodeBundle;
pub use self::gain::GainBundle;
pub use self::inductor::InductorBundle;
pub use self::isource::ISourceBundle;
pub use self::mosfet::Mos0Bundle;
pub use self::pairs::Pairs;
pub use self::resistor::ResistorBundle;
pub use self::triples::{TripleIdx, Triples};
pub use self::vsource::VSourceBundle;
pub use self::vsource_sine::VSourceSinBundle;
pub use self::vsource_step::VSourceStepBundle;

/// An Enum representing the Unit of the Value - Necessary for parsing and display.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize)]
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

impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct VariableWrapper {
            name: String,
            unit: Unit,
            idx: usize,
        }
        VariableWrapper {
            name: self.0.to_string(),
            unit: self.1,
            idx: self.2,
        }
        .serialize(serializer)
    }
}

impl Variable {
    /// Creates a new `Variable` object.
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
        Self(value.0, value.1, value.2)
    }
}

/// An enum representing different types of circuit elements.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Element {
    Capacitor(CapacitorBundle),
    Inductor(InductorBundle),
    CoupledInductors(CoupledInductorsBundle),
    Resistor(ResistorBundle),
    Diode(DiodeBundle),
    Mos0(Mos0Bundle),
    VSource(VSourceBundle),
    VSourceSin(VSourceSinBundle),
    VSourceStep(VSourceStepBundle),
    ISource(ISourceBundle),
    Gain(GainBundle),
    VCVS(VCVSBundle),
    VCCS(VCCSBundle),
    CCCS(CCCSBundle),
    CCVS(CCVSBundle),
}

impl Element {
    /// Returns the constant triples of the element, if applicable.
    pub(crate) fn get_constant_triples(&self) -> Option<Triples<Numeric, 4>> {
        match self {
            Element::VSourceSin(ele) => Some(ele.triples()),
            Element::VSource(ele) => Some(ele.triples()),
            Element::VSourceStep(ele) => Some(ele.triples()),
            Element::Resistor(ele) => Some(ele.triples()),
            Element::Gain(ele) => Some(ele.triples()), // Gain ist linear und konstant
            Element::VCVS(ele) => Some(ele.triples()),
            Element::VCCS(ele) => Some(ele.triples()),
            Element::CCCS(ele) => Some(ele.triples()),
            Element::CCVS(ele) => Some(ele.triples()),
            _ => None,
        }
    }

    /// Returns the constant pairs of the element, if applicable.
    pub(crate) fn get_constant_pairs(&self) -> Option<Pairs<Numeric, 2>> {
        match self {
            Element::VSourceSin(ele) => Some(ele.pairs(None)),
            Element::VSource(ele) => Some(ele.pairs()),
            Element::ISource(ele) => Some(ele.pairs()),
            _ => None,
        }
    }

    /// Returns the time variant triples of the element, if applicable.
    pub(crate) fn get_time_variant_triples(
        &self,
        delta_t: &Numeric,
    ) -> Option<Triples<Numeric, 4>> {
        match self {
            Element::Capacitor(ele) => Some(ele.triples(Some(delta_t))),
            Element::Inductor(ele) => Some(ele.triples(Some(delta_t))),
            Element::CoupledInductors(ele) => Some(ele.get_time_variant_triples(delta_t)),
            _ => None,
        }
    }

    /// Returns the time variant pairs of the element, if applicable.
    pub(crate) fn get_time_variant_pairs(
        &self,
        time: Option<&Numeric>,
        delta_t: &Numeric,
    ) -> Option<Pairs<Numeric, 2>> {
        match self {
            Element::VSourceSin(ele) => Some(ele.pairs(time)),
            Element::VSourceStep(ele) => Some(ele.pairs(time)),
            Element::Capacitor(ele) => Some(ele.pairs(delta_t)),
            Element::Inductor(ele) => Some(ele.pairs(delta_t)),
            Element::CoupledInductors(ele) => Some(ele.get_pairs()),
            Element::VCVS(_) => None,
            Element::VCCS(_) => None,
            Element::CCCS(_) => None,
            Element::CCVS(_) => None,
            _ => None,
        }
    }

    /// Returns the time variant pairs of the element using trapezoidal integration, if applicable.
    pub(crate) fn get_time_variant_pairs_trapezoidal(
        &self,
        time: Option<&Numeric>,
        delta_t: &Numeric,
    ) -> Option<Pairs<Numeric, 2>> {
        match self {
            Element::VSourceSin(ele) => Some(ele.pairs(time)), // Time sources use same method
            Element::VSourceStep(ele) => Some(ele.pairs(time)), // Time sources use same method
            Element::Capacitor(ele) => Some(ele.pairs_trapezoidal(delta_t)),
            Element::Inductor(ele) => Some(ele.pairs_trapezoidal(delta_t)),
            Element::CoupledInductors(ele) => Some(ele.get_pairs()), // Same as Euler for now
            Element::VCVS(_) => None,
            Element::VCCS(_) => None,
            Element::CCCS(_) => None,
            Element::CCVS(_) => None,
            _ => None,
        }
    }

    /// Returns the nonlinear triples. Nonlinear Triples are dependent on Vector x.
    pub(crate) fn get_nonlinear_triples(&self, x_vec: &[Numeric]) -> Option<Triples<Numeric, 4>> {
        match self {
            Element::Diode(ele) => Some(ele.triples(x_vec)),
            Element::Mos0(ele) => Some(ele.triples(x_vec)),
            Element::VCVS(_) => None,
            Element::VCCS(_) => None,
            Element::CCCS(_) => None,
            Element::CCVS(_) => None,
            _ => None,
        }
    }

    /// Returns the nonlinear pairs of the element, if applicable.
    pub(crate) fn get_nonlinear_pairs(&self, x_vec: &[Numeric]) -> Option<Pairs<Numeric, 2>> {
        match self {
            Element::Diode(ele) => Some(ele.pairs(x_vec)),
            Element::Mos0(ele) => Some(ele.pairs(x_vec)),
            Element::VCVS(_) => None,
            Element::VCCS(_) => None,
            Element::CCCS(_) => None,
            Element::CCVS(_) => None,
            _ => None,
        }
    }

    /// Checks if the element is nonlinear.
    pub(crate) fn is_nonlinear(&self) -> bool {
        matches!(
            self,
            Element::Diode(_) | Element::Mos0(_) | Element::CoupledInductors(_)
        )
    }

    /// Returns the AC triples. AC Triples are dependent on frequency f.
    pub(crate) fn get_ac_triples(&self, freq: Numeric) -> Option<Triples<ComplexNumeric, 4>> {
        match self {
            Element::Diode(_) => None,
            Element::Mos0(_) => None,
            Element::Capacitor(cap) => Some(cap.ac_triples(freq)),
            Element::Inductor(ind) => Some(ind.ac_triples(freq)),
            Element::CoupledInductors(coupled) => Some(coupled.get_ac_triples(freq)),
            Element::Resistor(res) => Some(res.ac_triples()),
            Element::VSource(vsource) => Some(vsource.ac_triples()),
            Element::VSourceStep(_) => None,
            Element::Gain(gain) => Some(gain.ac_triples()),
            Element::VCVS(vcvs) => Some(vcvs.ac_triples()),
            Element::VCCS(vccs) => Some(vccs.ac_triples()),
            Element::CCCS(cccs) => Some(cccs.ac_triples()),
            Element::CCVS(ccvs) => Some(ccvs.ac_triples()),
            Element::ISource(_) => None,
            Element::VSourceSin(_) => None,
        }
    }

    /// Returns the AC pairs of the element, if applicable.
    pub(crate) fn get_ac_pairs(&self, _freq: Numeric) -> Option<Pairs<ComplexNumeric, 2>> {
        match self {
            Element::Diode(_) => None,
            Element::Mos0(_) => None,
            Element::Capacitor(_) => None,
            Element::Inductor(_) => None,
            Element::Resistor(_) => None,
            Element::VSource(ele) => Some(ele.ac_pairs()),
            Element::VSourceStep(ele) => Some(ele.ac_pairs()),
            Element::CoupledInductors(_) => None,
            Element::VCVS(_) => None,
            Element::VCCS(_) => None,
            Element::CCCS(_) => None,
            Element::CCVS(_) => None,
            Element::ISource(_) => None,
            Element::Gain(_) => None,
            Element::VSourceSin(_) => None,
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
            Element::VSourceStep(ele) => ele.name(),
            Element::ISource(ele) => ele.name(),
            Element::Gain(ele) => ele.name(),
            Element::VCVS(ele) => ele.name(),
            Element::VCCS(ele) => ele.name(),
            Element::CCCS(ele) => ele.name(),
            Element::CCVS(ele) => ele.name(),
            Element::CoupledInductors(ele) => ele.name(),
            Element::VSourceSin(ele) => ele.name(),
        }
    }

    /// Returns the indices of the triples for the element.
    pub(crate) fn get_triple_indices(&self) -> Option<TripleIdx<4>> {
        match self {
            Element::Capacitor(ele) => ele.triple_idx(),
            Element::Inductor(ele) => ele.triple_idx(),
            Element::CoupledInductors(ele) => ele.get_triple_indices(),
            Element::Resistor(ele) => ele.triple_idx(),
            Element::Diode(ele) => ele.triple_idx(),
            Element::Mos0(ele) => ele.triple_idx(),
            Element::VSource(ele) => ele.triple_idx(),
            Element::VSourceStep(ele) => ele.triple_idx(),
            Element::Gain(ele) => ele.triple_idx(),
            Element::VCVS(ele) => ele.triple_idx(),
            Element::VCCS(ele) => ele.triple_idx(),
            Element::CCCS(ele) => ele.triple_idx(),
            Element::CCVS(ele) => ele.triple_idx(),
            Element::ISource(_) => None,
            Element::VSourceSin(ele) => ele.triple_idx(),
        }
    }
    /// Returns the indices of the complex triples for the element.
    pub(crate) fn get_cplx_triple_indices(&self) -> Option<TripleIdx<4>> {
        match self {
            Element::Diode(_) => None,
            Element::Mos0(_) => None,
            Element::Capacitor(cap) => cap.triple_idx(),
            Element::Inductor(ind) => ind.triple_idx(),
            Element::CoupledInductors(coupled) => coupled.get_cplx_triple_indices(),
            Element::Resistor(res) => res.triple_idx(),
            Element::VSource(vsource) => vsource.triple_idx(),
            Element::VSourceStep(vsource) => vsource.triple_idx(),
            Element::Gain(ele) => ele.triple_idx(),
            Element::VCVS(ele) => ele.triple_idx(),
            Element::VCCS(ele) => ele.triple_idx(),
            Element::CCCS(ele) => ele.triple_idx(),
            Element::CCVS(ele) => ele.triple_idx(),
            Element::ISource(_) => None,
            Element::VSourceSin(ele) => ele.triple_idx(),
        }
    }

    /// Setup coupled inductors by setting their node indices
    /// This should be called after all elements are parsed and before simulation
    /// Returns a list of validation errors, if any
    pub fn setup_coupled_inductors(elements: &mut [Element]) -> Vec<String> {
        let mut errors = Vec::new();

        // First, collect all inductor names and their node indices
        let mut inductor_map: HashMap<Arc<str>, (Option<usize>, Option<usize>)> = HashMap::new();

        for element in elements.iter() {
            if let Element::Inductor(ind) = element {
                inductor_map.insert(ind.name.clone(), (ind.node0_idx(), ind.node1_idx()));
            }
        }

        // Then, set up the coupled inductors with the node indices
        // Fixme: This nests way too deep and needs a refactor
        for element in elements.iter_mut() {
            if let Element::CoupledInductors(coupled) = element {
                // Validate coupling factor
                let coupling_factor = coupled.coupling_factor();
                if coupling_factor <= Numeric::zero() || coupling_factor >= Numeric::one() {
                    errors.push(format!(
                        "Coupled inductors '{}': Invalid coupling factor {}. Must be 0 < k < 1",
                        coupled.name(),
                        coupling_factor
                    ));
                }

                // Check if referenced inductors exist
                let inductor1_name = coupled.inductor1();
                let inductor2_name = coupled.inductor2();

                if !inductor_map.contains_key(&inductor1_name) {
                    errors.push(format!(
                        "Coupled inductors '{}': Referenced inductor '{}' not found",
                        coupled.name(),
                        inductor1_name
                    ));
                }

                if !inductor_map.contains_key(&inductor2_name) {
                    errors.push(format!(
                        "Coupled inductors '{}': Referenced inductor '{}' not found",
                        coupled.name(),
                        inductor2_name
                    ));
                }

                // Check if inductors are distinct
                if inductor1_name == inductor2_name {
                    errors.push(format!(
                        "Coupled inductors '{}': Inductor '{}' cannot be coupled to itself",
                        coupled.name(),
                        inductor1_name
                    ));
                }

                // Only set node indices if both inductors exist and are valid
                if inductor_map.contains_key(&inductor1_name)
                    && inductor_map.contains_key(&inductor2_name)
                {
                    if let Some((node0_idx1, node1_idx1)) = inductor_map.get(&inductor1_name) {
                        if let Some((node0_idx2, node1_idx2)) = inductor_map.get(&inductor2_name) {
                            // Validate that inductors have proper node connections
                            if node0_idx1.is_none() && node1_idx1.is_none() {
                                errors.push(format!(
                                    "Coupled inductors '{}': Inductor '{}' has no node connections",
                                    coupled.name(),
                                    inductor1_name
                                ));
                            }

                            if node0_idx2.is_none() && node1_idx2.is_none() {
                                errors.push(format!(
                                    "Coupled inductors '{}': Inductor '{}' has no node connections",
                                    coupled.name(),
                                    inductor2_name
                                ));
                            }

                            // Set node indices only if both inductors have valid connections
                            if (node0_idx1.is_some() || node1_idx1.is_some())
                                && (node0_idx2.is_some() || node1_idx2.is_some())
                            {
                                coupled.set_node_indices(
                                    *node0_idx1,
                                    *node1_idx1,
                                    *node0_idx2,
                                    *node1_idx2,
                                );
                            }
                        }
                    }
                }
            }
        }

        errors
    }
}
#[cfg(test)]
mod tests;
