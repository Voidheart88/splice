use crate::sim::commands::ACMode;
use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;
use log::info;

/// Calculate frequencies for AC analysis based on the specified mode
///
/// This function generates frequency points for AC analysis using different scaling modes:
/// - Linear (LIN): Equal frequency steps
/// - Decade (DEC): Logarithmic steps per decade
/// - Octave (OCT): Logarithmic steps per octave
///
/// # Arguments
///
/// * `fstart` - Starting frequency
/// * `fend` - Ending frequency
/// * `steps` - Number of frequency steps
/// * `mode` - Frequency scaling mode (Linear, Decade, Octave)
///
/// # Returns
///
/// Vector of frequency values for AC analysis
fn calculate_frequencies(fstart: Numeric, fend: Numeric, steps: usize, mode: ACMode) -> Vec<Numeric> {
    match mode {
        ACMode::Lin => {
            let step_size = (fend - fstart) / (steps as Numeric);
            (0..=steps)
                .map(|i| fstart + i as Numeric * step_size)
                .collect()
        }
        ACMode::Dec => {
            let log_fstart = fstart.log10();
            let log_fend = fend.log10();
            let step_size = (log_fend - log_fstart) / (steps as Numeric);
            (0..=steps)
                .map(|i| NUMERIC_TEN.powf(log_fstart + i as Numeric * step_size))
                .collect()
        }
        ACMode::Oct => {
            let oct_fstart = fstart.log2();
            let oct_fend = fend.log2();
            let step_size = (oct_fend - oct_fstart) / (steps as Numeric);
            (0..=steps)
                .map(|i| NUMERIC_TWO.powf(oct_fstart + i as Numeric * step_size))
                .collect()
        }
    }
}

/// Trait for AC (Alternating Current) analysis.
///
/// AC analysis calculates the frequency response of a circuit by performing
/// complex frequency sweeps. This is essential for analyzing circuit behavior
/// in the frequency domain, including gain, phase, impedance, and stability.
pub(super) trait AcSimulation<SO: Solver> {
    /// Runs an AC analysis over a specified frequency range.
    ///
    /// # Arguments
    ///
    /// * `fstart` - Starting frequency in Hz
    /// * `fend` - Ending frequency in Hz
    /// * `steps` - Number of frequency steps
    /// * `ac_option` - Frequency scaling mode (Linear, Decade, Octave)
    ///
    /// # Returns
    ///
    /// * `Ok(Sim::Ac)` - AC simulation results containing frequency response data
    /// * `Err(SimulatorError)` - If the simulation fails
    fn run_ac(
        &mut self,
        fstart: &Numeric,
        fend: &Numeric,
        steps: &usize,
        ac_option: &ACMode,
    ) -> Result<Sim, SimulatorError>;
}

impl<SO: Solver> AcSimulation<SO> for Simulator<SO> {
    fn run_ac(
        &mut self,
        fstart: &Numeric,
        fend: &Numeric,
        steps: &usize,
        ac_option: &ACMode,
    ) -> Result<Sim, SimulatorError> {
        info!("Run ac analysis");
        info!("Find operating point");
        // First find the DC operating point as the starting point for AC analysis
        self.find_op()?;

        // Calculate frequencies in the range from [fstart;fend] using the specified mode
        let freqs = calculate_frequencies(*fstart, *fend, *steps, ac_option.clone());

        info!("Run analysis");

        let mut ac_results = Vec::new();
        
        // Perform AC analysis at each frequency point
        for freq in freqs {
            // Build complex conductance matrix and source vector for current frequency
            self.build_ac_a_mat(freq);
            self.build_ac_b_vec(freq);

            // Solve the complex system of equations
            let x_new = match self.solver.solve_cplx().cloned() {
                Ok(solution) => solution,
                Err(err) => return Err(err.into()),
            };

            // Add variable names to the complex solution
            let x_new = self.add_complex_var_name(x_new);

            // Store frequency and corresponding solution
            ac_results.push((freq, x_new))
        }

        Ok(Sim::Ac(ac_results))
    }
}
