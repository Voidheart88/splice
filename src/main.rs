//! Splice CLI - Circuit Simulator Command Line Interface
//!
//! This is the main entry point for the Splice circuit simulator CLI.
//! For library usage, see the `splice` crate documentation.

use clap::Parser;
use log::{error, info};
use miette::{Diagnostic, Result};
use thiserror::Error;

use splice::sim::Simulator;
use splice::solver::{FaerSolver, FaerSparseSolver, NalgebraSolver, RSparseSolver, Solvers};
use splice::sim::{SimulatorError, simulation_result::SimulationResults};
use splice::models::{Element, Variable};
use splice::frontends::serde::{ProcessSerdeElement, SerdeCircuit, SerdeFormat};

// Import from modules directly (pub(crate) items accessible within crate)
use splice::frontends::{
    Frontend, Frontends, FrontendError,
    SpiceFrontend, SerdeFrontend, NetworkFrontend, KicadFrontend, SelectFrontend,
};
use splice::backends::{
    Backend, Backends, BackendError,
    CsvBackend, RawBackend, PlotBackend, NetworkBackend,
};

/// Splice CLI arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "select")]
    frontend: Frontends,

    #[arg(short, long, default_value = "faer-sparse")]
    solver: Solvers,

    #[arg(short, long, default_value = "error")]
    verbose: log::Level,

    #[arg(short, long, default_value = "csv")]
    backend: Backends,

    #[arg(long, default_value = "false")]
    autotune: bool,

    path: Option<String>,
}

/// Application-specific errors
#[derive(Debug, Error, Diagnostic)]
enum ApplicationError {
    #[error("No Path given")]
    #[diagnostic(help("try setting a path when using Splice in cli mode"))]
    NoPathGiven,
}

/// Run a simulation with a specific solver (internal helper)
fn run_sim<T: splice::solver::Solver>(
    sim: splice::frontends::Simulation,
) -> Result<SimulationResults, SimulatorError> {
    let mut sim: Simulator<T> = Simulator::from(sim);
    sim.run()
}

/// Simple network server that handles circuit simulation requests
/// Uses a single port (8080) for both receiving circuits and sending results
fn network_loop(solver: Solvers) {
    // Start listener on port 8080
    let listener = match std::net::TcpListener::bind("0.0.0.0:8080") {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to port 8080: {}", e);
            return;
        }
    };

    info!("Network server started on port 8080 (single-port mode)");

    loop {
        // Accept incoming connection
        let (stream, addr) = match listener.accept() {
            Ok(conn) => conn,
            Err(e) => {
                error!("Failed to accept connection: {}", e);
                continue;
            }
        };

        info!("New connection from {}", addr);

        // Handle the connection in a simple request-response manner
        if let Err(e) = handle_network_connection(stream, solver) {
            error!("Connection handling error: {}", e);
        }
    }
}

/// Handle a single network connection with request-response pattern
fn handle_network_connection(
    stream: std::net::TcpStream,
    solver: Solvers,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read circuit from stream
    let circuit: SerdeCircuit = match rmp_serde::decode::from_read(&stream) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to decode circuit: {}", e);
            // Send error response
            let error_response = serde_json::json!({
                "status": "error",
                "error": format!("Failed to decode circuit: {}", e),
                "details": "Invalid MessagePack format"
            });
            let mut stream = stream.try_clone()?;
            rmp_serde::encode::write(&mut stream, &error_response)?;
            return Err(e.into());
        }
    };

    // Convert circuit to simulation
    let sim = match convert_serde_circuit_to_simulation(circuit) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to convert circuit: {}", e);
            let error_response = serde_json::json!({
                "status": "error",
                "error": format!("Circuit conversion failed: {}", e),
                "details": "Invalid circuit structure"
            });
            let mut stream = stream.try_clone()?;
            rmp_serde::encode::write(&mut stream, &error_response)?;
            return Err(e.into());
        }
    };

    // Run simulation
    let results = match solver {
        Solvers::Rsparse => run_sim::<RSparseSolver>(sim),
        Solvers::Nalgebra => run_sim::<NalgebraSolver>(sim),
        Solvers::Faer => run_sim::<FaerSolver>(sim),
        Solvers::FaerSparse => run_sim::<FaerSparseSolver>(sim),
    };

    let results = match results {
        Ok(res) => res,
        Err(e) => {
            error!("Simulation failed: {}", e);
            let error_response = serde_json::json!({
                "status": "error",
                "error": format!("Simulation failed: {}", e),
                "details": "Check circuit for convergence issues"
            });
            let mut stream = stream.try_clone()?;
            rmp_serde::encode::write(&mut stream, &error_response)?;
            return Err(e.into());
        }
    };

    // Send results back to client
    let mut stream = stream.try_clone()?;
    rmp_serde::encode::write(&mut stream, &results)?;

    info!("Successfully processed simulation request");
    Ok(())
}

/// Convert SerdeCircuit to Simulation (extracted from NetworkFrontend)
fn convert_serde_circuit_to_simulation(
    circuit: SerdeCircuit,
) -> Result<splice::frontends::Simulation, FrontendError> {
    use std::collections::HashMap;
    use std::sync::Arc;

    let mut elements: Vec<Element> = Vec::new();
    let mut variables: Vec<Variable> = Vec::new();
    let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();

    circuit.elements.iter().for_each(|element| {
        element.process(&mut variables, &mut elements, &mut var_map);
    });

    let commands = circuit.simulations.into_iter().map(Into::into).collect();
    let options = circuit.options.into_iter().map(Into::into).collect();

    Ok(splice::frontends::Simulation {
        elements,
        commands,
        options,
        variables,
    })
}

/// Main entry point for the Splice CLI
fn main() -> Result<()> {
    let cli = Cli::parse();

    simple_logger::init_with_level(cli.verbose)
        .expect("Failed to initialize logger. This should not happen and indicates a system configuration issue.");

    if cli.frontend == Frontends::Network && cli.backend == Backends::Network {
        network_loop(cli.solver);
    }

    info!("Splice - a blazingly fast circuit simulator");
    let pth = match cli.path {
        Some(pth) => pth,
        None => return Err(ApplicationError::NoPathGiven.into()),
    };

    info!("Read schematic");
    let frontend: Box<dyn Frontend> = match cli.frontend {
        Frontends::Spice => Box::new(SpiceFrontend::new(pth.clone())),
        Frontends::Yaml => Box::new(SerdeFrontend::try_new_from_path(
            pth.clone(),
            SerdeFormat::Yaml,
        )?),
        Frontends::Json => Box::new(SerdeFrontend::try_new_from_path(
            pth.clone(),
            SerdeFormat::Json,
        )?),
        Frontends::Network => {
            let frontend = NetworkFrontend::new(8080)?;
            info!("Network frontend started on port {}", frontend.get_port());
            Box::new(frontend)
        }
        Frontends::Kicad => Box::new(KicadFrontend::new()),
        Frontends::Select => SelectFrontend::try_from_path(pth.clone())?,
    };

    let mut sim = frontend.simulation()?;

    // Setup coupled inductors by setting their node indices
    let coupling_errors = Element::setup_coupled_inductors(&mut sim.elements);
    if !coupling_errors.is_empty() {
        coupling_errors.iter().for_each(|err| error!("{err}"));
        return Err(SimulatorError::CircuitError(format!(
            "{} circuit coupling error(s) found. Simulation aborted.",
            coupling_errors.len()
        ))
        .into());
    }

    // Apply autotune if enabled
    if cli.autotune {
        info!("Autotune mode enabled");
        let autotune_options =
            splice::sim::autotune::analyze_circuit_and_suggest_settings(&sim.elements, &sim.commands);
        sim.options.extend(autotune_options);
    }

    info!("Simulate!");
    let results = match cli.solver {
        Solvers::Rsparse => run_sim::<RSparseSolver>(sim)?,
        Solvers::Nalgebra => run_sim::<NalgebraSolver>(sim)?,
        Solvers::Faer => run_sim::<FaerSolver>(sim)?,
        Solvers::FaerSparse => run_sim::<FaerSparseSolver>(sim)?,
    };

    let out: Box<dyn Backend> = match cli.backend {
        Backends::Csv => Box::new(CsvBackend::new()),
        Backends::Raw => Box::new(RawBackend::new()),
        Backends::Plot => Box::new(PlotBackend::new(pth)),
        Backends::Network => {
            // Network backend requires accepting connection from frontend
            // Current implementation uses a temporary listener on port 8081
            let listener = std::net::TcpListener::bind("0.0.0.0:8081")
                .map_err(|e| BackendError::IoError(e.to_string()))?;
            let (stream, _) = listener
                .accept()
                .map_err(|e| BackendError::IoError(e.to_string()))?;
            Box::new(NetworkBackend::new(stream))
        }
    };

    info!("Output Data");
    out.output(results)?;

    info!("Finished without Errors");
    Ok(())
}
