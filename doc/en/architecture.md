# Splice Architecture & Design Decisions

## Overview

Splice is a high-performance circuit simulator written in Rust. This document explains the architecture, design decisions, and visibility management strategy.

---

## Module Visibility Strategy

### Background

During development, we identified that many internal modules were exposed as `pub` purely to support benchmarking. This created a larger public API surface than necessary, making it harder to evolve internal implementations without breaking external code.

### Solution: Two-Tier Visibility

We implemented a two-tier visibility strategy:

#### **Tier 1: Internal Modules (`pub(crate)`)**
Modules that are **NOT** used by external benchmarks:
- `guesser` - Initial guess generation (internal to simulation)
- `backends` - Output backends (CSV, Plot, Network)
- `sim/newton_armijo` - Newton-Raphson solver with Armijo line search
- `sim/autotune` - Automatic parameter tuning

These modules are `pub(crate)` only, meaning they're accessible within the library but not to external crates.

#### **Tier 2: Benchmark-Required Modules (`pub`)**
Modules that **ARE** used by benchmarks must remain `pub`:
- `solver` - Solver backends (Faer, Nalgebra, RSparse, FaerSparse)
- `spot` - Type aliases (`Numeric`, `ComplexNumeric`) and constants
- `frontends/serde` - Serde serialization types for network benchmark
- `sim/integration` - Integration method enums
- `sim/options` - Simulation options
- `sim/simulation_result` - Result types

### Why Not Feature-Gate Benchmark Modules?

We initially attempted to use a `bench-internals` Cargo feature to conditionally reduce visibility:

```toml
# Cargo.toml
[features]
bench-internals = []
```

However, this approach doesn't work for our benchmark setup because:

1. **External Binaries**: Benchmarks use `harness = false`, making them separate executables
2. **Crate Boundaries**: `pub(crate)` means "visible within this crate only"
3. **Feature Limitations**: Features can't override Rust's visibility rules across crate boundaries

The `bench-internals` feature remains in `Cargo.toml` as documentation and for potential future use if benchmarks are restructured.

---

## Build Performance Optimizations

### LLD Linker (Windows)

By default, Rust on Windows uses Microsoft's `link.exe`, which is sequential and slow. We configure the faster LLVM linker (`rust-lld.exe`):

```toml
# .cargo/config.toml
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
```

**Speedup**: 2-3x faster linking (measured on large projects)

### Development Profile Optimizations

```toml
[profile.dev]
debug = "line-tables-only"  # Minimal debug info for backtraces

[profile.dev.package."*"]
debug = false      # No debug info for dependencies
opt-level = 3      # Keep dependencies optimized
```

**Speedup**: 30-50% faster incremental builds

---

## Data Flow Architecture

### Main Simulation Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│                    SPICE SIMULATION DATA FLOW                        │
└─────────────────────────────────────────────────────────────────────┘

INPUT LAYER (File Formats)
│
├─ SPICE Netlist ──▶ SpiceFrontend ──┐
├─ YAML/JSON ─────▶ SerdeFrontend ───┼─▶ simulation() ──┐
├─ KiCad ─────────▶ KicadFrontend ───┤                 │
└─ Network ───────▶ NetworkFrontend ─┘                 │
                                                         ▼
                                                  ┌──────────────┐
                                                  │  Simulation  │
                                                  │ (Core Model) │
                                                  └──────────────┘
                                                         │
                                                         ▼
                                              ┌──────────────────┐
                                              │    Simulator     │
                                              │ (Matrix Builder) │
                                              └──────────────────┘
                                                         │
                                                         ▼
                                              ┌───────────────────┐
                                              │ Solver Backends   │
                                              │ - Faer            │
                                              │ - FaerSparse      │
                                              │ - Nalgebra        │
                                              │ - RSparse         │
                                              └───────────────────┘
                                                         │
                                                         ▼
                                              ┌─────────────────────┐
                                              │ SimulationResults   │
                                              └─────────────────────┘
                                                         │
                    ┌────────────────────────────────────┼────────────────────────────────────┐
                    ▼                                  ▼                                     ▼
           ┌────────────────┐                ┌────────────────┐                   ┌────────────────┐
           │  CsvBackend    │                │  PlotBackend   │                   │ NetworkBackend │
           │  (CSV Export)  │                │  (SVG/PNG)     │                   │  (HTTP API)    │
           └────────────────┘                └────────────────┘                   └────────────────┘
```

### Key Data Transformations

#### 1. Frontend → Simulation
All frontends implement the `Frontend` trait:
```rust
impl Frontend for SerdeFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        Ok(Simulation {
            commands: self.commands.clone(),
            options: self.options.clone(),
            elements: self.elements.clone(),
            variables: self.variables.clone(),
        })
    }
}
```

#### 2. Serde Types → Internal Types
```rust
impl From<SerdeSimulation> for SimulationCommand {
    fn from(simulation: SerdeSimulation) -> Self {
        match simulation {
            SerdeSimulation::OP => SimulationCommand::Op,
            SerdeSimulation::DC(serdedc) => SimulationCommand::Dc(...),
            SerdeSimulation::AC(serdeac) => SimulationCommand::Ac(...),
            SerdeSimulation::Tran(serdetran) => SimulationCommand::Tran(...),
        }
    }
}
```

#### 3. Element Conversion
```rust
impl From<SerdeVSourceStep> for Element {
    fn from(value: SerdeVSourceStep) -> Self {
        // Creates Variable instances for nodes
        // Builds element with proper node mapping
    }
}
```

---

## Module Responsibilities

### `frontends/`
**Purpose**: Parse input formats and create `Simulation` objects

| Module | Visibility | Purpose |
|--------|-----------|---------|
| `serde` | `pub` | YAML/JSON parsing, network API |
| `spice` | `pub(crate)` | SPICE netlist parser |
| `kicad` | `pub(crate)` | KiCad schematic parser |
| `network` | `pub(crate)` | Network protocol handler |

### `models/`
**Purpose**: Circuit element implementations

Each element module (resistor, capacitor, diode, etc.) provides:
- `serde.rs` - Serialization support
- `spice.rs` - SPICE parser support
- Element behavior (conductance, capacitance, etc.)

### `sim/`
**Purpose**: Simulation orchestration

| Module | Visibility | Purpose |
|--------|-----------|---------|
| `integration` | `pub` | Integration methods (Backward Euler, Trapezoidal, BDF) |
| `options` | `pub` | Simulation configuration |
| `simulation_result` | `pub` | Result types |
| `newton_armijo` | `pub(crate)` | Non-linear solver |
| `autotune` | `pub(crate)` | Auto-parameter tuning |

### `solver/`
**Purpose**: Linear algebra backends

| Solver | Visibility | Use Case |
|--------|-----------|----------|
| `FaerSolver` | `pub` | Dense matrices, modern Rust |
| `FaerSparseSolver` | `pub` | Sparse matrices |
| `NalgebraSolver` | `pub` | Small to medium matrices |
| `RSparseSolver` | `pub` | Alternative sparse solver |

### `backends/`
**Purpose**: Output generation

| Backend | Visibility | Output Format |
|---------|-----------|---------------|
| `CsvBackend` | `pub(crate)` | CSV files |
| `PlotBackend` | `pub(crate)` | SVG/PNG plots |
| `NetworkBackend` | `pub(crate)` | HTTP API responses |
| `RawBackend` | `pub(crate)` | Raw binary data |

### `spot/`
**Purpose**: Type aliases and constants

```rust
pub type Numeric = f64;
pub type ComplexNumeric = Complex<Numeric>;

// Physical constants
pub const KB: Numeric = 1.380_649e-23;  // Boltzmann constant
pub const ELE_CHRG: Numeric = 1.602_176_634e-19;  // Elementary charge
pub const TEMP: Numeric = 293.15;  // Room temperature (K)
pub const UT: Numeric = KB * TEMP / ELE_CHRG;  // Thermal voltage

// Simulation constants
pub const VECTOL: Numeric = 1e-3;  // Voltage tolerance
pub const MAXITER: usize = 1000;  // Maximum iterations
```

### `guesser/`
**Purpose**: Initial guess generation for non-linear solvers

```rust
pub(crate) trait Guesser<SO: Solver> {
    fn guess(&self, sim: &mut Simulator<SO>) -> Result<Vec<Numeric>, SimulatorError>;
}
```

---

## Testing Strategy

### Test Organization
- **Unit tests**: In each module (`#[cfg(test)] mod tests`)
- **Integration tests**: `src/sim/tests/`, `src/backends/tests/`
- **Benchmarks**: `benches/` directory (external binaries)

### Test Results
- **388 tests passing** (as of Nov 2026)
- **3 pre-existing failures** in frontend node_units tests (known issues)

---

## Benchmarking

### Benchmark Categories

1. **Performance Benchmarks** (`benches/performance.rs`)
   - Matrix operations
   - Solver performance
   - Sparsity patterns

2. **Error Benchmarks** (`benches/error.rs` and variants)
   - Numerical accuracy vs. analytical solutions
   - Integration method comparison
   - RC circuit step response

3. **Model Benchmarks** (`benches/models.rs`)
   - Element triple/pair computation
   - Diode, resistor, transistor models

4. **Network Benchmarks** (`benches/network.rs`)
   - MessagePack serialization/deserialization
   - Network protocol overhead

### Running Benchmarks

```bash
# All benchmarks (5-8 minutes with LLD)
cargo bench

# Specific benchmark
cargo bench --bench faer

# With bench-internals feature (documentation only)
cargo bench --features bench-internals
```

### Benchmark Prelude Module

The `bench_prelude` module (`src/bench_prelude.rs`) provides a consolidated set of imports and utilities for benchmarks:

```rust
#[cfg(feature = "bench-internals")]
use splice::bench_prelude::*;
```

**Purpose:**
- Reduces boilerplate in benchmark files
- Consolidates common imports (solvers, types, helpers)
- Keeps benchmark-specific code out of public API (`#[doc(hidden)]`)

**Exports:**
- Solver types: `FaerSolver`, `FaerSparseSolver`, `NalgebraSolver`, `RSparseSolver`
- Core types: `Numeric`, `Solver`, `Unit`, `Variable`
- Frontend types: `SerdeCircuit`, `SerdeElement`, etc.
- Helper functions: `generate_solvable_system()`, `create_*_circuit()`, `rc_analytical_solution_golden()`

**Note:** The module is excluded from public documentation but available when the `bench-internals` feature is enabled.

### Build Configuration

See `.cargo/config.toml` for linker and profile optimizations.

---

## Contributing

See `CONTRIBUTING.md` for development guidelines.

### Key Points for Contributors

1. **Visibility**: New internal modules should be `pub(crate)` by default
2. **Benchmarks**: If benchmarks need access, document why `pub` is required
3. **Tests**: Every new module needs tests
4. **Documentation**: Update this file for architectural changes

---

*Last updated: November 2026*
