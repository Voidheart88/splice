use super::Output;
use crate::models::Variable;
use crate::sim::simulation_result::Sim;
use crate::{sim::simulation_result::SimulationResults, OutputError};

/// A struct for handling CSV output of simulation results.
pub struct CsvOutput {}

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
impl Output for CsvOutput {
    /// Outputs the simulation results in CSV format.
    ///
    /// # Parameters
    ///
    /// - `results`: The `SimulationResult` to be output.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the output operation succeeds, or an `OutputError` if it fails.
    fn output(&self, results: SimulationResults) -> Result<(), OutputError> {
        for res in results.iter() {
            match res {
                Sim::Op(res) => Self::output_op(res),
                Sim::Dc(_) => return Err(OutputError::Unimplemented),
            }
        }

        Ok(())
    }
}

impl CsvOutput {
    /// Creates a new `CsvOutput` instance.
    ///
    /// # Returns
    ///
    /// A new `CsvOutput` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let csv_output = CsvOutput::new();
    /// ```
    pub fn new() -> Self {
        CsvOutput {}
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
    /// CsvOutput::output_op(&results);
    /// ```
    fn output_op(results: &Vec<(Variable, f64)>) {
        for res in results {
            println!("{},{}, {}", res.0.name(), res.1, res.0.unit())
        }
    }
}
