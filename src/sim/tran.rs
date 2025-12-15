use log::info;

use crate::models::Element;
use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;
use num::Zero;

pub(super) trait TranSimulation<SO: Solver> {
    fn run_tran(&mut self, tstep: &Numeric, tstop: &Numeric) -> Result<Sim, SimulatorError>;
}

impl<SO: Solver> TranSimulation<SO> for Simulator<SO> {
    fn run_tran(&mut self, tstep: &Numeric, tstop: &Numeric) -> Result<Sim, SimulatorError> {
        info!("Run transient analysis");

        let mut t = Numeric::zero();
        let mut tran_results = Vec::new();

        let mut x_prev: Vec<Numeric> = self.find_op()?.iter().map(|op| op.1).collect();
        
        // Initialize capacitor voltages and inductor currents for transient analysis
        // For transient analysis, capacitors typically start with 0V across them
        // (discharged state), and inductors start with 0A through them.
        // The OP analysis treats capacitors as open circuits and inductors as short circuits,
        // which doesn't give us the correct initial conditions for transient simulation.
        for element in &mut self.elements {
            if let Element::Capacitor(cap) = element {
                // Start with 0V across the capacitor (discharged state)
                cap.update_previous_voltage(Numeric::zero());
            } else if let Element::Inductor(ind) = element {
                // Start with 0A through the inductor (no initial current)
                ind.update_previous_current(Numeric::zero());
            }
        }

        while t <= *tstop {
            // Start with the previous solution as initial guess
            let mut x_current = x_prev.clone();

            // Newton-Raphson iteration within each time step
            let mut converged = false;
            for _ in 0..MAXITER {
                self.solver.reset();
                self.build_constant_a_mat();
                self.build_constant_b_vec();
                self.build_time_variant_a_mat(tstep);
                self.build_time_variant_b_vec(&t, tstep);
                self.build_nonlinear_a_mat(&x_current);
                self.build_nonlinear_b_vec(&x_current);

                let x_new = self.solver.solve()?.clone();

                if self.has_converged(&x_current, &x_new, VECTOL) {
                    tran_results.push((t, self.add_var_name(x_new.clone())));
                    x_prev = x_new.clone();
                    
                    // Update capacitor voltages and inductor currents for next time step
                    self.update_capacitor_voltages(&x_new);
                    self.update_inductor_currents(&x_new, tstep);
                    
                    converged = true;
                    break;
                }

                x_current = x_new;
            }

            if !converged {
                return Err(SimulatorError::NonConvergentMaxIter {
                    max_iter: MAXITER,
                    tol: VECTOL,
                });
            }

            t += tstep;
        }

        Ok(Sim::Tran(tran_results))
    }
}
