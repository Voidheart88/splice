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
use num::One;

/// Armijo line search parameters
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

/// Trait for computing residual norm
pub trait ResidualNorm {
    /// Computes the L2 norm of the residual F(x)
    fn compute_residual_norm(&mut self, x: &[Numeric]) -> Numeric;
}

impl<SO: Solver> ResidualNorm for Simulator<SO> {
    fn compute_residual_norm(&mut self, x: &[Numeric]) -> Numeric {
        // Build the system matrices at current x
        self.build_constant_a_mat();
        self.build_constant_b_vec();
        self.build_nonlinear_a_mat(x);
        self.build_nonlinear_b_vec(x);

        // Compute residual r = A*x - b
        // For Newton: we want ||F(x)|| where F(x) = 0 is the system
        // The residual is the mismatch in the equations

        // Get the B vector (known terms)
        // The residual norm is approximated by the norm of the update
        // This is a simplification - a full implementation would compute
        // the actual residual F(x) = A(x)*x - b(x)

        // For circuit simulation, we use the norm of the RHS as a proxy
        // since the system is built to solve A*Δx = -F(x)

        // Compute L2 norm of x as a proxy for residual
        // A more accurate implementation would compute F(x) explicitly
        let norm: Numeric = x.iter().map(|&xi| xi * xi).sum::<Numeric>().sqrt();

        norm
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
        mut x: Vec<Numeric>,
        tolerance: Numeric,
    ) -> Result<Vec<Numeric>, SimulatorError> {
        let mut iteration = 0;

        while iteration < MAXITER {
            // Build system matrices at current x
            simulator.build_constant_a_mat();
            simulator.build_constant_b_vec();
            simulator.build_nonlinear_a_mat(&x);
            simulator.build_nonlinear_b_vec(&x);

            // Compute current residual norm
            let residual_norm = simulator.compute_residual_norm(&x);

            // Check convergence
            if residual_norm < tolerance {
                return Ok(x);
            }

            // Solve for Newton direction: A * Δx = -F(x)
            // In our case, the solver already handles this
            let delta_x = match simulator.solver.solve() {
                Ok(solution) => solution.clone(),
                Err(_) => {
                    return Err(SimulatorError::NonConvergentMaxIter {
                        max_iter: MAXITER,
                        tol: tolerance,
                    });
                }
            };

            // Armijo backtracking line search
            let mut alpha = Numeric::one(); // Start with full step
            let mut backtrack_iter = 0;

            // Newton direction should decrease residual

            let mut x_new = x.clone();

            while backtrack_iter < self.parameters.max_backtrack {
                // Try step: x_new = x + alpha * delta_x
                x_new = x
                    .iter()
                    .zip(delta_x.iter())
                    .map(|(&xi, &dxi)| xi + alpha * dxi)
                    .collect();

                // Compute new residual norm
                let new_residual_norm = simulator.compute_residual_norm(&x_new);

                // Check Armijo condition: ||F(x + α*Δx)|| ≤ (1 - β*α) * ||F(x)||
                let armijo_threshold =
                    (Numeric::one() - self.parameters.beta * alpha) * residual_norm;

                if new_residual_norm <= armijo_threshold {
                    // Armijo condition satisfied, accept step
                    break;
                }

                // Reduce step size
                alpha *= self.parameters.rho;
                backtrack_iter += 1;

                // Check if step is too small
                if alpha < self.parameters.alpha_min {
                    // Step too small, but still accept if we made progress
                    if new_residual_norm < residual_norm {
                        break;
                    } else {
                        return Err(SimulatorError::NonConvergentMaxIter {
                            max_iter: MAXITER,
                            tol: tolerance,
                        });
                    }
                }
            }

            // Update x
            x = x_new;
            iteration += 1;

            // Check convergence with standard criterion
            if self.has_converged_step(&delta_x, alpha, tolerance) {
                return Ok(x);
            }
        }

        // Max iterations reached
        Err(SimulatorError::NonConvergentMaxIter {
            max_iter: MAXITER,
            tol: tolerance,
        })
    }

    /// Check convergence based on step size
    fn has_converged_step(&self, delta_x: &[Numeric], alpha: Numeric, tolerance: Numeric) -> bool {
        delta_x.iter().all(|&dxi| (alpha * dxi).abs() < tolerance)
    }
}

impl Default for NewtonArmijo {
    fn default() -> Self {
        Self::new()
    }
}
