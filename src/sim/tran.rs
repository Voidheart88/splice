use log::{debug, info};

use crate::models::Element;
use crate::sim::options::IntegrationMethod;
use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;
use num::Zero;

pub(super) trait TranSimulation<SO: Solver> {
    fn run_tran(&mut self, tstep: &Numeric, tstop: &Numeric) -> Result<Sim, SimulatorError>;
}

impl<SO: Solver> Simulator<SO> {
    /// Perform Newton-Raphson iteration for a single time step
    fn perform_newton_raphson_iteration(
        &mut self,
        x_initial: &[Numeric],
        t: &Numeric,
        timestep: &Numeric,
        max_iter: usize,
        tol: Numeric,
    ) -> Result<Vec<Numeric>, SimulatorError> {
        let mut x_current = x_initial.to_vec();

        for _ in 0..max_iter {
            self.solver.reset();
            self.build_constant_a_mat();
            self.build_constant_b_vec();
            self.build_time_variant_a_mat(timestep);

            // Use trapezoidal integration if specified
            let integration_method = self.get_integration_method();
            match integration_method {
                IntegrationMethod::BackwardEuler => {
                    self.build_time_variant_b_vec(t, timestep);
                }
                IntegrationMethod::Trapezoidal => {
                    self.build_time_variant_b_vec_trapezoidal(t, timestep);
                }
            }

            self.build_nonlinear_a_mat(&x_current);
            self.build_nonlinear_b_vec(&x_current);

            let x_new = self.solver.solve()?.clone();

            if self.has_converged(&x_current, &x_new, tol) {
                return Ok(x_new);
            }

            x_current = x_new;
        }

        Err(SimulatorError::NonConvergentMaxIter {
            max_iter,
            tol,
        })
    }
}

impl<SO: Solver> TranSimulation<SO> for Simulator<SO> {
    fn run_tran(&mut self, tstep: &Numeric, tstop: &Numeric) -> Result<Sim, SimulatorError> {
        info!("Run transient analysis");

        let mut t = Numeric::zero();
        let mut tran_results = Vec::new();

        let mut x_prev: Vec<Numeric> = self.find_op()?.iter().map(|op| op.1).collect();

        // Initialize capacitor voltages and inductor currents for transient analysis
        for element in &mut self.elements {
            if let Element::Capacitor(cap) = element {
                cap.update_previous_voltage(Numeric::zero());
            } else if let Element::Inductor(ind) = element {
                ind.update_previous_current(Numeric::zero());
            }
        }

        let x_current = x_prev.clone();

        // Store the initial condition (t=0)
        tran_results.push((Numeric::zero(), self.add_var_name(x_current.clone())));

        // Use adaptive timestep if the provided tstep is very small (indication for adaptive mode)
        let use_adaptive = *tstep <= ADAPTIVE_INITIAL_TIMESTEP;
        let mut current_timestep = if use_adaptive {
            ADAPTIVE_INITIAL_TIMESTEP
        } else {
            *tstep
        };

        // Main transient simulation loop
        while t < *tstop {
            // Perform Newton-Raphson iteration for current time step
            let x_new_final = self.perform_newton_raphson_iteration(
                &x_prev,
                &t,
                &current_timestep,
                MAXITER,
                VECTOL,
            )?;

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
