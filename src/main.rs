//! Splice - A blazingly fast Spice simulator
//! 
//! This is the main entry point for the Splice circuit simulator.
//! The simulator supports DC, AC, transient, and operating point analyses
//! for electronic circuits described in SPICE netlist format.

use miette::Result;
use splice::run;

/// Main entry point for the Splice circuit simulator.
///
/// This function initializes the application and starts the simulation process.
/// It handles command-line argument parsing, configuration, and error reporting.
///
/// # Returns
///
/// * `Ok(())` - If the simulation completes successfully
/// * `Err(miette::Error)` - If any error occurs during simulation setup or execution
///
/// # Examples
///
/// ```bash
/// # Run a DC sweep analysis
/// splice --input circuit.cir --dc V1 0 5 0.1
/// 
/// # Run a transient analysis
/// splice --input circuit.cir --tran 10ms 100ms
/// ```
fn main() -> Result<()> {
    run()
}
