# Splice - A blazingly fast Spice simulator

The goal of Splice is to provide a better simulation experience.

## Goals
- Better error reporting! Your simulation fails? Splice tells you why. It shows you non-convergent nodes and tries to use the best possible method to choose a solving strategy for your problems. A valid but non-converging simulation is considered a bug!
- Faster! Splice uses modern mechanisms like parallel element evaluation to be as fast as possible. The code is well-tested!
- Extensible! The interfaces between frontend, backend, and output modules are well-defined. New models, frontends, and backends are easy to implement.
- Minimal! Splice is runnable on small ARM PCs as well as on your HPC rig.
- Useful error‑messages!

## Features
- Working .dc simulation
- Working .op simulation
- Working .ac simulation
- Working V,I,R,L,C,D device models
- Minimal diode model
- Minimal mosfet model
- Capacitors and inductors work in .ac simulation
- Advanced transient simulation with adaptive time step control
- Multiple integration methods (Backward Euler, Trapezoidal)
- Network mode with MessagePack protocol – run Splice as a server for remote simulations

## Installation & Build
```bash
# Build the project (release mode for best performance)
cargo build --release
```

You can also run the examples directly without installing:
```bash
cargo run -- -h   # show CLI help
cargo run -- run test/simple_ac.cir   # execute a sample circuit
```

## Quick‑Start
1. Clone the repository and change into the project directory.
2. Build the binary as shown above.
3. Run a simulation on one of the example netlist files in the `test/` directory, e.g.:
```bash
./target/release/splice run test/simple_ac.cir
```
   This will produce CSV output in the current directory.

## Architecture Overview
Splice is organized into four main crates:
- **frontends** – parsers for KiCad, SPICE netlists, etc.
- **sim** – high‑level simulation orchestration (ac, dc, op, tran).
- **solver** – numeric back‑ends (faer, nalgebra, rsparse) handling the linear systems.
- **backends** – output formats (CSV, plot, network server) and optional data storage.
These layers communicate through well‑defined traits, making it straightforward to add new frontends, models, or solvers.

## CLI Overview
| Command | Description |
|---------|-------------|
| `ac`    | Perform an AC analysis |
| `dc`    | Perform a DC sweep |
| `op`    | Compute the operating point |
| `tran`  | Perform a transient analysis |
| `run <file>` | Convenience wrapper to select the appropriate sub‑command based on file extension |
| `server` | Start the network mode server (MessagePack protocol) |

Run `splice -h` for a full list of options.

## Example Output
Running the example `test/simple_ac.cir` creates a CSV file with columns for frequency, magnitude, and phase. You can plot it with your favourite tool, e.g. `gnuplot` or Python's `matplotlib`.

## Tests & Benchmarks
```bash
# Run all unit and integration tests
cargo test

# Run performance benchmarks (requires the `criterion` crate)
cargo bench
```
Benchmark results are stored under `target/criterion/`.

### Benchmarking Internal Functions

By default, some internal modules are only `pub(crate)` to keep the public API clean.
To benchmark internal functions, enable the `bench-internals` feature:

```bash
cargo bench --features bench-internals
```

This feature is used by the benchmark suite to access internal solver and simulation components.

### Benchmark Prelude Module

When using the `bench-internals` feature, you can use the benchmark prelude module to simplify your benchmark code:

```rust
#[cfg(feature = "bench-internals")]
use splice::bench_prelude::*;
```

The prelude re-exports commonly used types and utilities:
- `Numeric` - The numeric type used throughout splice
- `Solver` - The solver trait
- `FaerSolver`, `FaerSparseSolver`, `NalgebraSolver`, `RSparseSolver` - Solver implementations
- `Unit`, `Variable` - Model types
- `SerdeCircuit`, `SerdeElement`, `SerdeOption`, `SerdeSimulation` - Frontend types
- `create_simulation_from_spice()` - Parse SPICE netlists
- `run_sim_for_benchmark()`, `run_sim_with_results()` - Simulation runners
- `generate_solvable_system()` - Helper function to generate test linear systems
- `create_simple_rc_circuit()` - Helper function to create a simple RC test circuit
- `create_medium_circuit(size)` - Helper function to create medium-sized test circuits
- `create_large_circuit(rows, cols)` - Helper function to create large resistor grid circuits
- `generate_rc_step_circuit()` - Helper function to create RC step response circuits for error testing
- `rc_analytical_solution_golden()` - Analytical solution for RC step response

This reduces boilerplate in benchmark files and ensures consistency across the benchmark suite.

**Note:** The `bench_prelude` module is excluded from public documentation (`#[doc(hidden)]`) as it is intended for internal benchmark use only.

## Contributing
- Write tests for every module, frontend, backend, and output. Tests define the expected behavior.
- Find failing simulations and make them runnable – a failing simulation should be considered a bug.
- Write benchmarks: simulation should be as fast as possible.
- **Network mode testing**: help test the new network functionality with different clients and scenarios.
- **Documentation**: improve the network mode documentation and add more examples.

## Todos / Roadmap
### Frontends
- Build a KiCad frontend

### Solver
- Build a CUDA/OpenCL backend
- Improve adaptive time step control with more sophisticated error estimation

### Outputs
- Improve the CSV output
- Improve the plot output
- Build the raw output

### Models
- Implement magnetic simulations

### Features
- Implement an intelligent solving strategy finder (maybe an AI thingy?)

## Project Structure – Quick Overview

| Path | Purpose / Content |
|------|-------------------|
| **`src/`** | Core code. Divided into:<br>• `frontends/` – Parsers for KiCad, SPICE netlists, etc.<br>• `sim/` – Orchestration of AC/DC/OP/TRAN analyses.<br>• `solver/` – Numerical backends (faer, nalgebra, rsparse).<br>• `backends/` – Output formats (CSV, plot, network server). |
| **`src/models/`** | Component implementations (Resistor, Capacitor, Inductor, Diode, MOSFET, …) including `tests/` per model. |
| **`src/guesser/`** | Heuristics for automatic parameter estimation. |
| **`test/`** | Sample netlists (`*.cir`) and small scripts for running/deployment. |
| **`benches/`** | Criterion benchmarks for performance measurements (e.g., `Faer__solve`).<br>• `error.rs` - Error benchmark runner that tests numerical accuracy of all solvers against known analytical solutions<br>• `error/` - Subdirectory with individual solver error benchmarks (nalgebra, faer, rsparse, faer_sparse) |
| **`scripts/`** | Helper scripts (e.g., `generate_callgraph.py`). |
| **`doc/`** | Documentation sources (German & English, `.typ` files). |
| **`notes/`** | Additional drawings, papers, license info. |

#### How to Quickly Find What You Need
- **Start a simulation** → `cargo run -- run <path/to/file.cir>` (or the compiled binary `./target/release/splice …`).
- **Add a new component** → `src/models/<device>/` (see existing models as templates).
- **CLI extensions** → `src/main.rs` & `src/sim/commands.rs`.
- **Check performance** → `cargo bench` (benchmark results in `target/criterion/`).
- **Run tests** → `cargo test` (includes unit, integration, and solver tests).

---
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
