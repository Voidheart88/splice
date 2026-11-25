# Eine Liste mit ToDos:

- Abarbeiten der verbleibenden FIXME und CHECK Markierungen 
  - For/While - Loops sollten wenn möglich gegen Iteratoren getauscht werden
  - Ein funktionaler Code-Stil sollte beibehalten werden
  - Die Nestingtiefe sollte auf 4 Ebenen begrenzt werden -- Das betrifft auch eventuelle Ausdrücke, 
    die so lang sind, dass sie über mehrerere Zeilen gehen.
  - Die zyklomatische Komplexität sollte minimiert werden
  - Die Länge der Funktionen sollte klein sein.
  - Eine Funktion sollte Daten transformieren und nicht mutieren.
  - Das DRY Prinzip sollt eeingehalten werden -- Auch bei Tests.
- Update des Style-Guides
- ~~Erstellung von zusätzlichen Current-Sources (Sine, Pulse, Step)~~ ✅
- Erstellung von zusätzlichen Voltage-Sources (Sine, Pulse, Step)
- BJT Modelle haben kein serde Modul
- Erstellung eines Integrator-Blocks zusätzlich zum Gain Block
- Erstellung eines Sum-Blocks
- tests/test_rlc_transient.cir schlägt fehl - das sollte es nicht
- Refactoring: integration.rs gehört eigentlich ins sim Modul
