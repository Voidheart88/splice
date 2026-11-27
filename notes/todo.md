# ToDos:

## Code Quality (Style Guide)
- Abarbeiten der verbleibenden FIXME, CHECK und TODO Markierungen 
  - For/While - Loops sollten wenn möglich gegen Iteratoren getauscht werden
  - Ein funktionaler Code-Stil sollte beibehalten werden
  - Die Nestingtiefe sollte auf 4 Ebenen begrenzt werden -- Das betrifft auch eventuelle Ausdrücke, 
    die so lang sind, dass sie über mehrerere Zeilen gehen.
  - Die zyklomatische Komplexität sollte minimiert werden
  - Die Länge der Funktionen sollte klein sein.
  - Eine Funktion sollte Daten transformieren und nicht mutieren.
  - Das DRY Prinzip sollte eingehalten werden -- Auch bei Tests.

## Code Organisation:
- [x] Move integration tests to dedicated test file ✅
- [x] Refactor integrator methods into submodules ✅
  - Split `src/sim/integration.rs` (588 lines) into modular structure:
    - `src/sim/integration/mod.rs` - Traits + re-exports
    - `src/sim/integration/backward_euler.rs` - BackwardEuler implementation
    - `src/sim/integration/forward_euler.rs` - ForwardEuler implementation
    - `src/sim/integration/trapezoidal.rs` - Trapezoidal implementation
    - `src/sim/integration/bdf.rs` - BDF implementation
- [x] Add data-flow visualization tool ✅
  - Created `scripts/data_flow_analyzer.py` 
  - Tracks 128 data structures and From trait implementations
  - Generates interactive HTML report showing data transformations
  - Identified main pipeline: SerdeCircuit → Simulation → Simulator → Results
- [x] Reduce interface publicity via `bench-internals` feature ✅
  - **Approach**: Created `bench_prelude` module with `#[doc(hidden)]` instead of gating individual modules
  - **Implementation** ✅ COMPLETE (Nov 2026)
    - [x] Created `src/bench_prelude.rs` with consolidated imports and utilities
    - [x] Added `#[doc(hidden)]` to exclude from public documentation
    - [x] Exported: Solvers, Numeric, Unit, Variable, Serde types, helper functions
    - [x] Refactored all 20+ benchmark files to use `splice::bench_prelude::*`
    - [x] Added `#![cfg(feature = "bench-internals")]` to all benchmark files
    - [x] Updated `README.md` with comprehensive prelude documentation
    - [x] Updated `doc/en/architecture.md` with prelude section
    - [x] Verified: `cargo check --benches --features bench-internals` ✅
    - [x] Verified: `bench_prelude` hidden from docs (`cargo doc`) ✅
  - **Benefits**:
    - Eliminates code duplication (e.g., `generate_solvable_system` in 4 files)
    - Reduces boilerplate across benchmark suite
    - Maintains clean public API without conditional compilation complexity
    - Easier maintenance: changes propagate automatically
  - **Committed**: Git commit `4faa4fb` - "refactor(benchmarks): consolidate imports with bench_prelude module"

## Documentation:
- [ ] Update des Style-Guides
- [x] Benchmark prelude documentation ✅
  - [x] Added to `README.md` with full export list
  - [x] Added to `doc/en/architecture.md` in Benchmarking section
- [ ] Update der Dokumentation
  - [ ] Beschreibung des Spice frontends
  - [ ] Beschreibung aller Module
  - [ ] Vergleich der Integration
  - [ ] Beschreibung aller Optionen 
  - [ ] die Infos sollten aus den docstrings der parts kommen.

## Public interface:
- [ ] Define the pub interface for the lib part
- [x] Benchmarks excluded from public API ✅
  - [x] `bench_prelude` marked with `#[doc(hidden)]`
  - [x] All benchmark files feature-gated with `#![cfg(feature = "bench-internals")]`
