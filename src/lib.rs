#![deny(unsafe_code)]

pub mod backends;
pub mod frontends;
pub mod models;
pub mod sim;
pub mod solver;
pub mod spot;

use clap::Parser;
use log::{info, error};
use miette::{Diagnostic, Result};
use thiserror::Error;

use backends::*;
use frontends::*;
use sim::Simulator;
use solver::{FaerSolver, NalgebraSolver, RSparseSolver, Solvers};

use crate::{
    frontends::serde::SerdeFormat,
    sim::{simulation_result::SimulationResults, SimulatorError},
    solver::{FaerSparseSolver, Solver},
};

#[derive(Debug, Error, Diagnostic)]
enum ApplicationError {
    #[error("No Path given")]
    #[diagnostic(help("try setting a path when using Splice in cli mode"))]
    NoPathGiven,
}

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

fn run_sim<T: Solver>(sim: Simulation) -> Result<SimulationResults, SimulatorError> {
    let mut sim: Simulator<T> = Simulator::from(sim);
    sim.run()
}

/// Public function to run a simulation with a specific solver for benchmarking
/// This is exposed for benchmarking purposes and returns a simple Result
#[doc(hidden)]
pub fn run_sim_for_benchmark<T: Solver>(sim: Simulation) -> Result<(), String> {
    let mut sim: Simulator<T> = Simulator::from(sim);
    match sim.run() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}



pub fn run() -> Result<()> {
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

    // Apply autotune if enabled
    if cli.autotune {
        info!("Autotune mode enabled");
        let autotune_options = sim::autotune::analyze_circuit_and_suggest_settings(&sim.elements, &sim.commands);
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
            // For network backend, we need to accept the connection from the frontend
            // This is a temporary solution - in production, we'd want to pass the stream
            let listener = std::net::TcpListener::bind("0.0.0.0:8081").map_err(|e| BackendError::IoError(e.to_string()))?;
            let (stream, _) = listener.accept().map_err(|e| BackendError::IoError(e.to_string()))?;
            Box::new(NetworkBackend::new(stream))
        }
    };

    info!("Output Data");
    out.output(results)?;

    info!("Finished without Errors");
    Ok(())
}

fn network_loop(solver: Solvers) {
    // Start frontend on port 8080
    let frontend = match NetworkFrontend::new(8080) {
        Ok(f) => f,
        Err(_) => return, // Could not start network frontend
    };
    
    info!("Network mode started on port {}", frontend.get_port());
    
    loop {
        let sim = match frontend.simulation() {
            Ok(sim) => sim,
            Err(e) => {
                error!("Network frontend error: {}", e);
                continue;
            }
        };

        let results = match solver {
            Solvers::Rsparse => run_sim::<RSparseSolver>(sim),
            Solvers::Nalgebra => run_sim::<NalgebraSolver>(sim),
            Solvers::Faer => run_sim::<FaerSolver>(sim),
            Solvers::FaerSparse => run_sim::<FaerSparseSolver>(sim),
        };

        let results = match results {
            Ok(res) => res,
            Err(e) => {
                error!("Simulation error: {}", e);
                continue;
            }
        };

        // Create backend with the same connection
        let listener = match std::net::TcpListener::bind("0.0.0.0:8081") {
            Ok(l) => l,
            Err(_) => continue,
        };
        
        let (stream, _) = match listener.accept() {
            Ok(s) => s,
            Err(_) => continue,
        };
        
        let out = NetworkBackend::new(stream);
        match out.output(results) {
            Ok(_) => info!("Results sent successfully"),
            Err(e) => {
                error!("Network backend error: {}", e);
                continue;
            }
        };
    }
}
