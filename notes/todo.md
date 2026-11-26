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
- [ ] Find a way to semi-automatic visualize the data-flow for the user to improve management decisions
