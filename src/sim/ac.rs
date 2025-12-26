use crate::sim::commands::ACMode;
use crate::sim::simulation_result::Sim;
use crate::sim::SimulatorError;
use crate::solver::Solver;
use crate::spot::*;
use crate::Simulator;
use log::info;

pub(super) trait AcSimulation<SO: Solver> {
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
        self.find_op()?;

        // Calculate frequencies in the range from [fstart;fend]
        // TODO: Consider refactoring to reduce nesting complexity
        let freqs: Vec<Numeric> = match ac_option {
            ACMode::Lin => {
                let step_size = (fend - fstart) / (*steps as Numeric);
                (0..=*steps)
                    .map(|i| fstart + i as Numeric * step_size)
                    .collect()
            }
            ACMode::Dec => {
                let log_fstart = fstart.log10();
                let log_fend = fend.log10();
                let step_size = (log_fend - log_fstart) / (*steps as Numeric);
                (0..=*steps)
                    .map(|i| NUMERIC_TEN.powf(log_fstart + i as Numeric * step_size))
                    .collect()
            }
            ACMode::Oct => {
                let oct_fstart = fstart.log2();
                let oct_fend = fend.log2();
                let step_size = (oct_fend - oct_fstart) / (*steps as Numeric);
                (0..=*steps)
                    .map(|i| NUMERIC_TWO.powf(oct_fstart + i as Numeric * step_size))
                    .collect()
            }
        };

        info!("Run analysis");

        let mut ac_results = Vec::new();
        for freq in freqs {
            self.build_ac_a_mat(freq);
            self.build_ac_b_vec(freq);

            let x_new = match self.solver.solve_cplx().cloned() {
                Ok(solution) => solution,
                Err(err) => return Err(err.into()),
            };

            let x_new = self.add_complex_var_name(x_new);

            ac_results.push((freq, x_new))
        }

        Ok(Sim::Ac(ac_results))
    }
}
