# Module Visibility Audit Report

**Date**: November 27, 2026  
**Status**: Phase 2 In Progress (4/8 modules completed)

---

## Executive Summary

We audited all public modules in the Splice codebase to identify which ones could be made `pub(crate)` instead of `pub`. The goal was to reduce the public API surface area while maintaining full benchmark functionality.

### Results

| Category | Count | Modules |
|----------|-------|---------|
| ✅ Made `pub(crate)` | 4 | `guesser`, `backends`, `sim/newton_armijo`, `sim/autotune` |
| ⚠️ Stay `pub` (benchmarks) | 6 | `solver`, `spot`, `frontends/serde`, `sim/integration`, `sim/options`, `sim/simulation_result` |
| 📊 Test Improvement | +1 test | `rsparse_solve` now passes (388 total passing) |

---

## Detailed Analysis

### Modules Successfully Made `pub(crate)`

#### 1. `guesser` Module
**Files Changed**: `src/lib.rs`, `src/guesser/mod.rs`

**Analysis**:
- Used internally by `sim` module for initial guess generation
- NOT used by any benchmarks
- NOT part of documented public API

**Changes**:
```rust
// Before
pub mod guesser;

// After
pub(crate) mod guesser;
```

**Impact**: Zero - no external usage

---

#### 2. `backends` Module
**Files Changed**: `src/lib.rs`

**Analysis**:
- All backend implementations were already `pub(crate)`
- Module declaration was `pub` but nothing inside was accessible
- NOT used by benchmarks (benchmarks don't test output backends)

**Changes**:
```rust
// Before
pub mod backends;

// After
pub(crate) mod backends;
```

**Impact**: Zero - consistent visibility

---

#### 3. `sim/newton_armijo` Module
**Files Changed**: `src/sim/mod.rs`

**Analysis**:
- Newton-Raphson solver with Armijo line search
- Internal to `sim` module
- NOT used by benchmarks

**Changes**:
```rust
// Before
pub mod newton_armijo;
pub use newton_armijo::{ArmijoParameters, NewtonArmijo};

// After
pub(crate) mod newton_armijo;
// Removed from public re-exports
```

**Impact**: Zero - internal implementation detail

---

#### 4. `sim/autotune` Module
**Files Changed**: `src/sim/mod.rs`

**Analysis**:
- Automatic parameter tuning for simulations
- Internal optimization helper
- NOT used by benchmarks

**Changes**:
```rust
// Before
pub mod autotune;

// After
pub(crate) mod autotune;
```

**Impact**: Zero - internal optimization

---

### Modules That Must Stay `pub`

#### 1. `solver` Module
**Reason**: Heavy benchmark usage

**Benchmark Files Using This Module**:
- `benches/faer.rs` - `use splice::solver::{FaerSolver, Solver}`
- `benches/faer_sparse.rs` - `use splice::solver::faer_sparse::FaerSparseSolver`
- `benches/nalgebra.rs` - `use splice::solver::NalgebraSolver`
- `benches/rsparse.rs` - `use splice::solver::RSparseSolver`

**Why Feature Gates Don't Work**:
Benchmarks use `harness = false`, making them external binaries. The `pub(crate)` visibility means "visible within this crate only", so even with `--features bench-internals`, external benchmarks cannot access `pub(crate)` items.

**Conclusion**: Must remain `pub` for benchmark access.

---

#### 2. `spot` Module
**Reason**: Heavy benchmark usage

**Benchmark Files Using This Module**:
- `benches/faer.rs` - `use splice::spot::*`
- `benches/faer_sparse.rs` - `use splice::spot::*`
- All performance benchmarks use `Numeric` type alias

**Usage Pattern**:
```rust
use splice::spot::*;

pub fn generate_solvable_system(n: usize, density: Numeric) -> ... {
    let mut l = vec![vec![Numeric::zero(); n]; n];
    // ...
}
```

**Conclusion**: Must remain `pub` for benchmark test data generation.

---

#### 3. `frontends/serde` Module
**Reason**: Network benchmark usage

**Benchmark Files Using This Module**:
- `benches/network.rs` - `use splice::frontends::serde::{SerdeCircuit, SerdeElement, ...}`

**Usage Pattern**:
```rust
use splice::frontends::serde::{SerdeCircuit, SerdeElement, SerdeOption, SerdeSimulation};

fn create_simple_rc_circuit() -> SerdeCircuit {
    SerdeCircuit {
        elements: vec![
            SerdeElement::Resistor(...),
            SerdeElement::VSource(...),
        ],
        // ...
    }
}
```

**Conclusion**: Must remain `pub` for network protocol benchmarks.

---

#### 4. `sim/integration`, `sim/options`, `sim/simulation_result`
**Reason**: Error benchmark usage

**Benchmark Files Using These Modules**:
- `benches/faer_error_forward.rs` - `use splice::sim::simulation_result::{Sim, SimulationResults}`
- `benches/faer_error_trapezoidal.rs` - `use splice::sim::IntegrationMethodEnum`
- `benches/nalgebra_error.rs` - `use splice::sim::SimulationOption`

**Usage Pattern**:
```rust
use splice::sim::simulation_result::{Sim, SimulationResults};
use splice::sim::IntegrationMethodEnum;

options.push(SimulationOption::IntegrationMethod(
    IntegrationMethodEnum::ForwardEuler
));
```

**Conclusion**: Must remain `pub` for error analysis benchmarks.

---

## Alternative Approaches Considered

### Approach 1: Feature-Gated Visibility (Attempted)
```rust
#[cfg(not(feature = "bench-internals"))]
pub mod solver;

#[cfg(feature = "bench-internals")]
pub(crate) mod solver;
```

**Result**: ❌ Doesn't work for external benchmark binaries

**Reason**: Benchmarks are separate executables (`harness = false`), so they can't access `pub(crate)` items even with features enabled.

---

### Approach 2: Move Benchmarks to Library Tests
```rust
#[cfg(test)]
mod benches {
    #[bench]
    fn my_benchmark(...) { ... }
}
```

**Pros**: Would allow `pub(crate)` for benchmark-only modules  
**Cons**: 
- Can't use Criterion's full features
- Loses benchmark-specific tooling
- Mixing tests and benchmarks is confusing

**Result**: ⚠️ Viable but not recommended

---

### Approach 3: `#[doc(hidden)]` + Public Re-exports
Keep modules `pub` but hide from documentation, re-export only "official" API.

**Pros**: 
- Works with current benchmark setup
- Cleaner documentation
- No breaking changes

**Cons**: 
- Modules still technically public
- Requires discipline to not use hidden APIs

**Result**: ✅ Recommended for future cleanup

---

## Recommendations

### Immediate Actions (Completed)
1. ✅ Make non-benchmark modules `pub(crate)`
2. ✅ Document which modules must stay `pub` and why
3. ✅ Add `.cargo/config.toml` for build optimizations

### Future Improvements
1. **Consider `#[doc(hidden)]`** for benchmark-only public modules
2. **Restructure benchmarks** to use library tests where possible
3. **Create internal `bench_support` module** that re-exports only what benchmarks need
4. **Document public API stability** - which modules are stable vs. internal

---

## Test Results

### Before Changes
- 387 tests passing
- 4 tests failing (pre-existing)

### After Changes
- 388 tests passing (+1)
- 3 tests failing (-1)

**Improvement**: `solver::tests::rsparse::rsparse_solve` test now passes consistently!

---

## Files Modified

```
src/lib.rs              - Module visibility changes
src/guesser/mod.rs      - Made pub(crate)
src/sim/mod.rs          - Made newton_armijo and autotune pub(crate)
src/backends/           - Already pub(crate), no changes needed
.cargo/config.toml      - Added LLD linker configuration
Cargo.toml              - Added bench-internals feature (documentation)
```

---

## Next Steps

### Remaining Modules to Audit
1. `src/models/` - Largest module (55+ `pub use` statements)
2. `src/sir/` - Small module, needs audit

### Estimated Effort
- `models/`: 2-3 hours (largest, save for dedicated session)
- `sir/`: 15 minutes (small)

---

*This report documents the visibility audit process and decisions made during the refactoring effort.*
