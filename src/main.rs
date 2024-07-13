#![deny(unsafe_code)]

mod backends;
mod consts;
mod frontends;
mod models;
mod sim;
mod solver;

use clap::Parser;
use log::info;
use miette::{Diagnostic, Result};

use backends::*;
use frontends::*;
use sim::Simulator;
use solver::{FaerSolver, NalgebraSolver, RSparseSolver, Solvers};
use thiserror::Error;

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

    #[arg(short, long, default_value = "r-sparse")]
    solver: Solvers,

    #[arg(short, long, default_value = "error")]
    verbose: log::Level,

    #[arg(short, long, default_value = "csv")]
    backend: Backends,

    path: Option<String>,
}

fn main() -> Result<()> {
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
        Frontends::Yml => Box::new(YmlFrontend::new(pth.clone())),
        Frontends::Json => Box::new(JsonFrontend::new(pth.clone())),
        Frontends::Network => Box::new(NetworkFrontend::new()),
        Frontends::Kicad => Box::new(KicadFrontend::new()),
        Frontends::Select => SelectFrontend::from_path(pth.clone())?,
    };

    let sim = frontend.simulation()?;

    info!("Simulate!");
    // Fixme: Implement backend selection logic
    let results = match cli.solver {
        Solvers::Rsparse => {
            let mut sim: Simulator<RSparseSolver> = Simulator::from(sim);
            sim.run()
        }
        Solvers::Nalgebra => {
            let mut sim: Simulator<NalgebraSolver> = Simulator::from(sim);
            sim.run()
        }
        Solvers::Faer => {
            let mut sim: Simulator<FaerSolver> = Simulator::from(sim);
            sim.run()
        }
    };

    info!("Output Data");
    let out: Box<dyn Backend> = match cli.backend {
        Backends::Csv => Box::new(CsvBackend::new()),
        Backends::Raw => Box::new(RawBackend::new()),
        Backends::Plot => Box::new(PlotBackend::new(pth)),
        Backends::Network => Box::new(NetworkBackend::new()),
    };

    out.output(results?)?;

    Ok(())
}
