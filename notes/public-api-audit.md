# Public API Audit - Splice

**Date**: November 27, 2026  
**Total Public Items**: 564 (excluding `pub(crate)`, `pub(super)`, etc.)

> **Note**: This manual audit has been superseded by the automated script at `scripts/analyze_public_api.py`.
> Run `python scripts/analyze_public_api.py` to generate an up-to-date report at `notes/public_api_report.md`.

---

## Executive Summary

### Root Module Exports (`src/lib.rs`)
- **5 public modules**: `frontends`, `models`, `sim`, `solver`, `spot`
- **2 public functions**: `run_sim_for_benchmark()`, `run_sim_with_results()`
- **1 public function**: `run()` (CLI entry point)
- **1 hidden module**: `bench_prelude` (marked with `#[doc(hidden)]`)

### Modules NOT Public (Internal Only)
- ✅ `backends` - `pub(crate)` - Output generation (CSV, Plot, Network)
- ✅ `guesser` - `pub(crate)` - Initial guess generation

---

## Detailed Breakdown by Module

### 1. **`spot/`** - Type Aliases & Constants (14 items)
**Purpose**: Core type definitions and physical constants

**Type Aliases**:
- `pub type Numeric = f64`
- `pub type ComplexNumeric = Complex<Numeric>`

**Constants**:
- `NUMERIC_TEN`, `NUMERIC_TWO`
- `KB` (Boltzmann constant)
- `ELE_CHRG` (Elementary charge)
- `TEMP` (Room temperature)
- `DEFAULT_CONDUCTANCE`
- `UT` (Thermal voltage)
- `DIO_GUESS`
- `VECTOL` (Voltage tolerance)
- `MAXITER` (Maximum iterations)
- `SOLVER_SELECTION_THRESHOLD`

---

### 2. **`models/`** - Circuit Element Models (~100 items)
**Purpose**: Circuit element implementations and bundles

#### Module Structure (21 submodules):
```
pub mod bjt;
pub mod capacitor;
pub mod controlled_sources;
pub mod coupled_inductors;
pub mod diode;
pub mod gain;
pub mod inductor;
pub mod integrator;
pub mod isource;
pub mod isource_pulse;
pub mod isource_sine;
pub mod isource_step;
pub mod mosfet;
pub mod pairs;
pub mod resistor;
pub mod summer;
pub mod triples;
pub mod vsource;
pub mod vsource_pulse;
pub mod vsource_sine;
pub mod vsource_step;
```

#### Re-exports (24 Bundle types):
- Passive: `ResistorBundle`, `CapacitorBundle`, `InductorBundle`
- Active: `DiodeBundle`, `Bjt0Bundle`, `Bjt1Bundle`, `Mos0Bundle`
- Sources: `VSourceBundle`, `VSourcePulseBundle`, `VSourceSinBundle`, `VSourceStepBundle`
-         `ISourceBundle`, `ISourcePulseBundle`, `ISourceSinBundle`, `ISourceStepBundle`
- Controlled: `VCVSBundle`, `VCCSBundle`, `CCCSBundle`, `CCVSBundle`
- Other: `GainBundle`, `SummerBundle`, `IntegratorBundle`, `CoupledInductorsBundle`
- Utilities: `Pairs<T>`, `Triples<T>`, `TripleIdx`

#### Core Types:
- `pub enum Unit` - Measurement units (Volt, Ampere, Ohm, etc.)
- `pub struct Variable` - Circuit variable (node, unit, index)
- `pub enum Element` - All circuit element types

#### Serde Types (for serialization):
- `SerdeResistor`, `SerdeCapacitor`, `SerdeInductor`
- `SerdeDiode`, `SerdeBjt0`, `SerdeBjt1`, `SerdeMos0`
- `SerdeVSource`, `SerdeVSourcePulse`, `SerdeVSourceSin`, `SerdeVSourceStep`
- `SerdeISource`, `SerdeISourcePulse`, `SerdeISourceSin`, `SerdeISourceStep`
- `SerdeVCVS`, `SerdeVCCS`, `SerdeCCCS`, `SerdeCCVS`
- `SerdeGain`, `SerdeSummer`, `SerdeIntegrator`, `SerdeCoupledInductors`

#### Options Types:
- `DiodeOptions`
- `Bjt0Options`, `Bjt1Options`
- `Mos0Options`
- `VCVSOptions`, `VCCSOptions`, `CCCSOptions`, `CCVSOptions`
- `CoupledInductorsOptions`

---

### 3. **`frontends/`** - Input Parsers (~30 items)
**Purpose**: Parse circuit descriptions from various formats

#### Modules:
- `pub mod serde` - YAML/JSON parsing
- `pub mod spice` - SPICE netlist parser
- `pub mod kicad` - KiCad schematic parser (stub)

#### Core Types:
- `pub enum FrontendError` - Error types for frontend parsing
- `pub struct Simulation` - Core simulation data structure
- `pub trait Frontend` - Frontend trait
- `pub fn create_simulation_from_spice()` - Parse SPICE netlist

#### Serde Module:
- `pub enum SerdeElement` - Serialized element types
- `pub enum SerdeSimulation` - OP, DC, AC, Tran
- `pub struct SerdeDC`, `SerdeAC`, `SerdeTran`
- `pub struct SerdeOption` - Simulation options
- `pub struct SerdeCircuit` - Complete circuit description
- `pub struct SerdeFrontend` - Serde frontend implementation
- `pub struct SelectFrontend` - Auto-detect frontend

#### SPICE Parser:
- `pub struct SpiceParser`
- `pub struct SpiceFrontend`
- `pub struct SpiceElementParser`
- Helper functions: `missing_element_error()`, `missing_node_error()`, `invalid_value_error()`

---

### 4. **`solver/`** - Linear Algebra Backends (~30 items)
**Purpose**: Matrix solvers for circuit equations

#### Modules:
- `pub mod faer` - Dense solver (Faer library)
- `pub mod faer_sparse` - Sparse solver (Faer library)
- `pub mod nalgebra` - Dense solver (Nalgebra library)
- `pub mod rsparse` - Sparse solver (RSparse library)
- `pub mod selector` - Solver selection strategy

#### Solver Implementations:
- `pub struct FaerSolver`
- `pub struct FaerSparseSolver`
- `pub struct NalgebraSolver`
- `pub struct RSparseSolver`

#### Selector:
- `pub enum SolverSelectionStrategy`
- `pub struct SolverSelector`

#### Core Types:
- `pub enum SolverError` - Solver error types
- `pub enum Solvers` - CLI solver selection
- `pub trait Solver` - Solver trait (insert_a, insert_b, solve, etc.)

---

### 5. **`sim/`** - Simulation Engine (~50 items)
**Purpose**: Simulation orchestration and time integration

#### Modules:
- `pub mod integration` - Integration methods
- `pub mod options` - Simulation options
- `pub mod simulation_result` - Result types
- `pub mod autotune` - Automatic parameter tuning
- `pub mod newton_armijo` - Newton-Raphson with line search

#### Core Types:
- `pub enum SimulatorError` - Simulation errors
- `pub struct Simulator<SO: Solver>` - Main simulator struct

#### Integration Methods:
- `pub trait IntegrationMethod` - Base trait
- `pub trait SingleStepMethod` - Single-step methods (Euler, Trapezoidal)
- `pub trait MultiStepMethod` - Multi-step methods (BDF)
- `pub struct BackwardEuler`
- `pub struct ForwardEuler`
- `pub struct Trapezoidal`
- `pub struct BDF`
- `pub enum IntegrationMethod` - Re-export enum
- `pub enum IntegrationMethodEnum` - Options enum

#### Options:
- `pub enum SimulationOption` - .options settings
- `pub mod defaults` - Default values

#### Results:
- `pub enum Sim` - DC, AC, Tran, OP result types
- `pub struct SimulationResults` - Complete results

#### Autotune:
- `pub fn analyze_circuit_and_suggest_settings()`
- `pub fn analyze_transient_parameters()`
- `pub fn analyze_ac_parameters()`

#### Newton-Armijo:
- `pub struct ArmijoParameters`
- `pub trait ResidualNorm`
- `pub struct NewtonArmijo`

---

### 6. **`backends/`** - Output Generation (~10 items)
**Purpose**: Generate output in various formats

#### Core Types:
- `pub enum BackendError` - Backend error types
- `pub enum Backends` - CLI backend selection
- `pub trait Backend` - Backend trait

#### Implementations:
- `pub struct CsvBackend`
- `pub struct PlotBackend`
- `pub struct RawBackend`
- `pub struct NetworkBackend` (in network.rs)

---

### 7. **`guesser/`** - Initial Guess Generation (~10 items)
**Purpose**: Generate initial guesses for non-linear solvers

#### Modules:
- `pub mod traits` - Guesser trait
- `pub mod default` - Standard guesser

#### Core Types:
- `pub struct GuesserOptions`
- `pub trait Guesser<SO: Solver>`
- `pub fn new_guesser()` - Factory function
- `pub struct StandardGuesser`

---

### 8. **`bench_prelude/`** - Benchmark Utilities (12 items) ⚠️ HIDDEN
**Purpose**: Consolidated imports for benchmarks  
**Visibility**: `#[doc(hidden)]` - Excluded from public docs

#### Re-exports:
- `pub use crate::spot::Numeric`
- `pub use crate::solver::{Solver, FaerSolver, FaerSparseSolver, NalgebraSolver, RSparseSolver}`
- `pub use crate::models::{Unit, Variable}`
- `pub use crate::frontends::create_simulation_from_spice`
- `pub use crate::frontends::serde::{SerdeCircuit, SerdeElement, SerdeOption, SerdeSimulation}`
- `pub use crate::run_sim_for_benchmark`
- `pub use crate::run_sim_with_results`

#### Helper Functions:
- `pub fn generate_solvable_system()` - Generate test linear systems
- `pub fn create_simple_rc_circuit()` - Simple RC circuit
- `pub fn create_medium_circuit()` - Medium-sized circuit
- `pub fn create_large_circuit()` - Large resistor grid
- `pub fn generate_rc_step_circuit()` - RC step response circuit
- `pub fn rc_analytical_solution_golden()` - Analytical solution

---

### 9. **`sir/`** - Multi-Step Integration Support (~5 items)
**Purpose**: State storage for multi-step methods

#### Core Types:
- `pub trait MultiStepIntegration` - Multi-step integration trait
- `pub struct BDFState` - BDF state storage
- `pub struct BDFSolver` - BDF solver helper

---

### 10. **`sim/tests/`** - Solver Test Utilities (~10 items)
**Purpose**: Test helpers for solver validation

#### Test Functions:
- `pub fn generate_solvable_system()` - Test system generator
- `pub fn faer_solve()` - Faer solver test
- `pub fn faer_sparse_solve()` - Faer sparse test
- `pub fn nalgebra_solve()` - Nalgebra test
- `pub fn rsparse_solve()` - RSparse test
- `pub fn newton_raphson_test()` - Newton-Raphson test
- `pub fn rsparse_update_sprs()` - RSparse update test
- `pub fn rsparse_update_b_vec()` - RSparse b-vector test

---

## Public API Surface Analysis

### By Category:

| Category | Count | Percentage |
|----------|-------|------------|
| Models & Elements | ~100 | 38% |
| Simulation Engine | ~50 | 19% |
| Frontends | ~30 | 11% |
| Solvers | ~30 | 11% |
| Type Aliases & Constants | 14 | 5% |
| Backends | ~10 | 4% |
| Guesser | ~10 | 4% |
| Bench Prelude (hidden) | 12 | 5% |
| SIR (Multi-step) | ~5 | 2% |
| Test Utilities | ~10 | 4% |

### Visibility Strategy:

#### Currently Public (should stay):
- ✅ `models` - External API for custom elements
- ✅ `frontends` - Used by network benchmark, may be external API
- ✅ `solver` - Core functionality, used by benchmarks
- ✅ `sim` - Simulation control, used by benchmarks
- ✅ `spot` - Core types used everywhere

#### Currently Public (candidates for `pub(crate)`):
- ⚠️ `backends` - Only used internally, not by benchmarks → **Already `pub(crate)`** ✅
- ⚠️ `guesser` - Only used internally → **Already `pub(crate)`** ✅

---

## Recommendations

### 1. **Keep Public** (External API or Benchmark Usage)
- `spot` - Core types
- `models` - Element definitions
- `solver` - Solver backends
- `frontends::serde` - Network API, serialization
- `sim::integration` - Integration methods
- `sim::simulation_result` - Result types
- `sim::options` - Configuration

### 2. **Consider Making `pub(crate)`** (Internal Only)
- `frontends::spice` - SPICE parser (internal)
- `frontends::kicad` - KiCad parser (internal)
- `sim::autotune` - Internal tuning
- `sim::newton_armijo` - Internal solver
- `backends/*` - Output generation (already `pub(crate)`)
- `guesser/*` - Guess generation (already `pub(crate)`)

### 3. **Already Hidden**
- ✅ `bench_prelude` - Marked with `#[doc(hidden)]`

---

## Next Steps

1. **Audit each module** for actual external usage
2. **Check examples/ directory** for external API usage
3. **Review network mode** - what's actually exposed via API?
4. **Consider feature-gating** benchmark-only items
5. **Document public API** - create clear API documentation

---

*Generated from grep analysis of 261 public items*
