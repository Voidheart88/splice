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
  - Das DRY Prinzip sollt eeingehalten werden -- Auch bei Tests.

## TODO Items (Found in Codebase)

### Current Sources - SERDE Support
- [x] `src/models/isource_pulse/serde.rs` - Implement full SERDE support ✅
- [x] `src/models/isource_step/serde.rs` - Implement full SERDE support ✅
- [x] `src/models/isource_sine/serde.rs` - Implement full SERDE support ✅

### Current Sources - SPICE Parsing
- [x] `src/models/isource_pulse/spice.rs` - Implement full SPICE parsing for pulse current sources ✅
- [x] `src/models/isource_step/spice.rs` - Implement full SPICE parsing for step current sources ✅
- [x] `src/models/isource_sine/spice.rs` - Implement full SPICE parsing for sinusoidal current sources ✅

### Voltage Sources - SERDE/SPICE Support
- [x] `src/models/vsource_pulse/serde.rs` - Implement full SERDE support ✅
- [x] `src/models/vsource_pulse/spice.rs` - Implement full SPICE parsing for pulse voltage sources ✅

### Functional Blocks - SERDE/SPICE Support
- [x] `src/models/integrator/serde.rs` - Implement full SERDE support ✅
- [x] `src/models/integrator/spice.rs` - Implement full SPICE parsing for integrator blocks ✅
- [x] `src/models/summer/serde.rs` - Implement full SERDE support ✅
- [x] `src/models/summer/spice.rs` - Implement full SPICE parsing for summer blocks ✅

### Code Review Items (CHECK) ✅ ALL RESOLVED
- [x] `src/solver/tests/selector.rs:9` - Thresholds already in spot.rs ✅
- [x] `src/solver/tests/selector.rs:20` - Thresholds already in spot.rs ✅
- [x] `src/sim/autotune.rs:149` - Refactored with early returns ✅
- [x] `src/sim/autotune.rs:196` - Enhanced documentation for Backward Euler choice ✅

## Code Organisation:
- Right now the integraton methords are more or less integrated in the integrator module in sim.
  - Check if it can be split up in submodules
- Find a way to semi-automatic visualize the data-flow for the user to improve management decisions
