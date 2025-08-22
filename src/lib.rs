#![deny(unsafe_code)]

pub mod backends;
pub mod frontends;
pub mod models;
pub mod sim;
pub mod solver;
pub mod spot;

use clap::Parser;
use log::info;
use miette::{Diagnostic, Result};
use thiserror::Error;

use backends::*;
use frontends::*;
use sim::Simulator;
use solver::{FaerSolver, NalgebraSolver, RSparseSolver, Solvers};

use crate::{
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

    #[arg(short, long, default_value = "rsparse")]
    solver: Solvers,

    #[arg(short, long, default_value = "error")]
    verbose: log::Level,

    #[arg(short, long, default_value = "csv")]
    backend: Backends,

    path: Option<String>,
}

fn run_sim<T: Solver>(sim: Simulation) -> Result<SimulationResults, SimulatorError> {
    let mut sim: Simulator<T> = Simulator::from(sim);
    sim.run()
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    simple_logger::init_with_level(cli.verbose).unwrap();

    info!("Splice - a blazingly fast circuit simulator");
    let pth = match cli.path {
        Some(pth) => pth,
        None => return Err(ApplicationError::NoPathGiven.into()),
    };

    info!("Read schematic");
    let frontend: Box<dyn Frontend> = match cli.frontend {
        Frontends::Spice => Box::new(SpiceFrontend::new(pth.clone())),
        Frontends::Yaml => Box::new(YamlFrontend::new_from_path(pth.clone())),
        Frontends::Json => Box::new(JsonFrontend::new(pth.clone())),
        Frontends::Network => Box::new(NetworkFrontend::new()),
        Frontends::Kicad => Box::new(KicadFrontend::new()),
        Frontends::Select => SelectFrontend::from_path(pth.clone())?,
    };

    let sim = frontend.simulation()?;

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
        Backends::Network => Box::new(NetworkBackend::new()),
    };

    info!("Output Data");
    out.output(results)?;

    info!("Finished without Errors");
    Ok(())
}
