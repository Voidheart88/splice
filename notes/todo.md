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
- [ ] Reduce interface publicity via `bench-internals` feature
  - **Approach**: Add `bench-internals` feature to Cargo.toml, gate public items
  - **Phase 1: Infrastructure** (Priority: High)
    - [ ] Add `bench-internals = []` feature to `Cargo.toml`
    - [ ] Update all benchmarks to use `--features bench-internals`
    - [ ] Test: `cargo build --lib` and `cargo bench --features bench-internals`
  - **Phase 2: Module-by-Module Audit** (Per-Module Approach)
    - [ ] **`src/lib.rs`** - Root module exports
      - [ ] Audit: `pub mod backends/frontends/guesser/models/sim/solver/spot`
      - [ ] Check which are used externally vs. benchmark-only
      - [ ] TODO: Add `#[cfg(not(feature = "bench-internals"))]` to benchmark-only mods
    - [ ] **`src/models/mod.rs`** - Model re-exports (55 pub use statements)
      - [ ] Audit: All `pub use` statements for Bundle structs
      - [ ] Check benchmark usage: `benches/models.rs` imports
      - [ ] Likely candidates for `pub(crate)` with feature gate:
        - [ ] `pub mod bjt/capacitor/controlled_sources/...` (21 model modules)
        - [ ] `pub use` re-exports for Bundle types
      - [ ] Keep public: Types used in examples or external API
    - [ ] **`src/sim/mod.rs`** - Simulation internals
      - [ ] Audit: `pub mod integration/newton_armijo/options/simulation_result/autotune`
      - [ ] Check benchmark usage: `benches/` imports from `splice::sim::`
      - [ ] Likely candidates:
        - [ ] `pub mod integration` - Used by benchmarks? Check `benches/`
        - [ ] `pub mod autotune` - Internal only?
        - [ ] `pub use IntegrationMethod/NewtonArmijo/etc.` - External API?
    - [ ] **`src/solver/mod.rs`** - Solver backends
      - [ ] Audit: `pub mod faer/faer_sparse/nalgebra/rsparse/selector`
      - [ ] Check benchmark usage: `benches/faer.rs`, `benches/nalgebra.rs`, etc.
      - [ ] Likely candidates:
        - [ ] All solver modules are benchmark-heavy
        - [ ] Consider: Keep `pub` but gate with `bench-internals`
    - [ ] **`src/frontends/mod.rs`** - Frontend parsers
      - [ ] Audit: `pub mod serde` (public) vs `pub(crate) mod spice`
      - [ ] Check benchmark usage: `benches/network.rs` uses `SerdeCircuit`
      - [ ] Likely candidates:
        - [ ] `pub mod serde` - Used by network benchmark, may need to stay public
        - [ ] `pub(crate) mod spice` - Already restricted ✅
    - [ ] **`src/backends/mod.rs`** - Output backends
      - [ ] Audit: Backend module visibility
      - [ ] Check benchmark usage
    - [ ] **`src/guesser/mod.rs`** - Guessing infrastructure
      - [ ] Audit: `pub mod default/traits` and `pub use` statements
      - [ ] Check if used externally or benchmark-only
    - [ ] **`src/spot/mod.rs`** - Numeric types (if exists)
      - [ ] Audit: Type exports
      - [ ] Check benchmark usage: `benches/faer.rs` uses `splice::spot::*`
  - **Phase 3: Implementation Pattern** (After Audit)
    - [ ] For each module, apply pattern:
      ```rust
      // Before:
      pub mod internal_module;
      
      // After:
      #[cfg(not(feature = "bench-internals"))]
      pub mod internal_module;
      
      #[cfg(feature = "bench-internals")]
      pub(crate) mod internal_module;
      ```
    - [ ] For re-exports:
      ```rust
      // Before:
      pub use internal::Type;
      
      // After:
      #[cfg(not(feature = "bench-internals"))]
      pub use internal::Type;
      
      #[cfg(feature = "bench-internals")]
      pub(crate) use internal::Type;
      ```
  - **Phase 4: Testing & Validation**
    - [ ] Test normal build: `cargo build --lib`
    - [ ] Test with feature: `cargo build --features bench-internals`
    - [ ] Run benchmarks: `cargo bench --features bench-internals`
    - [ ] Check docs: `cargo doc --no-deps` (verify bench-internals items hidden)
    - [ ] Run tests: `cargo test`
  - **Phase 5: Documentation**
    - [ ] Add feature documentation to `README.md`
    - [ ] Add inline comments explaining the pattern
    - [ ] Update `CONTRIBUTING.md` for benchmark authors
  - **Estimated Effort**: 5-8 hours total
    - Phase 1: 30 min
    - Phase 2: 2-3 hours (audit all modules)
    - Phase 3: 2-3 hours (implementation)
    - Phase 4: 1 hour
    - Phase 5: 30 min

## Documentation:
- [ ] Update des Style-Guides
- [ ] Update der Dokumentation
  - [ ] Beschreibung des Spice frontends
  - [ ] Beschreibung aller Module
  - [ ] Vergleich der Integration
  - [ ] Beschreibung aller Optionen 
  - [ ] die Infos sollten aus den docstrings der parts kommen.
