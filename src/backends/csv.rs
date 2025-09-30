use std::collections::HashSet;
use std::sync::Arc;

use super::Backend;
use crate::models::Variable;
use crate::sim::options::SimulationOption;
use crate::sim::simulation_result::Sim;
use crate::spot::*;
use crate::{sim::simulation_result::SimulationResults, BackendError};

/// A backend for outputting simulation results as CSV.
pub struct CsvBackend {}

impl Backend for CsvBackend {
    /// Outputs the simulation results in CSV format.
    ///
    /// # Arguments
    ///
    /// * `results` - A collection of simulation results to output.
    ///
    /// # Errors
    ///
    /// Returns a `BackendError` if there is a problem with the output.
    fn output(&self, results: SimulationResults) -> Result<(), BackendError> {
        let options = results.options;
        for res in results.results.iter() {
            match res {
                Sim::Op(res) => Self::output_op(res),
                Sim::Dc(res) => Self::output_dc(res, options.clone()),
                Sim::Ac(res) => Self::output_ac(res),
                Sim::Tran(res) => Self::output_tran(res),
            }
        }
        Ok(())
    }
}

impl CsvBackend {
    /// Creates a new `CsvBackend` instance.
    ///
    /// # Returns
    ///
    /// A new instance of `CsvBackend`.
    pub fn new() -> Self {
        CsvBackend {}
    }

    /// Outputs operating point simulation results in CSV format.
    ///
    /// # Arguments
    ///
    /// * `results` - A vector of tuples containing variables and their corresponding values.
    fn output_op(results: &Vec<(Variable, Numeric)>) {
        for res in results {
            println!("{},{},{}", res.0.name(), res.1, res.0.unit())
        }
    }

    /// Outputs DC sweep simulation results in CSV format.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of vectors, where each inner vector contains tuples of variables and their values for each step.
    /// * `options` - A vector of Simulation options.
    fn output_dc(data: &Vec<Vec<(Variable, Numeric)>>, options: Vec<SimulationOption>) {
        // Collect the variable names specified in the options.
        let mut filtered_headers = HashSet::new();
        for option in options {
            // Directly unpack the Out variant since it's irrefutable.
            let SimulationOption::Out(vars) = option;
            for var in vars {
                filtered_headers.insert(var);
            }
        }

        // If no filtering is specified, use all headers.
        if filtered_headers.is_empty() {
            for step_data in data {
                for (var, _) in step_data {
                    filtered_headers.insert(var.name());
                }
            }
        }

        let headers: Vec<_> = filtered_headers.into_iter().collect();

        // Iterate over each step data and collect values based on filtered headers.
        for step_data in data.iter() {
            let mut values = vec![];
            for header in &headers {
                let mut value_str = String::new();
                for (var, val) in step_data {
                    if &var.name() == header {
                        value_str = format!("{val}");
                        break;
                    }
                }
                values.push(value_str);
            }
            println!("{}", values.join(","));
        }
    }

    /// Outputs Transient simulation results in CSV format.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of tuples, where each tuple contains a timestep and a vector of tuples with variables and their numeric values.
    fn output_tran(data: &Vec<(Numeric, Vec<(Variable, Numeric)>)>) {
        let mut headers: HashSet<Arc<str>> = HashSet::new();
        for (_, step_data) in data {
            for (var, _) in step_data {
                headers.insert(var.name());
            }
        }
        let mut headers: Vec<_> = headers.into_iter().collect();
        headers.sort();

        let mut header_row = vec!["Time".to_string()];
        for header in &headers {
            header_row.push(format!("{header}"));
        }
        println!("{}", header_row.join(","));

        for (step_time, step_data) in data.iter() {
            let mut values = vec![format!("{}", step_time)];
            for header in &headers {
                let mut value_str = String::new();
                for (var, val) in step_data {
                    if &var.name() == header {
                        value_str = format!("{}", val);
                        break;
                    }
                }
                values.push(value_str);
            }
            println!("{}", values.join(","));
        }
    }

    /// Outputs AC sweep simulation results in CSV format.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of tuples, where each tuple contains a frequency and a vector of tuples with variables and their complex values.
    fn output_ac(data: &Vec<(Numeric, Vec<(Variable, ComplexNumeric)>)>) {
        let mut headers: HashSet<Arc<str>> = HashSet::new();
        for (_, step_data) in data {
            for (var, _) in step_data {
                headers.insert(var.name());
            }
        }
        let mut headers: Vec<_> = headers.into_iter().collect();
        headers.sort();

        let mut header_row = vec!["Frequency".to_string()];
        for header in &headers {
            header_row.push(format!("{header} (Real)"));
            header_row.push(format!("{header} (Imag)"));
        }
        println!("{}", header_row.join(","));

        for (step_time, step_data) in data.iter() {
            let mut values = vec![format!("{}", step_time)];
            for header in &headers {
                let mut value_str_real = String::new();
                let mut value_str_imag = String::new();
                for (var, val) in step_data {
                    if &var.name() == header {
                        value_str_real = format!("{}", val.re);
                        value_str_imag = format!("{}", val.im);
                        break;
                    }
                }
                values.push(value_str_real);
                values.push(value_str_imag);
            }
            println!("{}", values.join(","));
        }
    }
}
