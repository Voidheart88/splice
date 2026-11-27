use log::{debug, info};

use crate::Simulator;
use crate::models::Element;
use crate::sim::SimulatorError;
use crate::sim::newton_armijo::NewtonArmijo;
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

            // Newton-Raphson iteration within each time step with Armijo backtracking
            let armijo_solver = NewtonArmijo::new();
            
            // Get integration method once per timestep
            let integration_method = self.get_integration_method();
            
            x_current = armijo_solver.solve_with_strategy(
                &mut *self,
                x_current,
                VECTOL,
                |sim, x| {
                    // Reset solver and build matrices
                    sim.solver.reset();
                    sim.build_constant_a_mat();
                    sim.build_constant_b_vec();
                    sim.build_time_variant_a_mat(&current_timestep);

                    // Build time-variant B vector based on integration method
                    match integration_method {
                        IntegrationMethodEnum::BackwardEuler => {
                            sim.build_time_variant_b_vec(&t, &current_timestep);
                        }
                        IntegrationMethodEnum::ForwardEuler => {
                            sim.build_time_variant_b_vec(&t, &current_timestep);
                        }
                        IntegrationMethodEnum::Trapezoidal => {
                            sim.build_time_variant_b_vec_trapezoidal(&t, &current_timestep);
                        }
                        IntegrationMethodEnum::GearBdf { order: _ } => {
                            sim.build_time_variant_b_vec(&t, &current_timestep);
                        }
                    }

                    // Build nonlinear matrices at current x
                    sim.build_nonlinear_a_mat(x);
                    sim.build_nonlinear_b_vec(x);
                },
            )?;

            // Store results
            tran_results.push((t, self.add_var_name(x_current.clone())));
            x_prev = x_current.clone();

            // Update capacitor voltages and inductor currents for next time step
            self.update_capacitor_voltages(&x_current);
            self.update_inductor_currents(&x_current, &current_timestep);

            // Adaptive timestep control
            if use_adaptive {
                current_timestep = self.adjust_timestep(&x_prev, &x_current, current_timestep);
                debug!("Adaptive timestep: {} at t = {}", current_timestep, t);
            }

            t += current_timestep;
        }

        Ok(Sim::Tran(tran_results))
    }
}
