#![deny(unsafe_code)]

mod backends;
mod consts;
mod frontends;
mod models;
mod outputs;
mod sim;

use clap::Parser;
use log::info;
use miette::{Diagnostic, Result};

use backends::{faer::FaerBackend, Backends, NalgebraBackend, RSparseBackend};
use frontends::*;
use outputs::*;
use sim::Simulator;
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

    #[arg(short, long, default_value = "nalgebra")]
    backend: Backends,

    #[arg(short, long, default_value = "error")]
    verbose: log::Level,

    #[arg(short, long, default_value = "csv")]
    output: Outputs,

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
    let results = match cli.backend {
        Backends::Faer => {
            let mut sim: Simulator<FaerBackend> = Simulator::from(sim);
            sim.run()
        }
        Backends::RSparse => {
            let mut sim: Simulator<RSparseBackend> = Simulator::from(sim);
            sim.run()
        }
        Backends::Nalgebra => {
            let mut sim: Simulator<NalgebraBackend> = Simulator::from(sim);
            sim.run()
        }
    };

    info!("Output Data");
    let out: Box<dyn Output> = match cli.output {
        Outputs::Csv => Box::new(CsvOutput::new()),
        Outputs::Raw => Box::new(RawOutput::new()),
        Outputs::Plot => Box::new(PlotOutput::new(pth)),
        Outputs::Network => Box::new(NetworkOutput::new()),
    };

    out.output(results?)?;

    Ok(())
}
