# Splice Data Flow Analyzer

A tool to analyze and visualize how data is transformed through the Splice circuit simulator codebase.

## What It Does

The analyzer tracks:
1. **Data Structures** - All structs and enums in the codebase
2. **Transformations** - How data is converted (From/Into traits, conversion functions)
3. **Field Mappings** - How structures reference each other through fields
4. **Data Flow Paths** - Complete paths from input to output

## Usage

```bash
# Run from the scripts directory
cd scripts
python data_flow_analyzer.py

# Run with JSON export
python data_flow_analyzer.py --json

# Specify custom source directory
python data_flow_analyzer.py --src ../src

# Output to different file
python data_flow_analyzer.py --output my_report.html
```

## Output Files

### `data_flow_report.html`
Interactive HTML report with:
- **Statistics Dashboard** - Overview of structures and transformations
- **Main Pipeline Visualization** - High-level data flow diagram
- **Mermaid Diagram** - Interactive data flow graph
- **Structure Table** - All discovered data structures with fields
- **Transformation Table** - All conversion functions found
- **Filtering** - Search and filter by name, kind, module

### `data_flow_analysis.json`
Machine-readable JSON export for further processing or integration with other tools.

## Example: Main Simulation Pipeline

The analyzer identifies the key stages:

```
Input (Frontend)          Core                    Solver               Output (Backend)
┌─────────────────┐      ┌──────────────┐       ┌───────────┐        ┌──────────────┐
│ SerdeCircuit    │ ──▶  │ Simulation   │  ──▶  │ Simulator │  ──▶   │ Simulation-  │
│ SerdeElement    │      │ Element      │       │           │        │ Results      │
│ SerdeFrontend   │      │ Variable     │       │           │        │ CsvBackend   │
└─────────────────┘      └──────────────┘       └───────────┘        │ PlotBackend  │
                                                                    └──────────────┘
```

## Data Flow Example

The analyzer traces how data transforms:

1. **Parsing**: `SerdeCircuit` (YAML/JSON) → `Simulation`
2. **Setup**: `Simulation` → `Simulator<FaerSolver>`
3. **Execution**: `Simulator` builds matrices and solves
4. **Output**: Solution → `SimulationResults` → CSV/Plot/Network

## Architecture Insights

### Key Structures Found
- **128 data structures** across the codebase
- **21 transformations** (From/Into implementations, conversion functions)
- **147 Rust files** analyzed

### Module Organization
- `frontends::serde` - Input data structures
- `models` - Circuit element representations
- `sim` - Simulation logic and state
- `solver` - Numerical backends
- `backends` - Output formats

## Future Enhancements

Potential improvements:
- [ ] Track method calls to see data flow through functions
- [ ] Analyze lifetime and ownership patterns
- [ ] Generate sequence diagrams for specific simulations
- [ ] Interactive exploration of type relationships
- [ ] Export to Graphviz for advanced visualization
- [ ] Integration with rust-analyzer for more precise type information

## Technical Details

The analyzer uses:
- **Regex-based parsing** - Fast, works without dependencies
- **Optional tree-sitter** - More precise AST parsing (if available)
- **BFS pathfinding** - Discovers transformation chains
- **Type inference** - Extracts base types from complex expressions

## Troubleshooting

**No structures found?**
- Make sure you're running from the `scripts` directory
- Or use `--src` to specify the path to the `src` directory

**Missing transformations?**
- The analyzer looks for `impl From<X> for Y` patterns
- And functions named `xxx_to_yyy` or `convert_xxx`
- Some conversions may be manual and not follow these patterns
