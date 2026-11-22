= Programmierhandbuch (Deutsch)

Dieses Kapitel beschreibt die internen Strukturen, das API und Hinweise für Entwickler, die Splice erweitern wollen.

== Code‑Organisation
- `src/` – Kernmodule (Frontend, Simulation, Solver, Backend)
- `src/models/` – Bauteile
- `src/guesser/` – Parameter‑Schätzung
- `tests/` – Unit‑ und Integrationstests

== Erweiterungshinweise
- Neue Frontends können über das `Frontend`‑Trait implementiert werden.
- Solver‑Backends sollten das `Solver`‑Trait implementieren.
- Dokumentiere neue Modelle in `src/models/<name>/` und füge Tests hinzu.

