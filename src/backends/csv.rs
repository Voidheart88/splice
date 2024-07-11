use std::collections::HashSet;

use super::Backend;
use crate::models::Variable;
use crate::sim::simulation_result::Sim;
use crate::{sim::simulation_result::SimulationResults, BackendError};

/// A struct for handling CSV output of simulation results.
pub struct CsvBackend {}

/// Implementation of the `Output` trait for `CsvOutput`.
/// This implementation defines how the simulation results are output as CSV.
///
/// # Example
///
/// ```
/// let csv_output = CsvOutput::new();
/// let results = SimulationResult::new();
/// csv_output.output(results)?;
/// ```
impl Backend for CsvBackend {
    /// Outputs the simulation results in CSV format.
    ///
    /// # Parameters
    ///
    /// - `results`: The `SimulationResult` to be output.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the output operation succeeds, or an `BackendError` if it fails.
    fn output(&self, results: SimulationResults) -> Result<(), BackendError> {
        for res in results.iter() {
            match res {
                Sim::Op(res) => Self::output_op(res),
                Sim::Dc(res) => Self::output_dc(res),
            }
        }

        Ok(())
    }
}

impl CsvBackend {
    /// Creates a new `CsvOutput` instance.
    ///
    /// # Returns
    ///
    /// A new `CsvOutput` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let csv_output = CsvBackend::new();
    /// ```
    pub fn new() -> Self {
        CsvBackend {}
    }

    /// Outputs the operational simulation results in CSV format.
    ///
    /// # Parameters
    ///
    /// - `results`: A reference to a vector of tuples where each tuple contains a `String` and a `f64`.
    ///
    /// # Example
    ///
    /// ```
    /// let results = vec![("voltage".to_string(), 5.0), ("current".to_string(), 1.0)];
    /// CsvBackend::output_op(&results);
    /// ```
    fn output_op(results: &Vec<(Variable, f64)>) {
        for res in results {
            println!("{},{},{}", res.0.name(), res.1, res.0.unit())
        }
    }

    fn output_dc(data: &Vec<Vec<(Variable, f64)>>) {
        // Find all unique variables and print header
        let mut headers: HashSet<String> = HashSet::new();
        for step_data in data {
            for (var, _) in step_data {
                headers.insert(var.name().to_string());
            }
        }
        let headers: Vec<_> = headers.into_iter().collect();
        println!("Step,{}", headers.join(","));

        // Print each step's data
        for (step_idx, step_data) in data.iter().enumerate() {
            let mut values = vec![format!("{}", step_idx)];
            for header in &headers {
                let mut value_str = String::new();
                for (var, val) in step_data {
                    if &var.name().to_string() == header {
                        value_str = format!("{}", val);
                        break;
                    }
                }
                values.push(value_str);
            }
            println!("{}", values.join(","));
        }
    }
}
