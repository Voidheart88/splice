use log::{debug, info};

use crate::Simulator;
use crate::models::Element;
use crate::sim::SimulatorError;
use crate::sim::options::IntegrationMethodEnum;
use crate::sim::simulation_result::Sim;
use crate::solver::Solver;
use crate::spot::*;
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
        self.elements.iter_mut().for_each(|element| match element {
            Element::Capacitor(cap) => cap.update_previous_voltage(Numeric::zero()),
            Element::Inductor(ind) => ind.update_previous_current(Numeric::zero()),
            _ => {}
        });

        let mut x_current = x_prev.clone();

        // Store the initial condition (t=0)
        tran_results.push((Numeric::zero(), self.add_var_name(x_current.clone())));

        // Use adaptive timestep if the provided tstep is very small (indication for adaptive mode)
        let use_adaptive = *tstep <= ADAPTIVE_INITIAL_TIMESTEP;
        let mut current_timestep = if use_adaptive {
            ADAPTIVE_INITIAL_TIMESTEP
        } else {
            *tstep
        };

        // Transient simulation time loop
        while t < *tstop {
            // For subsequent time steps, use the previous solution as initial guess
            x_current = x_prev.clone();

            // Newton-Raphson iteration within each time step
            // Uses standard Newton with optional Armijo damping for difficult cases
            let mut converged = false;
            let mut x_new_final = x_prev.clone();
            let mut difficulty_count = 0; // Track how many iterations are having trouble

            for iter in 0..MAXITER {
                self.solver.reset();
                self.build_constant_a_mat();
                self.build_constant_b_vec();
                self.build_time_variant_a_mat(&current_timestep);

                // Use integration method
                let integration_method = self.get_integration_method();
                match integration_method {
                    IntegrationMethodEnum::BackwardEuler => {
                        self.build_time_variant_b_vec(&t, &current_timestep);
                    }
                    IntegrationMethodEnum::ForwardEuler => {
                        self.build_time_variant_b_vec(&t, &current_timestep);
                    }
                    IntegrationMethodEnum::Trapezoidal => {
                        self.build_time_variant_b_vec_trapezoidal(&t, &current_timestep);
                    }
                    IntegrationMethodEnum::GearBdf { order: _ } => {
                        self.build_time_variant_b_vec(&t, &current_timestep);
                    }
                }

                self.build_nonlinear_a_mat(&x_current);
                self.build_nonlinear_b_vec(&x_current);

                let x_new = self.solver.solve()?.clone();

                // Check for standard convergence
                if self.has_converged(&x_current, &x_new, VECTOL) {
                    x_new_final = x_new.clone();
                    converged = true;
                    break;
                }

                // Track difficulty - if we're not converging after a few iterations, use damping
                difficulty_count += 1;

                if difficulty_count > 3 {
                    // Apply Armijo damping for difficult cases
                    let delta_x: Vec<Numeric> = x_new
                        .iter()
                        .zip(x_current.iter())
                        .map(|(&new, &old)| new - old)
                        .collect();

                    let mut alpha = 0.5; // Start with half step
                    let rho = 0.7;
                    let mut damped_converged = false;

                    for _ in 0..8 {
                        let x_damped: Vec<Numeric> = x_current
                            .iter()
                            .zip(delta_x.iter())
                            .map(|(&xi, &dxi)| xi + alpha * dxi)
                            .collect();

                        if self.has_converged(&x_current, &x_damped, VECTOL) {
                            x_new_final = x_damped;
                            converged = true;
                            damped_converged = true;
                            difficulty_count = 0; // Reset difficulty counter
                            break;
                        }

                        alpha *= rho;

                        if alpha < 1e-4 {
                            break;
                        }
                    }

                    if damped_converged {
                        break;
                    }
                }

                x_current = x_new;

                if iter >= MAXITER - 1 {
                    break;
                }
            }

            if !converged {
                return Err(SimulatorError::NonConvergentMaxIter {
                    max_iter: MAXITER,
                    tol: VECTOL,
                });
            }

            // Store results
            tran_results.push((t, self.add_var_name(x_new_final.clone())));
            x_prev = x_new_final.clone();

            // Update capacitor voltages and inductor currents for next time step
            self.update_capacitor_voltages(&x_new_final);
            self.update_inductor_currents(&x_new_final, &current_timestep);

            // Adaptive timestep control
            if use_adaptive {
                current_timestep = self.adjust_timestep(&x_prev, &x_new_final, current_timestep);
                debug!("Adaptive timestep: {} at t = {}", current_timestep, t);
            }

            t += current_timestep;
        }

        Ok(Sim::Tran(tran_results))
    }
}
