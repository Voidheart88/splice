use std::collections::HashSet;
use std::sync::Arc;

use super::Backend;
use crate::models::Variable;
use crate::sim::simulation_result::Sim;
use crate::{sim::simulation_result::SimulationResults, BackendError};

/// A struct for handling CSV output of simulation results.
pub struct CsvBackend {}

/// Implementation of the `Output` trait for `CsvOutput`.
/// This implementation defines how the simulation results are output as CSV.
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
                Sim::Ac(_res) => return Err(BackendError::Unimplemented),
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
    pub fn new() -> Self {
        CsvBackend {}
    }

    /// Outputs the operational simulation results in CSV format.
    ///
    /// # Parameters
    ///
    /// - `results`: A reference to a vector of tuples where each tuple contains a `String` and a `f64`.
    fn output_op(results: &Vec<(Variable, f64)>) {
        for res in results {
            println!("{},{},{}", res.0.name(), res.1, res.0.unit())
        }
    }

    fn output_dc(data: &Vec<Vec<(Variable, f64)>>) {
        let mut headers: HashSet<Arc<str>> = HashSet::new();
        for step_data in data {
            for (var, _) in step_data {
                headers.insert(var.name());
            }
        }
        let headers: Vec<_> = headers.into_iter().collect();

        for (step_idx, step_data) in data.iter().enumerate() {
            let mut values = vec![format!("{}", step_idx)];
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
}
