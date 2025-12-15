# Netzwerkmodus Benchmark-Dokumentation

## Übersicht

Diese Dokumentation beschreibt die implementierten Benchmarks für den Netzwerkmodus von Splice, die in `benches/network.rs` zu finden sind.

## Benchmark-Kategorien

### 1. MessagePack Serialisierungs-Benchmarks

**Zweck**: Messung der Serialisierungsgeschwindigkeit für verschiedene Schaltungsgrößen

**Abgedeckte Szenarien**:
- Kleine Schaltungen (3 Elemente: R, C, V)
- Mittlere Schaltungen (10-100 Elemente in Serie)
- Große Schaltungen (5x5 bis 10x10 Widerstandsgitter)

**Metriken**:
- Serialisierungszeit pro Schaltung
- Ausgabepuffergröße (Payload-Größe)
- Skalierungsverhalten mit Schaltungsgröße

### 2. MessagePack Deserialisierungs-Benchmarks

**Zweck**: Messung der Deserialisierungsgeschwindigkeit für verschiedene Schaltungsgrößen

**Abgedeckte Szenarien**:
- Kleine Schaltungen (3 Elemente)
- Mittlere Schaltungen (50 Elemente)
- Große Schaltungen (10x10 Gitter)

**Metriken**:
- Deserialisierungszeit pro Schaltung
- Durchsatz (Schaltungen pro Sekunde)

### 3. Roundtrip-Benchmarks

**Zweck**: Messung der kombinierten Serialisierungs-/Deserialisierungsleistung

**Abgedeckte Szenarien**:
- Kleine, mittlere und große Schaltungen
- Vollständiger Zyklus: Objekt → Bytes → Objekt

**Metriken**:
- Gesamtzeit für Roundtrip
- Effektiver Durchsatz

### 4. Simulationstyp-Benchmarks

**Zweck**: Vergleich der Serialisierungsperformance für verschiedene Simulationstypen

**Abgedeckte Simulationstypen**:
- **OP (Arbeitspunktanalyse)**: Einfache Struktur
- **DC (DC-Sweep)**: Parameter für Spannungssweep
- **AC (AC-Analyse)**: Frequenzbereichsdefinition
- **Tran (Transientenanalyse)**: Zeitschritt- und Endzeitparameter

**Metriken**:
- Relative Serialisierungszeit
- Payload-Größenvergleich

### 5. Ergebnis-Serialisierungs-Benchmarks

**Zweck**: Messung der Serialisierungsperformance für Simulationsergebnisse

**Abgedeckte Ergebnisgrößen**:
- Kleine Ergebnisse (1 Variable)
- Mittlere Ergebnisse (50 Variablen)
- Große Ergebnisse (100x50 DC-Sweep-Ergebnisse)

**Metriken**:
- Serialisierungszeit für Ergebnisse
- Speichereffizienz

### 6. Payload-Skalierungs-Benchmarks

**Zweck**: Analyse des Skalierungsverhaltens mit zunehmender Schaltungsgröße

**Abgedeckte Größen**:
- 10, 50, 100, 200, 500 Elemente
- Separate Messung von Serialisierung und Deserialisierung

**Metriken**:
- Zeitkomplexität (O(n) Analyse)
- Speicherbedarfsskalierung

## Testschaltungen

### 1. Einfache RC-Schaltung
```
V1 (0 → n1): 5V
R1 (n1 → 0): 1kΩ
C1 (n1 → 0): 1µF
```

### 2. Mittlere Serien-Schaltung
```
V1 (0 → n1): 5V
R1 (n1 → n2): 1kΩ
R2 (n2 → n3): 1kΩ
...
Rn (nX → nX+1): 1kΩ
C1 (nX+1 → 0): 1µF
```

### 3. Großes Widerstandsgitter
```
V1 (0 → n1): 5V
Horizontal: R_0_0 (n1 → n2), R_0_1 (n2 → n3), ...
Vertical: Rvert_0 (n1 → nX), Rvert_1 (n2 → nX+1), ...
```

## Benchmark-Ergebnis-Interpretation

### Erwartete Ergebnisse

1. **Serialisierungsperformance**:
   - Lineare Skalierung mit Schaltungsgröße
   - MessagePack sollte 50-70% kleiner sein als JSON
   - Serialisierung sollte <1ms für kleine, <10ms für mittlere, <100ms für große Schaltungen

2. **Deserialisierungsperformance**:
   - Etwas langsamer als Serialisierung (typisch 1.2-1.5x)
   - Sollte ähnlich skalieren wie Serialisierung

3. **Simulationstyp-Vergleich**:
   - OP: Schnellste (einfache Struktur)
   - DC: Mittlere Komplexität
   - AC/Tran: Komplexeste (mehr Parameter)

### Performance-Ziele

| Schaltungsgröße | Serialisierung | Deserialisierung | Roundtrip |
|----------------|----------------|------------------|-----------|
| Klein (3 Elemente) | < 100µs | < 150µs | < 250µs |
| Mittel (50 Elemente) | < 1ms | < 1.5ms | < 2.5ms |
| Groß (100+ Elemente) | < 10ms | < 15ms | < 25ms |

## Benchmark-Ausführung

### Einzelne Benchmark-Gruppe ausführen
```bash
cargo bench -- network_benches
```

### Spezifischen Benchmark ausführen
```bash
cargo bench -- bench_msgpack_serialization
```

### Alle Benchmarks ausführen
```bash
cargo bench
```

## Performance-Optimierungsmöglichkeiten

### 1. Serialisierungsoptimierungen
- **Buffer-Wiederverwendung**: Pool von Byte-Buffern
- **Inkrementelle Serialisierung**: Streaming für große Schaltungen
- **Kompression**: Optional für sehr große Schaltungen

### 2. Deserialisierungsoptimierungen
- **Direkte Deserialisierung**: In vorhandene Strukturen
- **Parallelisierung**: Für sehr große Ergebnisse
- **Lazy Deserialisierung**: On-demand-Laden von Daten

### 3. Protokolloptimierungen
- **Binäre Optimierung**: Custom Binary Format statt MessagePack
- **Delta-Kodierung**: Für ähnliche Schaltungen
- **Schema-Evolution**: Versionierung für zukünftige Kompatibilität

## Vergleich mit Alternativen

### MessagePack vs JSON
- **Größe**: MessagePack ist 50-70% kleiner
- **Geschwindigkeit**: MessagePack ist 2-5x schneller
- **Typensicherheit**: MessagePack behält Typinformationen

### MessagePack vs Protocol Buffers
- **Flexibilität**: MessagePack ist schemalos
- **Einfachheit**: MessagePack hat geringeren Overhead
- **Performance**: Vergleichbar für kleine bis mittlere Payloads

### MessagePack vs Custom Binary
- **Entwicklungsgeschwindigkeit**: MessagePack ist einfacher zu implementieren
- **Wartbarkeit**: MessagePack hat bessere Tool-Unterstützung
- **Performance**: Custom Binary könnte 10-30% schneller sein

## Zukunftspläne für Benchmarks

### Kurzfristig (1-2 Wochen)
- [x] Grundlegende Serialisierungs-Benchmarks
- [x] Deserialisierungs-Benchmarks
- [x] Roundtrip-Benchmarks
- [x] Skalierungsanalysen

### Mittelfristig (1 Monat)
- [ ] Netzwerk-Latenz-Benchmarks (TCP-Overhead)
- [ ] Gleichzeitige Verbindungen (Stress-Tests)
- [ ] Langlauf-Benchmarks (Stabilität)

### Langfristig (3+ Monate)
- [ ] Vergleich mit anderen SPICE-Implementierungen
- [ ] Cloud-Skalierungs-Benchmarks
- [ ] Geografische Latenz-Tests

## Best Practices für Netzwerk-Benchmarking

### 1. Konsistente Testumgebung
- Gleiche Hardware für alle Tests
- Keine anderen laufenden Prozesse
- Netzwerkisolation für Latenztests

### 2. Statistische Signifikanz
- Mehrere Durchläufe pro Test
- Warm-up-Phase vor Messung
- Ausreißer-Eliminierung

### 3. Realistische Szenarien
- Echte Schaltungsdaten verwenden
- Variierende Netzwerkbedingungen simulieren
- Fehlerfälle testen

### 4. Dokumentation
- Klare Benchmark-Beschreibungen
- Versionskontrolle der Testdaten
- Reproduzierbare Ergebnisse

## Fehlerbehandlung in Benchmarks

### Häufige Probleme
1. **Timeouts**: Bei sehr großen Schaltungen
2. **Speicherlimit**: Bei extrem großen Gitter-Schaltungen
3. **Netzwerkprobleme**: Bei TCP-Benchmarks

### Lösungsstrategien
- **Chunking**: Große Schaltungen in Teile aufteilen
- **Streaming**: Inkrementelle Verarbeitung
- **Timeouts**: Angemessene Limits setzen

## Zusammenfassung

Die implementierten Netzwerk-Benchmarks bieten eine umfassende Abdeckung der kritischen Performance-Aspekte:

✅ **Serialisierung/Deserialisierung**: Grundlegende MessagePack-Performance
✅ **Skalierung**: Verhalten mit zunehmender Schaltungsgröße
✅ **Simulationstypen**: Unterschiedliche Komplexitätsstufen
✅ **Ergebnisverarbeitung**: Output-Serialisierung

🚧 **Geplant**: Netzwerk-Latenz, Gleichzeitigkeit, Langzeitstabilität

Diese Benchmarks ermöglichen:
- Performance-Regessionstests
- Optimierungsvalidierung
- Kapazitätsplanung für Produktionsumgebungen
- Vergleich mit alternativen Implementierungen
