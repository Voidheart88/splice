# Splice Refactoring Todo Liste

Diese Datei trackt alle identifizierten Refactoring-Aufgaben für das Splice-Projekt.

## 🔴 Kritische Probleme (Hohe Priorität)

### 1. Fehlerbehandlung und Sicherheit
- **Status**: ⏳ Geplant
- **Beschreibung**: Ersetze `unwrap()` und `expect()` durch ordentliche Fehlerbehandlung
- **Betroffene Dateien**: `src/models/*/spice.rs`, `src/models/*/mod.rs`
- **Priorität**: ⭐⭐⭐⭐⭐
- **Geschätzter Aufwand**: 4-8 Stunden

### 2. Unnötiges Klonen
- **Status**: ⏳ Geplant  
- **Beschreibung**: Reduziere häufiges Klonen von Datenstrukturen
- **Betroffene Methoden**: `add_var_name()`, `add_complex_var_name()`, `find_op()`
- **Priorität**: ⭐⭐⭐⭐
- **Geschätzter Aufwand**: 3-5 Stunden

### 3. TODO/FIXME Kommentare
- **Status**: ⏳ Geplant
- **Beschreibung**: Implementiere fehlende Funktionen oder entferne unfertige Features
- **Betroffene Dateien**: `src/models/bjt/bjt0.rs`, `src/sim/op.rs`
- **Priorität**: ⭐⭐⭐⭐
- **Geschätzter Aufwand**: 6-10 Stunden

## 🟡 Architekturverbesserungen (Mittlere Priorität)

### 4. Solver-Architektur
- **Status**: ⏳ Geplant
- **Beschreibung**: Extrahiere gemeinsame Funktionalität in Traits/Helper
- **Betroffene Dateien**: `src/solver/*.rs`
- **Priorität**: ⭐⭐⭐
- **Geschätzter Aufwand**: 8-12 Stunden

### 5. Element-Trait-System
- **Status**: ⏳ Geplant
- **Beschreibung**: Führe Polymorphismus für Element-Typen ein
- **Betroffene Datei**: `src/models/mod.rs`
- **Priorität**: ⭐⭐⭐
- **Geschätzter Aufwand**: 5-8 Stunden

### 6. Simulation State Management
- **Status**: ⏳ Geplant
- **Beschreibung**: Optimiertes Reset-Verhalten implementieren
- **Betroffene Datei**: `src/sim/mod.rs`
- **Priorität**: ⭐⭐⭐
- **Geschätzter Aufwand**: 4-6 Stunden

## 🟢 Codequalität (Mittlere Priorität)

### 7. Dokumentation
- **Status**: ⏳ Geplant
- **Beschreibung**: Doc-Kommentare für alle öffentlichen Funktionen
- **Priorität**: ⭐⭐⭐
- **Geschätzter Aufwand**: 4-8 Stunden

### 8. Konsistente Namensgebung
- **Status**: ⏳ Geplant
- **Beschreibung**: Vereinheitliche Namenskonventionen
- **Priorität**: ⭐⭐
- **Geschätzter Aufwand**: 2-4 Stunden

### 9. Magic Numbers
- **Status**: ⏳ Geplant
- **Beschreibung**: Ersetze harte Konstanten durch benannte Werte
- **Betroffene Datei**: `src/sim/mod.rs`
- **Priorität**: ⭐⭐
- **Geschätzter Aufwand**: 1-2 Stunden

## 🔵 Performance-Optimierungen (Niedrige Priorität)

### 10. Datenstruktur-Optimierung
- **Status**: ⏳ Geplant
- **Beschreibung**: Optimierung von Pairs/Triples Strukturen
- **Betroffene Dateien**: `src/models/pairs.rs`, `src/models/triples.rs`
- **Priorität**: ⭐⭐
- **Geschätzter Aufwand**: 3-5 Stunden

### 11. Parallelisierung
- **Status**: ⏳ Geplant
- **Beschreibung**: Erweiterte Parallelisierung untersuchen
- **Priorität**: ⭐
- **Geschätzter Aufwand**: 10-15 Stunden

## 🧪 Testverbesserungen

### 12. Testabdeckung
- **Status**: ⏳ Geplant
- **Beschreibung**: Tests für ungetestete Modelle (z.B. BJT)
- **Priorität**: ⭐⭐⭐
- **Geschätzter Aufwand**: 6-10 Stunden

### 13. Testdaten-Refactoring
- **Status**: ⏳ Geplant
- **Beschreibung**: Extrahiere duplizierte Testdaten
- **Betroffene Datei**: `src/sim/tests.rs`
- **Priorität**: ⭐⭐
- **Geschätzter Aufwand**: 3-5 Stunden

## 📊 Fortschrittsverfolgung

### Abgeschlossen: 0/13 Aufgaben
### In Arbeit: 0/13 Aufgaben  
### Geplant: 13/13 Aufgaben

## 🎯 Nächste Schritte

1. Beginne mit Aufgabe #1 (Fehlerbehandlung) - kritisch für Stabilität
2. Parallel kann Aufgabe #7 (Dokumentation) bearbeitet werden
3. Aufgabe #3 (TODO/FIXME) sollte vor einem Release abgeschlossen sein

## 📅 Zeitplan (Vorschlag)

- **Woche 1**: Aufgaben 1, 7, 9 (Fehlerbehandlung + Dokumentation)
- **Woche 2**: Aufgaben 2, 5, 6 (Performance + Architektur)
- **Woche 3**: Aufgaben 3, 4, 8 (Funktionalität + Codequalität)
- **Woche 4**: Aufgaben 10, 12, 13 (Optimierung + Tests)

## 🔧 Technische Hinweise

- Alle Änderungen sollten mit `cargo test` validiert werden
- Performance-kritische Änderungen sollten mit Benchmarks verifiziert werden
- Dokumentation sollte mit `cargo doc` generiert und überprüft werden