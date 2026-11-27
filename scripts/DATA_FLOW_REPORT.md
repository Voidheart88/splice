# Splice Data Flow Analysis Report

## Executive Summary

The data flow analyzer has identified the **core simulation pipeline** and tracked how data is transformed from input to output through `From` trait implementations and conversion methods.

**Key Finding**: The codebase uses a **frontend pattern** where each frontend implements a `simulation()` method that converts serialized input into the core `Simulation` structure.

---

## Main Data Transformation Pipeline

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

---

## Identified Data Transformations

### 1. Frontend → Simulation (Conversion Methods)

All frontends implement the `Frontend` trait with a `simulation()` method:

| Frontend | Method | Returns | Location |
|----------|--------|---------|----------|
| `SerdeFrontend` | `.simulation()` | `Result<Simulation, FrontendError>` | `frontends::serde` |
| `SpiceFrontend` | `.simulation()` | `Result<Simulation, FrontendError>` | `frontends::spice` |
| `KicadFrontend` | `.simulation()` | `Result<Simulation, FrontendError>` | `frontends::kicad` |
| `NetworkFrontend` | `.simulation()` | `Result<Simulation, FrontendError>` | `frontends::network` |

**Implementation Example** (`SerdeFrontend`):
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

---

### 2. Serde Types → Internal Types (From Traits)

#### Simulation Commands
```rust
impl From<SerdeSimulation> for SimulationCommand {
    fn from(simulation: SerdeSimulation) -> Self {
        match simulation {
            SerdeSimulation::OP => SimulationCommand::Op,
            SerdeSimulation::DC(serdedc) => SimulationCommand::Dc(
                Arc::from(serdedc.source),
                serdedc.vstart,
                serdedc.vstop,
                serdedc.vstep,
                None,
            ),
            SerdeSimulation::AC(serdeac) => {
                SimulationCommand::Ac(serdeac.fstart, serdeac.fstop, serdeac.fstep, ACMode::Dec)
            }
            SerdeSimulation::Tran(serdetran) => {
                SimulationCommand::Tran(serdetran.tstep, serdetran.tend)
            }
        }
    }
}
```

**Field Mappings**:
- `SerdeSimulation::OP` → `SimulationCommand::Op`
- `SerdeSimulation::DC(source, vstart, vstop, vstep)` → `SimulationCommand::Dc(Arc<source>, vstart, vstop, vstep, None)`
- `SerdeSimulation::AC(fstart, fstop, fstep)` → `SimulationCommand::Ac(fstart, fstop, fstep, ACMode::Dec)`
- `SerdeSimulation::Tran(tstep, tend)` → `SimulationCommand::Tran(tstep, tend)`

#### Simulation Options
```rust
impl From<SerdeOption> for SimulationOption {
    fn from(option: SerdeOption) -> Self {
        // Maps gmin, rshunt, integration method, out variables
    }
}
```

**Field Mappings**:
- `SerdeOption.gmin` → `SimulationOption::GMin(gmin)`
- `SerdeOption.rshunt` → `SimulationOption::RShunt(rshunt)`
- `SerdeOption.integration` → `SimulationOption::IntegrationMethod(...)`
- `SerdeOption.out` → `SimulationOption::Out(vec![Arc<str>])`

#### Element Conversion
```rust
impl From<SerdeVSourceStep> for Element {
    fn from(value: SerdeVSourceStep) -> Self {
        // Creates Variable instances for nodes
        // Builds element with proper node mapping
    }
}
```

**Field Mappings**:
- `SerdeVSourceStep.name` → Element name
- `SerdeVSourceStep.node0` → `Variable::new(...)`
- `SerdeVSourceStep.node1` → `Variable::new(...)`
- `SerdeVSourceStep.initial_value` → Element value

---

### 3. Bundle → Serde Types (Reverse Conversion)

```rust
impl From<VSourceStepBundle> for SerdeVSourceStep {
    fn from(value: VSourceStepBundle) -> Self {
        SerdeVSourceStep {
            name: value.name().to_string(),
            node0: value.node0.map(|v| v.name().to_string()),
            node1: value.node1.map(|v| v.name().to_string()),
            initial_value: value.initial_value,
        }
    }
}
```

**Purpose**: Enables serialization of internal representations back to serde format (for APIs, caching, etc.)

---

## Data Structure Overview

### Input Structures (frontends::serde)

| Structure | Fields | Purpose |
|-----------|--------|---------|
| `SerdeCircuit` | 3 | Top-level circuit definition |
| `SerdeFrontend` | 4 | Parsed circuit ready for simulation |
| `SerdeSimulation` | - | Simulation command (OP/DC/AC/Tran) |
| `SerdeOption` | - | Simulation options |
| `SerdeElement` | - | Serialized element data |

### Core Structures (frontends / models)

| Structure | Fields | Purpose |
|-----------|--------|---------|
| `Simulation` | 4 | Complete simulation specification |
| `SimulationCommand` | - | Type of analysis to perform |
| `SimulationOption` | - | Configuration options |
| `Element` | - | Circuit element trait |
| `Variable` | - | Node/branch variable reference |

### Solver Structures (sim)

| Structure | Fields | Purpose |
|-----------|--------|---------|
| `Simulator<F>` | 6 | Simulation executor with solver F |
| `SimulationResults` | 2 | Computed results |

### Output Structures (backends)

| Structure | Fields | Purpose |
|-----------|--------|---------|
| `CsvBackend` | 0 | CSV file export |
| `PlotBackend` | 1 | SVG/PNG plot generation |
| `NetworkBackend` | - | HTTP API responses |

---

## Key Insights

### 1. Clean Separation of Concerns
- **Frontends**: Parse input formats → `Simulation`
- **Core**: `Simulation` holds the canonical model
- **Simulator**: Executes simulation using solver backends
- **Backends**: Export results to various formats

### 2. From Trait Pattern
The codebase extensively uses `From` traits for conversions:
- ✅ **Pros**: Idiomatic Rust, composable, type-safe
- ⚠️ **Observation**: Most `From` implementations are for error types (18 of 24)
- ✅ **Data conversions**: 6 core transformations identified

### 3. Field Mapping Strategy
Conversions follow patterns:
- **Direct mapping**: `field: value.field.clone()`
- **Type conversion**: `field: Arc::from(value.field)`
- **Enum mapping**: `match value { Variant(x) => Target::Variant(x) }`
- **Optional handling**: `field: value.field.map(|v| v.to_string())`

---

## Recommendations for Codebase Improvements

### 1. Add Missing From Implementations
Consider adding `From` implementations for clearer data flow:
```rust
// Could be added:
impl From<SerdeCircuit> for SerdeFrontend { ... }
impl From<&Simulation> for Simulator<FaerSolver> { ... }
```

### 2. Document Transformation Logic
Add doc comments to `From` implementations explaining:
- Why the conversion exists
- Any assumptions or invariants
- Field mapping decisions

### 3. Consider Builder Pattern
For complex conversions (e.g., `SerdeSimulation` → `SimulationCommand`), a builder could improve readability:
```rust
SimulationCommandBuilder::from_serde(serdedc)
    .source(Arc::from(serdedc.source))
    .build()
```

### 4. Add Conversion Tests
Test that round-trip conversions preserve data:
```rust
#[test]
fn test_serde_to_element_roundtrip() {
    let serde_elem = SerdeVSourceStep { ... };
    let elem: Element = serde_elem.clone().into();
    let back: SerdeVSourceStep = elem.into();
    assert_eq!(serde_elem.name, back.name);
}
```

---

## Files Analyzed

- **147 Rust source files**
- **128 data structures** identified
- **24 transformations** tracked
- **4 main conversion methods** (Frontend trait implementations)
- **4 From trait implementations** (non-error types)

---

## Generated Outputs

1. **`data_flow_report.html`** - Interactive visualization
2. **`data_flow_analysis.json`** - Machine-readable export
3. **`DATA_FLOW_REPORT.md`** - This document

---

*Generated by Splice Data Flow Analyzer*
