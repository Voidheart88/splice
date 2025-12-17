// Coupled inductors model for SPICE simulation
// Implements mutual inductance between two inductors

use std::sync::Arc;
use crate::spot::{Numeric, ComplexNumeric};
use crate::models::triples::Triples;
use crate::models::pairs::Pairs;
use crate::models::triples::TripleIdx;
use num::{Complex, One, Zero};
use num::traits::FloatConst;

pub mod serde;

/// Options for coupled inductors
#[derive(Debug, Clone)]
pub struct CoupledInductorsOptions {
    #[allow(dead_code)]
    coupling_factor: Numeric, // Coupling factor k (0 < k < 1)
}

impl Default for CoupledInductorsOptions {
    fn default() -> Self {
        Self {
            coupling_factor: 0.999, // Default high coupling
        }
    }
}

/// Bundle containing two inductors and their mutual coupling
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CoupledInductorsBundle {
    name: Arc<str>,
    inductor1: Arc<str>, // Name of first inductor
    inductor2: Arc<str>, // Name of second inductor
    coupling_factor: Numeric, // Coupling factor k
    mutual_inductance: Numeric, // Calculated mutual inductance M = k * sqrt(L1*L2)
    // Node indices for the coupled inductors (will be set during simulation setup)
    node0_idx1: Option<usize>,
    node1_idx1: Option<usize>,
    node0_idx2: Option<usize>,
    node1_idx2: Option<usize>,
    // Previous currents for transient simulation (will be updated during simulation)
    previous_current1: Numeric,
    previous_current2: Numeric,
}

impl CoupledInductorsBundle {
    /// Create a new coupled inductors bundle
    pub fn new(
        name: Arc<str>,
        inductor1: Arc<str>,
        inductor2: Arc<str>,
        coupling_factor: Numeric,
    ) -> Self {
        Self {
            name,
            inductor1,
            inductor2,
            coupling_factor,
            mutual_inductance: 0.0, // Will be calculated when inductors are known
            node0_idx1: None,
            node1_idx1: None,
            node0_idx2: None,
            node1_idx2: None,
            previous_current1: Numeric::zero(),
            previous_current2: Numeric::zero(),
        }
    }

    pub fn name(&self) -> Arc<str> {
        self.name.clone()
    }

    pub fn inductor1(&self) -> Arc<str> {
        self.inductor1.clone()
    }

    pub fn inductor2(&self) -> Arc<str> {
        self.inductor2.clone()
    }

    pub fn coupling_factor(&self) -> Numeric {
        self.coupling_factor
    }

    /// Calculate mutual inductance from coupling factor and inductor values
    pub fn calculate_mutual_inductance(&mut self, l1: Numeric, l2: Numeric) {
        // Use the coupling factor from options if available, otherwise use the stored one
        let k = self.coupling_factor;
        self.mutual_inductance = k * (l1 * l2).sqrt();
    }

    pub fn mutual_inductance(&self) -> Numeric {
        self.mutual_inductance
    }

    /// Set the node indices for the coupled inductors
    /// This should be called during simulation setup after all elements are parsed
    pub fn set_node_indices(
        &mut self,
        node0_idx1: Option<usize>,
        node1_idx1: Option<usize>,
        node0_idx2: Option<usize>,
        node1_idx2: Option<usize>,
    ) {
        self.node0_idx1 = node0_idx1;
        self.node1_idx1 = node1_idx1;
        self.node0_idx2 = node0_idx2;
        self.node1_idx2 = node1_idx2;
    }

    /// Update the previous currents for transient simulation
    /// This should be called after each time step
    pub fn update_previous_currents(&mut self, current1: Numeric, current2: Numeric) {
        self.previous_current1 = current1;
        self.previous_current2 = current2;
    }

    /// Get the previous currents
    pub fn previous_currents(&self) -> (Numeric, Numeric) {
        (self.previous_current1, self.previous_current2)
    }

    /// Get the triple indices for MNA matrix contributions
    /// For coupled inductors, we need to contribute to both inductor equations
    pub fn get_triple_indices(&self) -> Option<TripleIdx<4>> {
        // If we don't have node indices yet, return None
        // This can happen if setup was not called or if there were validation errors
        if self.node0_idx1.is_none() || self.node1_idx1.is_none() || 
           self.node0_idx2.is_none() || self.node1_idx2.is_none() {
            return None;
        }

        let node0_idx1 = self.node0_idx1.unwrap();
        let node1_idx1 = self.node1_idx1.unwrap();
        let node0_idx2 = self.node0_idx2.unwrap();
        let node1_idx2 = self.node1_idx2.unwrap();

        // Validate that we have at least some valid node connections
        // At least one node per inductor should be connected
        if (node0_idx1 == usize::MAX || node1_idx1 == usize::MAX) ||
           (node0_idx2 == usize::MAX || node1_idx2 == usize::MAX) {
            return None;
        }

        // The mutual inductance affects all combinations of nodes from both inductors
        // For simplicity, we'll use the first 4 combinations that fit in TripleIdx<4>
        // In a production implementation, this would need to be handled differently
        Some(TripleIdx::new(&[
            (node0_idx1, node0_idx2),
            (node0_idx1, node1_idx2),
            (node1_idx1, node0_idx2),
            (node1_idx1, node1_idx2),
        ]))
    }

    /// Get time-variant triples for transient simulation
    /// For coupled inductors, the mutual inductance M contributes to both inductor equations:
    /// v1 = L1 * di1/dt + M * di2/dt
    /// v2 = L2 * di2/dt + M * di1/dt
    /// Using backward Euler: di/dt ≈ (i_current - i_prev) / delta_t
    pub fn get_time_variant_triples(&self, delta_t: &Numeric) -> Triples<Numeric, 4> {
        // If we don't have node indices yet, return empty triples
        // This can happen if setup was not called or if there were validation errors
        if self.node0_idx1.is_none() || self.node1_idx1.is_none() || 
           self.node0_idx2.is_none() || self.node1_idx2.is_none() {
            return Triples::new(&[]);
        }

        let m = self.mutual_inductance;
        
        // Check for reasonable mutual inductance value
        if m <= Numeric::zero() {
            return Triples::new(&[]);
        }

        let equivalent_conductance = delta_t / m;

        let node0_idx1 = self.node0_idx1.unwrap();
        let node1_idx1 = self.node1_idx1.unwrap();
        let node0_idx2 = self.node0_idx2.unwrap();
        let node1_idx2 = self.node1_idx2.unwrap();

        // The mutual inductance contributes to both inductor equations
        // For inductor 1: M * di2/dt term
        // For inductor 2: M * di1/dt term
        // This creates cross-coupling between the two inductor branches
        
        Triples::new(&[
            // Contribution to inductor 1 equation from inductor 2 current
            (node0_idx1, node0_idx2, equivalent_conductance),
            (node0_idx1, node1_idx2, -equivalent_conductance),
            (node1_idx1, node0_idx2, -equivalent_conductance),
            (node1_idx1, node1_idx2, equivalent_conductance),
        ])
    }

    /// Get pairs for current contributions
    /// For coupled inductors, we need to account for the previous currents
    /// in the transient simulation using backward Euler integration:
    /// i1 = (M/Δt) * (v1 - v2) + i1_prev + (M/Δt) * (v1_prev - v2_prev)
    /// i2 = (M/Δt) * (v2 - v1) + i2_prev + (M/Δt) * (v2_prev - v1_prev)
    pub fn get_pairs(&self) -> Pairs<Numeric, 2> {
        // If we don't have node indices yet, return empty pairs
        // This can happen if setup was not called or if there were validation errors
        if self.node0_idx1.is_none() || self.node1_idx1.is_none() || 
           self.node0_idx2.is_none() || self.node1_idx2.is_none() {
            return Pairs::new(&[]);
        }

        let m = self.mutual_inductance;
        
        // Check for reasonable mutual inductance value
        if m <= Numeric::zero() {
            return Pairs::new(&[]);
        }

        let _node0_idx1 = self.node0_idx1.unwrap();
        let _node1_idx1 = self.node1_idx1.unwrap();
        let _node0_idx2 = self.node0_idx2.unwrap();
        let _node1_idx2 = self.node1_idx2.unwrap();

        // For transient simulation, the current contributions depend on the previous currents
        // and the mutual inductance. However, we need access to the previous voltages and
        // the time step to calculate the correct contributions.
        // Since we don't have access to delta_t and previous voltages here, we return empty pairs
        // and the actual current contributions will be handled in the simulation engine.
        
        // Note: In a complete implementation, this would require:
        // 1. Access to the previous voltages (v1_prev, v2_prev)
        // 2. Access to the current time step (delta_t)
        // 3. The actual calculation would be:
        //    i1_contribution = (m/delta_t) * (v1_prev - v2_prev) + previous_current1
        //    i2_contribution = (m/delta_t) * (v2_prev - v1_prev) + previous_current2
        
        // For now, we return empty pairs. The simulation engine should handle
        // the current contributions based on the stored previous currents.
        Pairs::new(&[])
    }

    /// Get AC triples for AC analysis
    /// For AC analysis, mutual inductance contributes as jωM
    /// The impedance of mutual inductance is Z = jωM
    pub fn get_ac_triples(&self, freq: Numeric) -> Triples<ComplexNumeric, 4> {
        // If we don't have node indices yet, return empty triples
        // This can happen if setup was not called or if there were validation errors
        if self.node0_idx1.is_none() || self.node1_idx1.is_none() || 
           self.node0_idx2.is_none() || self.node1_idx2.is_none() {
            return Triples::new(&[]);
        }

        let m = self.mutual_inductance;
        
        // Check for reasonable mutual inductance value
        if m <= Numeric::zero() {
            return Triples::new(&[]);
        }

        // Check for reasonable frequency
        if freq < Numeric::zero() {
            return Triples::new(&[]);
        }

        let omega = (Numeric::one() + Numeric::one()) * Numeric::PI() * freq;
        let _j_omega_m = Complex {
            re: Numeric::zero(),
            im: omega * m,
        };
        let equivalent_admittance = Complex {
            re: Numeric::zero(),
            im: -omega * m, // 1/(jωM) = -j/(ωM)
        };

        let node0_idx1 = self.node0_idx1.unwrap();
        let node1_idx1 = self.node1_idx1.unwrap();
        let node0_idx2 = self.node0_idx2.unwrap();
        let node1_idx2 = self.node1_idx2.unwrap();

        // For AC analysis, the mutual inductance creates complex cross-coupling
        // We'll use the first 4 combinations to fit in Triples<ComplexNumeric, 4>
        Triples::new(&[
            // Contribution to inductor 1 equation from inductor 2
            (node0_idx1, node0_idx2, equivalent_admittance),
            (node0_idx1, node1_idx2, -equivalent_admittance),
            (node1_idx1, node0_idx2, -equivalent_admittance),
            (node1_idx1, node1_idx2, equivalent_admittance),
        ])
    }

    /// Get complex triple indices for AC analysis
    pub fn get_cplx_triple_indices(&self) -> Option<TripleIdx<4>> {
        // Complex triple indices are the same as real triple indices for coupled inductors
        self.get_triple_indices()
    }

    /// Check if the element is nonlinear (coupled inductors are linear)
    pub fn is_nonlinear(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Element, Variable};
    use std::sync::Arc;

    fn create_test_inductor(name: &str, node0: Option<usize>, node1: Option<usize>, value: Numeric) -> Element {
        Element::Inductor(crate::models::InductorBundle {
            name: Arc::from(name),
            node0: node0.map(|idx| Variable::new(Arc::from(format!("node{}", idx)), crate::models::Unit::Volt, idx)),
            node1: node1.map(|idx| Variable::new(Arc::from(format!("node{}", idx)), crate::models::Unit::Volt, idx)),
            value,
            previous_current: Numeric::zero(),
        })
    }

    fn create_test_coupled(name: &str, inductor1: &str, inductor2: &str, coupling: Numeric) -> Element {
        Element::CoupledInductors(CoupledInductorsBundle {
            name: Arc::from(name),
            inductor1: Arc::from(inductor1),
            inductor2: Arc::from(inductor2),
            coupling_factor: coupling,
            mutual_inductance: 0.0,
            node0_idx1: None,
            node1_idx1: None,
            node0_idx2: None,
            node1_idx2: None,
            previous_current1: Numeric::zero(),
            previous_current2: Numeric::zero(),
        })
    }

    #[test]
    fn test_new_coupled_inductors() {
        let coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        assert_eq!(coupled.name(), Arc::from("K1"));
        assert_eq!(coupled.inductor1(), Arc::from("L1"));
        assert_eq!(coupled.inductor2(), Arc::from("L2"));
        assert_eq!(coupled.coupling_factor(), 0.999);
        assert_eq!(coupled.mutual_inductance(), 0.0);
    }

    #[test]
    fn test_calculate_mutual_inductance() {
        let mut coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        coupled.calculate_mutual_inductance(10e-6, 15e-6); // L1=10µH, L2=15µH
        
        let product: Numeric = 10e-6 * 15e-6;
        let expected_m = 0.999 * product.sqrt();
        assert!((coupled.mutual_inductance() - expected_m).abs() < 1e-12);
    }

    #[test]
    fn test_set_node_indices() {
        let mut coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        coupled.set_node_indices(Some(0), Some(1), Some(2), Some(3));
        
        assert_eq!(coupled.node0_idx1, Some(0));
        assert_eq!(coupled.node1_idx1, Some(1));
        assert_eq!(coupled.node0_idx2, Some(2));
        assert_eq!(coupled.node1_idx2, Some(3));
    }

    #[test]
    fn test_update_previous_currents() {
        let mut coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        coupled.update_previous_currents(0.001, 0.002);
        
        let (current1, current2) = coupled.previous_currents();
        assert!((current1 - 0.001).abs() < 1e-12);
        assert!((current2 - 0.002).abs() < 1e-12);
    }

    #[test]
    fn test_get_triple_indices_no_indices() {
        let coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        assert!(coupled.get_triple_indices().is_none());
    }

    #[test]
    fn test_get_triple_indices_with_indices() {
        let mut coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        coupled.set_node_indices(Some(0), Some(1), Some(2), Some(3));
        
        let indices = coupled.get_triple_indices();
        assert!(indices.is_some());
        let idx = indices.unwrap();
        assert_eq!(idx.data().len(), 4); // 4 combinations of nodes (limited by TripleIdx<4>)
    }

    #[test]
    fn test_get_time_variant_triples_no_indices() {
        let coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        let triples = coupled.get_time_variant_triples(&1e-6);
        assert_eq!(triples.len(), 0);
    }

    #[test]
    fn test_get_ac_triples_no_indices() {
        let coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        let triples = coupled.get_ac_triples(1000.0);
        assert_eq!(triples.len(), 0);
    }

    #[test]
    fn test_setup_coupled_inductors_validation() {
        // Test invalid coupling factor
        let mut elements = vec![
            create_test_inductor("L1", Some(0), Some(1), 10e-6),
            create_test_inductor("L2", Some(2), Some(3), 15e-6),
            create_test_coupled("K1", "L1", "L2", 1.2), // Invalid: > 1.0
        ];
        
        let errors = Element::setup_coupled_inductors(&mut elements);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Invalid coupling factor"));
    }

    #[test]
    fn test_setup_coupled_inductors_missing_inductors() {
        let mut elements = vec![
            create_test_inductor("L1", Some(0), Some(1), 10e-6),
            create_test_coupled("K1", "L1", "L3", 0.999), // L3 doesn't exist
        ];
        
        let errors = Element::setup_coupled_inductors(&mut elements);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("not found"));
    }

    #[test]
    fn test_setup_coupled_inductors_self_coupling() {
        let mut elements = vec![
            create_test_inductor("L1", Some(0), Some(1), 10e-6),
            create_test_coupled("K1", "L1", "L1", 0.999), // Self-coupling
        ];
        
        let errors = Element::setup_coupled_inductors(&mut elements);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("cannot be coupled to itself"));
    }

    #[test]
    fn test_setup_coupled_inductors_successful() {
        let mut elements = vec![
            create_test_inductor("L1", Some(0), Some(1), 10e-6),
            create_test_inductor("L2", Some(2), Some(3), 15e-6),
            create_test_coupled("K1", "L1", "L2", 0.999),
        ];
        
        let errors = Element::setup_coupled_inductors(&mut elements);
        assert_eq!(errors.len(), 0);
        
        // Check that node indices were set
        if let Element::CoupledInductors(coupled) = &elements[2] {
            assert_eq!(coupled.node0_idx1, Some(0));
            assert_eq!(coupled.node1_idx1, Some(1));
            assert_eq!(coupled.node0_idx2, Some(2));
            assert_eq!(coupled.node1_idx2, Some(3));
        } else {
            panic!("Expected coupled inductors element");
        }
    }

    #[test]
    fn test_coupled_inductors_mna_contributions() {
        let mut coupled = CoupledInductorsBundle::new(
            Arc::from("K1"),
            Arc::from("L1"),
            Arc::from("L2"),
            0.999,
        );
        
        // Set up mutual inductance
        coupled.calculate_mutual_inductance(10e-6, 15e-6);
        coupled.set_node_indices(Some(0), Some(1), Some(2), Some(3));
        
        // Test transient triples
        let transient_triples = coupled.get_time_variant_triples(&1e-6);
        assert_eq!(transient_triples.len(), 4); // Should have 4 entries (limited by Triples<Numeric, 4>)
        
        // Test AC triples
        let ac_triples = coupled.get_ac_triples(1000.0);
        assert_eq!(ac_triples.len(), 4); // Should have 4 entries (limited by Triples<ComplexNumeric, 4>)
        
        // Test triple indices
        let indices = coupled.get_triple_indices();
        assert!(indices.is_some());
        assert_eq!(indices.unwrap().len(), 4); // Should have 4 entries (limited by TripleIdx<4>)
    }
}

