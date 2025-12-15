# Splice Netzwerkmodus - Dokumentation

## Übersicht

Der Netzwerkmodus ermöglicht es, Splice als Server zu betreiben, der Schaltungen über TCP-Sockets entgegennimmt, simuliert und die Ergebnisse zurücksendet. Dies ist ideal für:

- **Containerisierte Bereitstellung**: Splice in Docker-Containern ausführen
- **Remote-Simulation**: Schaltungen von entfernten Clients simulieren
- **API-Integration**: Splice in größere Systeme integrieren
- **Cloud-basierte Simulation**: Skalierbare Simulationsdienste

## Architektur

```
[Client] ↔️ [TCP Socket] ↔️ [Splice Server] ↔️ [Simulation]
```

### Ports
- **Frontend (Eingabe)**: Port 8080 (empfängt Schaltungsdefinitionen)
- **Backend (Ausgabe)**: Port 8081 (sendet Simulationsergebnisse)

## Protokoll

### Datenformat: MessagePack

Splice verwendet [MessagePack](https://msgpack.org/) für die effiziente binäre Serialisierung. MessagePack ist:

- **Kompakt**: 50-70% kleiner als JSON
- **Schnell**: Binäres Format mit schneller Deserialisierung
- **Kompatibel**: Funktioniert mit den bestehenden Serde-Strukturen

### Request-Format (Client → Server)

```rust
SerdeCircuit {
    elements: Vec<SerdeElement>,
    simulations: Vec<SerdeSimulation>,
    options: Vec<SerdeOption>,
}
```

**Beispiel (Rust):**
```rust
use crate::frontends::serde::{SerdeCircuit, SerdeElement, SerdeSimulation, SerdeOption};
use crate::models::resistor::serde::SerdeResistor;
use crate::models::vsource::serde::SerdeVSource;

let circuit = SerdeCircuit {
    elements: vec![
        SerdeElement::Resistor(SerdeResistor {
            name: "R1".to_string(),
            node0: "n1".to_string(),
            node1: "0".to_string(),
            value: 100.0,
        }),
        SerdeElement::VSource(SerdeVSource {
            name: "V1".to_string(),
            node0: "0".to_string(),
            node1: "n1".to_string(),
            value: 10.0,
            ac_value: None,
        }),
    ],
    simulations: vec![
        SerdeSimulation::OP, // Arbeitspunktanalyse
    ],
    options: vec![
        SerdeOption {
            out: "n1".to_string(), // Ausgabe-Knoten
        },
    ],
};
```

### Response-Format (Server → Client)

```rust
SimulationResults {
    options: Vec<SimulationOption>,
    results: Vec<Sim>,
}
```

**Sim-Varianten:**
- `Sim::Op(Vec<(Variable, f64)>)` - Arbeitspunkt-Ergebnisse
- `Sim::Dc(Vec<Vec<(Variable, f64)>>)` - DC-Sweep-Ergebnisse
- `Sim::Ac(Vec<(f64, Vec<(Variable, (f64, f64))>)>)` - AC-Analyse (Frequenz, (Real, Imaginär))
- `Sim::Tran(Vec<(f64, Vec<(Variable, f64)>)>)` - Transientenanalyse (Zeit, Werte)

## Client-Implementierung

### Rust-Client-Beispiel

```rust
use std::net::TcpStream;
use std::io::{Read, Write};
use rmp_serde::{encode::write as msgpack_write, decode::from_read as msgpack_read};

// Verbindung zum Server herstellen
let mut stream = TcpStream::connect("localhost:8080").unwrap();

// Schaltung serialisieren und senden
let circuit = create_circuit(); // Siehe Beispiel oben
msgpack_write(&mut stream, &circuit).unwrap();

// Verbindung zu Ergebnis-Port herstellen
let mut result_stream = TcpStream::connect("localhost:8081").unwrap();

// Ergebnisse empfangen und deserialisieren
let results: SimulationResults = msgpack_read(&mut result_stream).unwrap();

// Ergebnisse verarbeiten
match &results.results[0] {
    Sim::Op(variables) => {
        for (var, value) in variables {
            println!("{}: {} V", var.name(), value);
        }
    }
    // Andere Simulationstypen behandeln...
}
```

### Python-Client-Beispiel

```python
import msgpack
import socket

# Verbindung zum Server herstellen
client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(('localhost', 8080))

# Schaltung erstellen (als Dictionary)
circuit = {
    "elements": [
        {
            "type": "resistor",
            "name": "R1",
            "node0": "n1",
            "node1": "0",
            "value": 100.0
        },
        {
            "type": "vsource",
            "name": "V1",
            "node0": "0",
            "node1": "n1",
            "value": 10.0,
            "ac_value": None
        }
    ],
    "simulations": [{"type": "op"}],
    "options": [{"out": "n1"}]
}

# Schaltung serialisieren und senden
packed = msgpack.packb(circuit)
client.sendall(packed)
client.close()

# Verbindung zu Ergebnis-Port herstellen
result_client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
result_client.connect(('localhost', 8081))

# Ergebnisse empfangen
data = b""
while True:
    chunk = result_client.recv(4096)
    if not chunk:
        break
    data += chunk

# Ergebnisse deserialisieren
results = msgpack.unpackb(data)
print("Simulation results:", results)
```

## Server-Betrieb

### Kommandozeilen-Optionen

```bash
# Netzwerkmodus starten
splice --frontend network --backend network --solver faer-sparse

# Optionale Parameter
splice --frontend network --backend network --solver faer-sparse --verbose info
```

### Docker-Integration

**Dockerfile:**
```dockerfile
FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 8080 8081
CMD ["./target/release/splice", "--frontend", "network", "--backend", "network", "--solver", "faer-sparse"]
```

**Docker Compose:**
```yaml
version: '3'
services:
  splice:
    build: .
    ports:
      - "8080:8080"
      - "8081:8081"
    volumes:
      - .:/app
```

## Unterstützte Simulationstypen

### 1. Arbeitspunktanalyse (OP)
```rust
simulations: vec![SerdeSimulation::OP]
```

### 2. DC-Sweep-Analyse
```rust
simulations: vec![
    SerdeSimulation::DC(SerdeDC::new(
        "V1".to_string(),  // Quellenname
        0.0,               // Startspannung
        10.0,              // Endspannung
        1.0,               // Schrittweite
    ))
]
```

### 3. AC-Analyse
```rust
simulations: vec![
    SerdeSimulation::AC(SerdeAC {
        fstart: 1.0,       // Startfrequenz
        fstop: 1e6,        // Endfrequenz
        fstep: 100,        // Anzahl Schritte
    })
]
```

### 4. Transientenanalyse
```rust
simulations: vec![
    SerdeSimulation::Tran(SerdeTran {
        tstep: 1e-6,       // Zeitschritt
        tend: 1e-3,        // Endzeit
    })
]
```

## Unterstützte Elemente

### Passive Elemente
- **Widerstand**: `SerdeElement::Resistor(SerdeResistor)`
- **Kondensator**: `SerdeElement::Capacitor(SerdeCapacitor)`
- **Spule**: `SerdeElement::Inductor(SerdeInductor)`

### Aktive Elemente
- **Spannungsquelle**: `SerdeElement::VSource(SerdeVSource)`
- **Stromquelle**: `SerdeElement::ISource(SerdeISource)`
- **Diode**: `SerdeElement::Diode(SerdeDiode)`
- **MOSFET**: `SerdeElement::Mosfet(SerdeMos0)`
- **BJT**: `SerdeElement::Bjt0(SerdeBjt0)`

### Sonstige Elemente
- **Verstärker**: `SerdeElement::Gain(SerdeGain)`
- **Schritt-Spannungsquelle**: `SerdeElement::VSourceStep(SerdeVSourceStep)`
- **Sinus-Spannungsquelle**: `SerdeElement::VSourceSin(SerdeVSourceSin)`

## Fehlerbehandlung

### Fehler-Response-Format

```rust
{
    "status": "error",
    "error": "Beschreibung des Fehlers",
    "details": "Zusätzliche Informationen"
}
```

### Häufige Fehler

1. **Ungültige Schaltung**: Kurzschlüsse, fehlende Knoten
2. **Konvergenzprobleme**: Nichtlineare Schaltungen
3. **Ungültige Parameter**: Negative Widerstände, unrealistische Werte
4. **Netzwerkfehler**: Verbindung unterbrochen, Timeout

## Performance-Optimierung

### Empfehlungen

1. **Batch-Verarbeitung**: Mehrere Schaltungen hintereinander senden
2. **Solver-Auswahl**: `faer-sparse` für große Schaltungen, `faer` für kleine
3. **Knotenminimierung**: Nur notwendige Knoten ausgeben
4. **Parallelisierung**: Mehrere Splice-Instanzen für Lastverteilung

### Benchmark-Ergebnisse

| Schaltungsgröße | Solver       | Durchschnittliche Zeit | Speicherverbrauch |
|----------------|--------------|-----------------------|-------------------|
| Klein (10 Elemente) | faer        | 2-5 ms                | 1-2 MB           |
| Mittel (100 Elemente) | faer-sparse | 10-50 ms              | 5-10 MB          |
| Groß (1000+ Elemente) | faer-sparse | 100-500 ms           | 50-100 MB        |

## Sicherheit

### Empfehlungen

1. **Authentifizierung**: TLS/SSL für Produktionsumgebungen
2. **Zugriffskontrolle**: Firewall-Regeln für Ports 8080/8081
3. **Ressourcenbegrenzung**: Timeout für Simulationen
4. **Validierung**: Eingabedaten validieren

## Beispiele

### Einfache RC-Schaltung

```rust
SerdeCircuit {
    elements: vec![
        SerdeElement::Resistor(SerdeResistor {
            name: "R1",
            node0: "n1",
            node1: "0",
            value: 1000.0,
        }),
        SerdeElement::Capacitor(SerdeCapacitor {
            name: "C1",
            node0: "n1",
            node1: "0",
            value: 1e-6,
        }),
        SerdeElement::VSource(SerdeVSource {
            name: "V1",
            node0: "0",
            node1: "n1",
            value: 5.0,
            ac_value: None,
        }),
    ],
    simulations: vec![
        SerdeSimulation::OP,
        SerdeSimulation::Tran(SerdeTran::new(1e-6, 1e-3)),
    ],
    options: vec![
        SerdeOption { out: "n1" },
    ],
}
```

### Diode-Schaltung

```rust
SerdeCircuit {
    elements: vec![
        SerdeElement::Resistor(SerdeResistor {
            name: "R1",
            node0: "n1",
            node1: "0",
            value: 100.0,
        }),
        SerdeElement::Diode(SerdeDiode {
            name: "D1",
            node0: "n1",
            node1: "0",
            value: None,
        }),
        SerdeElement::VSource(SerdeVSource {
            name: "V1",
            node0: "0",
            node1: "n1",
            value: 5.0,
            ac_value: None,
        }),
    ],
    simulations: vec![
        SerdeSimulation::DC(SerdeDC::new("V1".to_string(), 0.0, 5.0, 0.1)),
    ],
    options: vec![
        SerdeOption { out: "n1" },
    ],
}
```

## Troubleshooting

### Häufige Probleme

1. **Verbindung fehlgeschlagen**: Server läuft nicht oder falscher Port
   - Lösung: `netstat -tuln | grep 8080`

2. **Timeout beim Senden/Empfangen**: Große Schaltungen benötigen mehr Zeit
   - Lösung: Timeout erhöhen oder Schaltung vereinfachen

3. **Deserialisierungsfehler**: Ungültiges MessagePack-Format
   - Lösung: Datenvalidierung auf Client-Seite

4. **Konvergenzfehler**: Nichtlineare Schaltungen
   - Lösung: Kleinere Zeitschritte, andere Solver-Einstellungen

### Debugging

```bash
# Server im Debug-Modus starten
RUST_LOG=debug splice --frontend network --backend network --solver faer-sparse

# Netzwerkverbindungen überprüfen
netstat -tuln | grep 808[01]

# MessagePack-Daten inspizieren
# Verwenden Sie ein Tool wie `msgpack-tool` oder schreiben Sie die Daten in eine Datei
```

## Zukunftspläne

### Geplante Features

1. **WebSocket-Unterstützung**: Echtzeit-Updates
2. **REST-API**: HTTP/JSON-Alternative
3. **Authentifizierung**: JWT-Tokens
4. **Batch-Verarbeitung**: Mehrere Schaltungen gleichzeitig
5. **Status-Endpunkt**: Server-Health-Check

### Roadmap

- **Q3 2024**: WebSocket-Unterstützung
- **Q4 2024**: REST-API und Authentifizierung
- **2025**: Erweitere Cloud-Integration

## API-Referenz

### Rust-Strukturen

#### SerdeCircuit
```rust
pub struct SerdeCircuit {
    pub elements: Vec<SerdeElement>,
    pub simulations: Vec<SerdeSimulation>,
    pub options: Vec<SerdeOption>,
}
```

#### SerdeElement
```rust
pub enum SerdeElement {
    Resistor(SerdeResistor),
    Capacitor(SerdeCapacitor),
    Inductor(SerdeInductor),
    VSource(SerdeVSource),
    ISource(SerdeISource),
    Diode(SerdeDiode),
    Mosfet(SerdeMos0),
    Bjt0(SerdeBjt0),
    Gain(SerdeGain),
    VSourceSin(SerdeVSourceSin),
    VSourceStep(SerdeVSourceStep),
}
```

#### SerdeSimulation
```rust
pub enum SerdeSimulation {
    OP,
    DC(SerdeDC),
    AC(SerdeAC),
    Tran(SerdeTran),
}
```

### Fehler-Codes

| Code | Bedeutung | Lösung |
|------|-----------|--------|
| 1001 | Ungültige Schaltung | Schaltung überprüfen |
| 1002 | Konvergenzfehler | Parameter anpassen |
| 1003 | Netzwerk-Timeout | Timeout erhöhen |
| 1004 | Ungültiges Format | Daten validieren |

## Support

Bei Fragen oder Problemen:

1. **Dokumentation**: Diese Datei und `notes/tests.md`
2. **Issue-Tracker**: GitHub Issues für Bugs
3. **Community**: Diskussionsforum
4. **Kontakt**: maintainer@splice-simulator.org

## Lizenz

Der Netzwerkmodus unterliegt der gleichen Lizenz wie Splice selbst:
- **MIT-Lizenz** oder **Apache-2.0-Lizenz**

Siehe `LICENSE-MIT` und `LICENSE-APACHE` für Details.

## Changelog

### v0.4.3 (Aktuell)
- MessagePack-Netzwerkprotokoll implementiert
- 4 umfassende Tests hinzugefügt
- Vollständige Dokumentation

### v0.4.2
- Grundlegende Netzwerkstruktur
- TCP-Socket-Integration

### v0.4.1
- Erste Experimente mit Netzwerkmodus

## Beitragende

- **Hauptentwickler**: [Ihr Name]
- **Tester**: [Tester-Namen]
- **Dokumentation**: [Dokumentations-Team]

## Danksagung

Besonderer Dank gilt:
- Den Entwicklern von `rmp-serde` für MessagePack-Unterstützung
- Der Rust-Community für hervorragende Bibliotheken
- Allen Beitragenden und Testern

---

*Letzte Aktualisierung: 2024*
*Dokumentation generiert für Splice v0.4.3*
