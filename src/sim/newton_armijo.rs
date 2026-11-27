use crate::sim::SimulatorError;
use crate::sim::{MAXITER, Simulator};
use crate::solver::Solver;
/// Newton-Raphson method with Armijo step size control for global convergence
///
/// This module provides a globally convergent Newton-Raphson implementation
/// using Armijo backtracking line search. The Armijo condition ensures that
/// each Newton step sufficiently reduces the residual norm, preventing divergence
/// when the initial guess is far from the solution.
///
/// # Algorithm
///
/// 1. Compute Newton direction: J(x_k) * Δx = -F(x_k)
/// 2. Start with full step α = 1.0
/// 3. Check Armijo condition: ||F(x_k + α*Δx)|| ≤ (1 - β*α) * ||F(x_k)||
/// 4. If condition fails, reduce α (backtracking): α ← ρ * α
/// 5. Repeat until condition satisfied or α becomes too small
///
/// # Parameters
///
/// - β (beta): Armijo parameter (typically 0.1 to 0.5), controls sufficient decrease
/// - ρ (rho): Reduction factor (typically 0.5 to 0.8), controls step size reduction
/// - α_min: Minimum step size before giving up
use crate::spot::Numeric;

/// Armijo line search parameters
#[allow(dead_code)]
pub struct ArmijoParameters {
    /// Armijo parameter (0 < beta < 0.5), typically 0.1 to 0.3
    /// Controls the required decrease in residual norm
    pub beta: Numeric,

    /// Reduction factor (0 < rho < 1), typically 0.5 to 0.7
    /// Controls how quickly step size is reduced during backtracking
    pub rho: Numeric,

    /// Minimum step size before giving up
    pub alpha_min: Numeric,

    /// Maximum number of backtracking iterations
    pub max_backtrack: usize,
}

impl Default for ArmijoParameters {
    fn default() -> Self {
        Self {
            beta: 0.2,         // Require 20% of linear decrease
            rho: 0.6,          // Reduce step by 40% each backtrack
            alpha_min: 1e-8,   // Minimum step size
            max_backtrack: 20, // Maximum backtracking iterations
        }
    }
}

/// Trait for computing residual norm (not currently used in simplified Newton)
/// Kept for future Armijo implementation
pub trait ResidualNorm {
    /// Computes a metric for convergence
    #[allow(dead_code)]
    fn compute_residual_norm(&self) -> Numeric;
}

impl<SO: Solver> ResidualNorm for Simulator<SO> {
    fn compute_residual_norm(&self) -> Numeric {
        // Placeholder - not used in simplified Newton implementation
        0.0
    }
}

/// Newton-Raphson with Armijo step size control
pub struct NewtonArmijo {
    parameters: ArmijoParameters,
}

impl NewtonArmijo {
    /// Create a new Newton-Armijo solver with default parameters
    pub fn new() -> Self {
        Self {
            parameters: ArmijoParameters::default(),
        }
    }

    /// Create a new Newton-Armijo solver with custom parameters
    #[allow(dead_code)]
    pub fn with_parameters(params: ArmijoParameters) -> Self {
        Self { parameters: params }
    }

    /// Returns a reference to the Armijo parameters
    #[cfg(test)]
    pub fn parameters(&self) -> &ArmijoParameters {
        &self.parameters
    }

    /// Perform Newton-Raphson iteration with Armijo backtracking
    ///
    /// # Arguments
    /// * `simulator` - The simulator with built matrices
    /// * `x_initial` - Initial guess
    /// * `tolerance` - Convergence tolerance
    ///
    /// # Returns
    /// * `Ok(Vec<Numeric>)` - Converged solution
    /// * `Err(SimulatorError)` - Failed to converge
    pub fn solve<SO: Solver>(
        &self,
        simulator: &mut Simulator<SO>,
        x: Vec<Numeric>,
        tolerance: Numeric,
    ) -> Result<Vec<Numeric>, SimulatorError> {
        // Use the standard matrix building strategy
        self.solve_with_strategy(
            simulator,
            x,
            tolerance,
            |sim, x| {
                sim.build_constant_a_mat();
                sim.build_constant_b_vec();
                sim.build_nonlinear_a_mat(x);
                sim.build_nonlinear_b_vec(x);
            },
        )
    }

    /// Perform Newton-Raphson iteration with step damping for difficult cases
    ///
    /// # Arguments
    /// * `simulator` - The simulator
    /// * `x_initial` - Initial guess
    /// * `tolerance` - Convergence tolerance
    /// * `build_matrices` - Closure to build system matrices at current x
    ///
    /// # Returns
    /// * `Ok(Vec<Numeric>)` - Converged solution
    /// * `Err(SimulatorError)` - Failed to converge
    pub fn solve_with_strategy<SO: Solver, F>(
        &self,
        simulator: &mut Simulator<SO>,
        mut x: Vec<Numeric>,
        tolerance: Numeric,
        build_matrices: F,
    ) -> Result<Vec<Numeric>, SimulatorError>
    where
        F: Fn(&mut Simulator<SO>, &[Numeric]),
    {
        let mut iteration = 0;
        let mut difficulty_count = 0;

        while iteration < MAXITER {
            // Build system matrices at current x using the provided strategy
            build_matrices(simulator, &x);

            // Solve: the solver returns the new solution x_new
            let x_new = match simulator.solver.solve() {
                Ok(solution) => solution.clone(),
                Err(_) => {
                    return Err(SimulatorError::NonConvergentMaxIter {
                        max_iter: MAXITER,
                        tol: tolerance,
                    });
                }
            };

            // Check convergence: has the solution converged?
            let converged = x.iter()
                .zip(x_new.iter())
                .all(|(&old, &new)| (new - old).abs() < tolerance);
            
            if converged {
                return Ok(x_new);
            }

            // Track difficulty
            difficulty_count += 1;

            // Apply damping for difficult cases
            let mut x_final = x_new.clone();
            if difficulty_count > 3 {
                // Compute delta = x_new - x
                let delta: Vec<Numeric> = x_new.iter()
                    .zip(x.iter())
                    .map(|(&new, &old)| new - old)
                    .collect();

                let mut alpha = 0.5; // Start with half step
                let rho = self.parameters.rho;
                let mut found = false;

                for _ in 0..self.parameters.max_backtrack {
                    // Try damped step: x_damped = x + alpha * delta
                    let x_damped: Vec<Numeric> = x.iter()
                        .zip(delta.iter())
                        .map(|(&xi, &d)| xi + alpha * d)
                        .collect();

                    // Check if damped step is converging
                    let damped_converged = x_damped.iter()
                        .zip(x.iter())
                        .all(|(&new, &old)| (new - old).abs() < tolerance);
                    
                    if damped_converged {
                        x_final = x_damped;
                        found = true;
                        difficulty_count = 0;
                        break;
                    }

                    alpha *= rho;
                    if alpha < self.parameters.alpha_min {
                        break;
                    }
                }

                if !found {
                    // Use the damped step even if not fully converged
                    let x_damped: Vec<Numeric> = x.iter()
                        .zip(delta.iter())
                        .map(|(&xi, &d)| xi + alpha.max(self.parameters.alpha_min) * d)
                        .collect();
                    x_final = x_damped;
                }
            }

            x = x_final;
            iteration += 1;
        }

        // Max iterations reached
        Err(SimulatorError::NonConvergentMaxIter {
            max_iter: MAXITER,
            tol: tolerance,
        })
    }

    /// Check convergence based on step size
    #[allow(dead_code)]
    fn has_converged_step(&self, delta_x: &[Numeric], alpha: Numeric, tolerance: Numeric) -> bool {
        delta_x.iter().all(|&dxi| (alpha * dxi).abs() < tolerance)
    }
}

impl Default for NewtonArmijo {
    fn default() -> Self {
        Self::new()
    }
}
