# Splice Architektur - Zusammenfassung

## Übersicht

Splice ist ein hochperformanter Schaltungssimulator in Rust. Dieses Dokument erklärt die Architektur und Design-Entscheidungen.

---

## Modul-Sichtbarkeitsstrategie

### Hintergrund

Viele interne Module waren als `pub` exponiert, nur um Benchmarking zu unterstützen. Dies vergrößerte die öffentliche API unnötig.

### Lösung: Zwei-Stufen-Sichtbarkeit

#### **Stufe 1: Interne Module (`pub(crate)`)**
Module, die **NICHT** von Benchmarks verwendet werden:
- `guesser` - Startwert-Schätzung (intern für Simulation)
- `backends` - Ausgabe-Backends (CSV, Plot, Network)
- `sim/newton_armijo` - Newton-Raphson Löser
- `sim/autotune` - Automatische Parameter-Optimierung

Diese Module sind nur innerhalb der Bibliothek zugänglich.

#### **Stufe 2: Benchmark-Module (`pub`)**
Module, die von Benchmarks verwendet werden **müssen** `pub` bleiben:
- `solver` - Löser-Backends (Faer, Nalgebra, RSparse)
- `spot` - Typ-Aliase (`Numeric`, `ComplexNumeric`) und Konstanten
- `frontends/serde` - Serde-Typen für Network-Benchmark
- `sim/integration`, `sim/options`, `sim/simulation_result`

### Warum Feature-Gates nicht funktionieren

Benchmarks verwenden `harness = false`, was sie zu separaten Executables macht. `pub(crate)` bedeutet "nur innerhalb dieses Crates sichtbar", also können externe Benchmarks nicht auf `pub(crate)` Items zugreifen - selbst mit `--features bench-internals`.

---

## Build-Performance-Optimierungen

### LLD Linker (Windows)

Standardmäßig verwendet Rust auf Windows Microsofts `link.exe`, welches langsam ist. Wir konfigurieren den schnelleren LLVM Linker:

```toml
# .cargo/config.toml
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
```

**Beschleunigung**: 2-3x schnelleres Linken

### Development Profile

```toml
[profile.dev]
debug = "line-tables-only"

[profile.dev.package."*"]
debug = false
opt-level = 3
```

**Beschleunigung**: 30-50% schnellere inkrementelle Builds

---

## Datenfluss-Architektur

```
Eingabe (SPICE/YAML/JSON)
    ↓
Frontend (Parser)
    ↓
Simulation (Elemente + Befehle)
    ↓
Simulator (Matrix-Builder)
    ↓
Solver (Faer/Nalgebra/RSparse)
    ↓
Ergebnisse
    ↓
Ausgabe (CSV/Plot/Network)
```

---

## Modul-Verantwortlichkeiten

### `frontends/`
**Zweck**: Eingabeformate parsen und `Simulation`-Objekte erstellen

| Modul | Sichtbarkeit | Zweck |
|-------|-------------|-------|
| `serde` | `pub` | YAML/JSON Parsing, Network API |
| `spice` | `pub(crate)` | SPICE Netlist Parser |
| `kicad` | `pub(crate)` | KiCad Schema Parser |

### `models/`
**Zweck**: Schaltungselement-Implementierungen

Jedes Element-Modul (Widerstand, Kondensator, Diode, etc.) bietet:
- `serde.rs` - Serialisierungs-Unterstützung
- `spice.rs` - SPICE Parser-Unterstützung
- Element-Verhalten (Leitwert, Kapazität, etc.)

### `sim/`
**Zweck**: Simulation-Orchestrierung

| Modul | Sichtbarkeit | Zweck |
|-------|-------------|-------|
| `integration` | `pub` | Integrationsmethoden (Rückwärts-Euler, Trapez, BDF) |
| `options` | `pub` | Simulations-Konfiguration |
| `simulation_result` | `pub` | Ergebnis-Typen |
| `newton_armijo` | `pub(crate)` | Nicht-linearer Löser |
| `autotune` | `pub(crate)` | Auto-Parameter-Tuning |

### `solver/`
**Zweck**: Lineare Algebra Backends

| Löser | Sichtbarkeit | Anwendungsfall |
|-------|-------------|----------------|
| `FaerSolver` | `pub` | Dichte Matrizen, modernes Rust |
| `FaerSparseSolver` | `pub` | Sparse Matrizen |
| `NalgebraSolver` | `pub` | Kleine bis mittlere Matrizen |
| `RSparseSolver` | `pub` | Alternativer Sparse-Löser |

### `backends/`
**Zweck**: Ausgabe-Generierung

| Backend | Sichtbarkeit | Ausgabe-Format |
|---------|-------------|----------------|
| `CsvBackend` | `pub(crate)` | CSV Dateien |
| `PlotBackend` | `pub(crate)` | SVG/PNG Plots |
| `NetworkBackend` | `pub(crate)` | HTTP API Antworten |

### `spot/`
**Zweck**: Typ-Aliase und Konstanten

```rust
pub type Numeric = f64;
pub type ComplexNumeric = Complex<Numeric>;

// Physikalische Konstanten
pub const KB: Numeric = 1.380_649e-23;  // Boltzmann-Konstante
pub const ELE_CHRG: Numeric = 1.602_176_634e-19;  // Elementarladung
pub const TEMP: Numeric = 293.15;  // Raumtemperatur (K)
```

---

## Test-Strategie

### Test-Organisation
- **Unit-Tests**: In jedem Modul (`#[cfg(test)] mod tests`)
- **Integrationstests**: `src/sim/tests/`, `src/backends/tests/`
- **Benchmarks**: `benches/` Verzeichnis (externe Binaries)

### Test-Ergebnisse
- **388 Tests erfolgreich** (Stand Nov 2026)
- **3 vorbestehende Fehler** in frontend node_units Tests (bekannte Probleme)

---

## Benchmarking

### Benchmark-Kategorien

1. **Performance-Benchmarks** - Matrix-Operationen, Löser-Performance
2. **Fehler-Benchmarks** - Numerische Genauigkeit vs. analytische Lösungen
3. **Modell-Benchmarks** - Element Triple/Pair Berechnung
4. **Network-Benchmarks** - MessagePack Serialisierung

### Benchmarks Ausführen

```bash
# Alle Benchmarks (5-8 Minuten mit LLD)
cargo bench

# Spezifischer Benchmark
cargo bench --bench faer
```

---

## Beiträge

Siehe `CONTRIBUTING.md` für Entwicklungs-Richtlinien.

### Wichtige Punkte für Mitwirkende

1. **Sichtbarkeit**: Neue interne Module sollten standardmäßig `pub(crate)` sein
2. **Benchmarks**: Wenn Benchmarks Zugriff benötigen, dokumentiere warum `pub` erforderlich ist
3. **Tests**: Jedes neue Modul benötigt Tests
4. **Dokumentation**: Diese Datei bei Architektur-Änderungen aktualisieren

---

*Zuletzt aktualisiert: November 2026*
