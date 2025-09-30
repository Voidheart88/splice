# Extrahierte Funktionen aus dem Projekt

## lib.rs

### `run_sim`

- **Parameter:** `(sim: Simulation)`
- **Rückgabewert:** `Result<SimulationResults, SimulatorError>`
- **Datei:** `lib.rs`
- **Aufgerufene Funktionen:** run
- **Aufrufer:** Keine

---

### `run`

- **Parameter:** `()`
- **Rückgabewert:** `Result<()>`
- **Datei:** `lib.rs`
- **Aufgerufene Funktionen:** network_loop
- **Aufrufer:** run_sim

---

### `network_loop`

- **Parameter:** `(solver: Solvers)`
- **Rückgabewert:** ``
- **Datei:** `lib.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** run

---

## main.rs

### `main`

- **Parameter:** `()`
- **Rückgabewert:** `Result<()>`
- **Datei:** `main.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## csv.rs

### `output`

- **Parameter:** `(&self, results: SimulationResults)`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `csv.rs`
- **Aufgerufene Funktionen:** output_ac, output_op, output_tran, output_dc
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `csv.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** output_ac, output_tran, output_dc

---

### `output_op`

- **Parameter:** `(results: &Vec<(Variable, Numeric)>)`
- **Rückgabewert:** ``
- **Datei:** `csv.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** output

---

### `output_dc`

- **Parameter:** `(data: &Vec<Vec<(Variable, Numeric)>>, options: Vec<SimulationOption>)`
- **Rückgabewert:** ``
- **Datei:** `csv.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** output

---

### `output_tran`

- **Parameter:** `(data: &Vec<(Numeric, Vec<(Variable, Numeric)>)>)`
- **Rückgabewert:** ``
- **Datei:** `csv.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** output

---

### `output_ac`

- **Parameter:** `(data: &Vec<(Numeric, Vec<(Variable, ComplexNumeric)>)>)`
- **Rückgabewert:** ``
- **Datei:** `csv.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** output

---

## mod.rs

### `rom(`

- **Parameter:** `err: DrawingAreaErrorKind<std::io::Error>)`
- **Rückgabewert:** `elf`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `utput(`

- **Parameter:** `&self, res: SimulationResults)`
- **Rückgabewert:** `esult<(), BackendError>;`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## network.rs

### `output`

- **Parameter:** `(&self, _res: SimulationResults)`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `network.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `network.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## plot.rs

### `output`

- **Parameter:** `(&self, res: SimulationResults)`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `plot.rs`
- **Aufgerufene Funktionen:** select_output
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `(pth: String)`
- **Rückgabewert:** `Self`
- **Datei:** `plot.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** plot_ac, plot_dc, plot_op, plot_tran

---

### `select_output`

- **Parameter:** `(&self, sim: &Sim, options: Vec<SimulationOption>)`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `plot.rs`
- **Aufgerufene Funktionen:** plot_ac, plot_dc, plot_op, plot_tran
- **Aufrufer:** output

---

### `plot_dc`

- **Parameter:** `(
        &self,
        data: &Vec<Vec<(Variable, Numeric)>>,
        options: Vec<SimulationOption>,
    )`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `plot.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** select_output

---

### `plot_tran`

- **Parameter:** `(
        &self,
        data: &Vec<(Numeric, Vec<(Variable, Numeric)>)>,
    )`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `plot.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** select_output

---

### `plot_ac`

- **Parameter:** `(
        &self,
        data: &[(Numeric, Vec<(Variable, ComplexNumeric)>)],
    )`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `plot.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** select_output

---

### `plot_op`

- **Parameter:** `(&self, data: &[(Variable, Numeric)])`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `plot.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** select_output

---

## raw.rs

### `output`

- **Parameter:** `(&self, _res: SimulationResults)`
- **Rückgabewert:** `Result<(), BackendError>`
- **Datei:** `raw.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `raw.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## kicad.rs

### `simulation`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Result<Simulation, FrontendError>`
- **Datei:** `kicad.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `kicad.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `try_new_from_path`

- **Parameter:** `(_path: String)`
- **Rückgabewert:** `Result<Self, FrontendError>`
- **Datei:** `kicad.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `from`

- **Parameter:** `(error: serde_yml::Error)`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** get_variable

---

### `try_from_path`

- **Parameter:** `(pth: String)`
- **Rückgabewert:** `Result<Box<dyn Frontend>, FrontendError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `simulation`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Result<Simulation, FrontendError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_variable`

- **Parameter:** `(
    inp: &str,
    unit: Unit,
    variables: &mut Vec<Variable>,
    var_map: &mut HashMap<Arc<str>, usize>,
)`
- **Rückgabewert:** `Option<Variable>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** from
- **Aufrufer:** Keine

---

## network.rs

### `simulation`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Result<Simulation, FrontendError>`
- **Datei:** `network.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `network.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## serde.rs

### `try_new_from_path`

- **Parameter:** `(path: String, format: SerdeFormat)`
- **Rückgabewert:** `Result<Self, FrontendError>`
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** try_new_from_string
- **Aufrufer:** Keine

---

### `try_new_from_string`

- **Parameter:** `(
        circuit_string: String,
        format: SerdeFormat,
    )`
- **Rückgabewert:** `Result<Self, FrontendError>`
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** process_tran, process_ac, process_out, process, process_op, process_dc
- **Aufrufer:** try_new_from_path

---

### `process_op`

- **Parameter:** `(commands: &mut Vec<SimulationCommand>)`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** try_new_from_string

---

### `process_dc`

- **Parameter:** `(commands: &mut Vec<SimulationCommand>, serdedc: SerdeDC)`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** try_new_from_string

---

### `process_ac`

- **Parameter:** `(commands: &mut Vec<SimulationCommand>, serdeac: SerdeAC)`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** try_new_from_string

---

### `process_tran`

- **Parameter:** `(commands: &mut Vec<SimulationCommand>, serdetran:SerdeTran)`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** try_new_from_string

---

### `process_out`

- **Parameter:** `(options: &mut Vec<SimulationOption>, option: SerdeOption)`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** try_new_from_string

---

### `simulation`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Result<Simulation, FrontendError>`
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** try_new_from_string

---

## spice.rs

### `simulation`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Result<Simulation, FrontendError>`
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** process_directive, new
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `(pth: String)`
- **Rückgabewert:** `Self`
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** process_include, simulation

---

### `process_directive`

- **Parameter:** `(
        &self,
        directive: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** process_command, process_element
- **Aufrufer:** process_include, simulation

---

### `process_command`

- **Parameter:** `(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** process_tran, process_ac, process_out, process_include, process_op, process_dc
- **Aufrufer:** process_directive

---

### `process_include`

- **Parameter:** `(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** process_directive, new
- **Aufrufer:** process_command

---

### `process_op`

- **Parameter:** `(&self, commands: &mut Vec<SimulationCommand>)`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** process_command

---

### `process_dc`

- **Parameter:** `(&self, command: Pair<Rule>, commands: &mut Vec<SimulationCommand>)`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** from
- **Aufrufer:** process_command

---

### `process_ac`

- **Parameter:** `(&self, command: Pair<Rule>, commands: &mut Vec<SimulationCommand>)`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** process_command

---

### `process_tran`

- **Parameter:** `(&self, command: Pair<Rule>, commands: &mut Vec<SimulationCommand>)`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** process_command

---

### `process_out`

- **Parameter:** `(&self, option: Pair<Rule>, options: &mut Vec<SimulationOption>)`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** from
- **Aufrufer:** process_command

---

### `process_element`

- **Parameter:** `(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** process
- **Aufrufer:** process_directive

---

### `from`

- **Parameter:** `(value: pest::error::Error<Rule>)`
- **Rückgabewert:** `Self`
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** process_dc, process_out

---

### `process`

- **Parameter:** `(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** process_element

---

## pest_parser_tests.rs

### `process_minimal_vsource`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_gain`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_whitespace`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_minimal_resistor`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_resistor_case`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_vsource_case`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_resistor_with_designator`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_minimal_diode`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_op_command`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_op_command_case`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_dc_command`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_ac_command`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_tran_command`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_with_suffix1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_with_suffix2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_node`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_lines`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_empty_lines`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_comment`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_diode_regression1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_include`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_ac_vsource`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_mos0`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_out_option1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_out_option2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_out_option3`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_sine`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_sine2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `pest_parser_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice_pest_tests.rs

### `parse_resistor1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_resistor2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_vsource1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_vsource2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_vr`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_wrong1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_diode1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_diode2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_regression1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_isource1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_isource2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_with_include`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_ac`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_ac_lin`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_ac_dec`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_ac_oct`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_vsource_ac_option`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_dc_single`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_dc_double`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_mosfet`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_out1`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_minimal_circuit`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_minimal_tran`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_gain`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `parse_sine`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `spice_pest_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `test_new_diode_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_triples`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_pairs`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_pairs2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_pairs3`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## yaml_tests.rs

### `process_minimal`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `yaml_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `process_full`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `yaml_tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `fmt`

- **Parameter:** `(&self, f: &mut std::fmt::Formatter<'_>)`
- **Rückgabewert:** `std::fmt::Result`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `(name: Arc<str>, unit: Unit, index: usize)`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** name
- **Aufrufer:** name

---

### `unit`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Unit`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `from`

- **Parameter:** `(value: (Arc<str>, Unit, usize))`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_constant_triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<Triples<Numeric, 4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_constant_pairs`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<Pairs<Numeric, 2>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_time_variant_triples`

- **Parameter:** `(&self, delta_t: &Numeric)`
- **Rückgabewert:** `Option<Triples<Numeric, 4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_time_variant_pairs`

- **Parameter:** `(&self, delta_t: &Numeric)`
- **Rückgabewert:** `Option<Pairs<Numeric, 2>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_nonlinear_triples`

- **Parameter:** `(&self, x_vec: &[Numeric])`
- **Rückgabewert:** `Option<Triples<Numeric, 4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_nonlinear_pairs`

- **Parameter:** `(&self, x_vec: &[Numeric])`
- **Rückgabewert:** `Option<Pairs<Numeric, 2>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `is_nonlinear`

- **Parameter:** `(&self)`
- **Rückgabewert:** `bool`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_ac_triples`

- **Parameter:** `(&self, freq: Numeric)`
- **Rückgabewert:** `Option<Triples<ComplexNumeric, 4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_ac_pairs`

- **Parameter:** `(&self, _freq: Numeric)`
- **Rückgabewert:** `Option<Pairs<ComplexNumeric, 2>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_triple_indices`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `get_cplx_triple_indices`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## pairs.rs

### `new`

- **Parameter:** `(initial_data: &[(usize, T)])`
- **Rückgabewert:** `Self`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** len
- **Aufrufer:** Keine

---

### `data`

- **Parameter:** `(&self)`
- **Rückgabewert:** `[(usize, T); N]`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `index`

- **Parameter:** `(&self, index: usize)`
- **Rückgabewert:** `&Self::Output`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `index_mut`

- **Parameter:** `(&mut self, index: usize)`
- **Rückgabewert:** `&mut Self::Output`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `next`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Option<Self::Item>`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `size_hint`

- **Parameter:** `(&self)`
- **Rückgabewert:** `(usize, Option<usize>)`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `len`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** new

---

### `into_iter`

- **Parameter:** `(self)`
- **Rückgabewert:** `Self::IntoIter`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `is_empty`

- **Parameter:** `(&self)`
- **Rückgabewert:** `bool`
- **Datei:** `pairs.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## triples.rs

### `new`

- **Parameter:** `(initial_data: &[(usize, usize)])`
- **Rückgabewert:** `Self`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** len
- **Aufrufer:** Keine

---

### `data`

- **Parameter:** `(&self)`
- **Rückgabewert:** `[(usize, usize); N]`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `index`

- **Parameter:** `(&self, index: usize)`
- **Rückgabewert:** `&Self::Output`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `index_mut`

- **Parameter:** `(&mut self, index: usize)`
- **Rückgabewert:** `&mut Self::Output`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `next`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Option<Self::Item>`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `size_hint`

- **Parameter:** `(&self)`
- **Rückgabewert:** `(usize, Option<usize>)`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `len`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** new

---

### `into_iter`

- **Parameter:** `(self)`
- **Rückgabewert:** `Self::IntoIter`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `is_empty`

- **Parameter:** `(&self)`
- **Rückgabewert:** `bool`
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## bjt0.rs

### `default`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `(
        name: Arc<str>,
        base: Option<Variable>,
        collector: Option<Variable>,
        emitter: Option<Variable>,
        value: Option<Bjt0Options>,
    )`
- **Rückgabewert:** `Bjt0Bundle`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `triples`

- **Parameter:** `(&self, _x_vec: &[Numeric])`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `pairs`

- **Parameter:** `(&self, _x_vec: &[Numeric])`
- **Rückgabewert:** `Pairs<Numeric, 2>`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `b_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `c_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `e_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `bjt0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
    )`
- **Rückgabewert:** `CapacitorBundle`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `node0_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `node1_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `triples`

- **Parameter:** `(&self,delta_t: Option<&Numeric>)`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

### `ac_triples`

- **Parameter:** `(&self, freq: Numeric)`
- **Rückgabewert:** `Triples<ComplexNumeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

### `triple_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `test_new_capacitor_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_triples`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `make_var`

- **Parameter:** `(index: usize)`
- **Rückgabewert:** `Option<Variable>`
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** test_transient_triples_node0_none, test_new, test_triples_node0_none, test_node_indices, test_triple_idx_node1_none, test_node_indices_none, test_ac_triples_both_nodes, test_triples_both_nodes, test_triple_idx_node0_none, test_ac_triples_node0_none, test_transient_triples_zero_capacitance, test_triples_node1_none, test_transient_triples_node1_none, test_transient_triples_both_nodes, test_triple_idx_both_nodes, test_ac_triples_node1_none, test_transient_triples_large_delta_t

---

### `test_new`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_node_indices`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_node_indices_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_triples_both_nodes`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_triples_node0_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_triples_node1_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_ac_triples_both_nodes`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_ac_triples_node0_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_ac_triples_node1_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_triple_idx_both_nodes`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_triple_idx_node0_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_triple_idx_node1_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_triple_idx_both_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_transient_triples_both_nodes`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_transient_triples_node0_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_transient_triples_node1_none`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_transient_triples_zero_capacitance`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

### `test_transient_triples_large_delta_t`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** make_var
- **Aufrufer:** Keine

---

## mod.rs

### `default`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `(
        name: Arc<str>,
        anode: Option<Variable>,
        cathode: Option<Variable>,
        value: Option<DiodeOptions>,
    )`
- **Rückgabewert:** `DiodeBundle`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, triples, triple_idx

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `triples`

- **Parameter:** `(&self, x_vec: &[Numeric])`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** a_idx, new, c_idx
- **Aufrufer:** Keine

---

### `triple_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** a_idx, new, c_idx
- **Aufrufer:** Keine

---

### `pairs`

- **Parameter:** `(&self, x_vec: &[Numeric])`
- **Rückgabewert:** `Pairs<Numeric, 2>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** a_idx, new, c_idx
- **Aufrufer:** Keine

---

### `a_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, triples, triple_idx

---

### `c_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, triples, triple_idx

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `test_new_diode_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_triples`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_pairs`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_pairs2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_pairs3`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(
        name: Arc<str>,
        input: Option<Variable>,
        output: Option<Variable>,
        value: Numeric,
    )`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, ac_triples, triples, triple_idx

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** output_idx, input_idx, new
- **Aufrufer:** Keine

---

### `triple_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** output_idx, input_idx, new
- **Aufrufer:** Keine

---

### `ac_triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<ComplexNumeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** output_idx, input_idx, new
- **Aufrufer:** Keine

---

### `pairs`

- **Parameter:** `(&self, _x_vec: &[Numeric])`
- **Rückgabewert:** `Pairs<Numeric, 0>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** Keine

---

### `input_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `output_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `test_new_gain_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_triples`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_triples_missing_input`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_triples_missing_output`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_pairs`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_input_idx`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_output_idx`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_missing_input_idx`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_missing_output_idx`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
    )`
- **Rückgabewert:** `InductorBundle`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `triples`

- **Parameter:** `(&self, delta_t: Option<&Numeric>)`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `_idx(&self`

- **Parameter:** `) -> Op`
- **Rückgabewert:** `<TripleIdx<4>> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `idx(&self`

- **Parameter:** `) -> Op`
- **Rückgabewert:** `<usize> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ples(&self`

- **Parameter:** `, freq: Numeric) -> Tr`
- **Rückgabewert:** `s<ComplexNumeric, 4> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `test_new_inductor_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_triples_both_nodes`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `est_triples_node0_none(`

- **Parameter:** `)`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `est_triples_node1_none(`

- **Parameter:** `)`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `est_ac_triples_both_nodes(`

- **Parameter:** `)`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_ac_triples_node0_none()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_ac_triples_node1_none()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_triple_idx_both_nodes()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_triple_idx_node0_none()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_triple_idx_node1_none()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_triple_idx_both_none()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_transient_triples_both_nodes()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_transient_triples_node0_none()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_transient_triples_node1_none()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_transient_triples_zero_inductance()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `st_transient_triples_large_delta_t()`

- **Parameter:** `{`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
    )`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `pairs`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Pairs<Numeric, 2>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** Keine

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `create_variable`

- **Parameter:** `(name: &str, unit: Unit, idx: usize)`
- **Rückgabewert:** `Variable`
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** test_name, test_pairs_with_both_nodes, test_pairs_with_one_node, test_new_isource_bundle

---

### `test_new_isource_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_pairs_with_both_nodes`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_pairs_with_one_node`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_pairs_with_no_nodes`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mos0.rs

### `default`

- **Parameter:** `()`
- **Rückgabewert:** `Self`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `new`

- **Parameter:** `(
        name: Arc<str>,
        gate: Option<Variable>,
        drain: Option<Variable>,
        source: Option<Variable>,
        options: Option<Mos0Options>,
    )`
- **Rückgabewert:** `Mos0Bundle`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, triples, triple_idx

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `triples`

- **Parameter:** `(&self, x_vec: &[Numeric])`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** new, s_idx, g_idx, d_idx
- **Aufrufer:** Keine

---

### `triple_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** s_idx, new, d_idx
- **Aufrufer:** Keine

---

### `pairs`

- **Parameter:** `(&self, x_vec: &[Numeric])`
- **Rückgabewert:** `Pairs<Numeric, 2>`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** new, s_idx, g_idx, d_idx
- **Aufrufer:** Keine

---

### `g_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, triples

---

### `d_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, triples, triple_idx

---

### `s_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mos0.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, triples, triple_idx

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(
        name: Arc<str>,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
    )`
- **Rückgabewert:** `ResistorBundle`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `node0_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `node1_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

### `triple_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

### `ac_triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<ComplexNumeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `create_variable`

- **Parameter:** `(name: &str, unit: Unit, idx: usize)`
- **Rückgabewert:** `Variable`
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** test_name, test_triples, test_new_resistor_bundle

---

### `test_new_resistor_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_triples`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

## triples.rs

### `init_one_triple`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `init_two_triple`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `init_complex_two_triple`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `triples.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(
        name: Arc<str>,
        branch: Variable,
        node0: Option<Variable>,
        node1: Option<Variable>,
        value: Numeric,
        ac_value: Option<Numeric>,
    )`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** triple_idx, triples, pairs, ac_triples, ac_pairs

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `branch_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, ac_triples, triples, ac_pairs

---

### `node0_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `node1_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `value`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Numeric`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, branch_idx, node1_idx
- **Aufrufer:** Keine

---

### `triple_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

### `ac_triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<ComplexNumeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, branch_idx, node1_idx
- **Aufrufer:** Keine

---

### `pairs`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Pairs<Numeric, 2>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** new, branch_idx
- **Aufrufer:** Keine

---

### `ac_pairs`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Pairs<ComplexNumeric, 2>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** new, branch_idx
- **Aufrufer:** Keine

---

### `set_voltage`

- **Parameter:** `(&mut self, voltage: Numeric)`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<crate::models::Variable>,
        elements: &mut Vec<crate::models::Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `create_variable`

- **Parameter:** `(name: &str, unit: Unit, idx: usize)`
- **Rückgabewert:** `Variable`
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** test_name, test_triples, test_new_vsource_bundle, test_pairs

---

### `test_new_vsource_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_triples`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_pairs`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(
        name: Arc<str>,
        branch: Variable,
        node0: Option<Variable>,
        node1: Option<Variable>,
        dc_offset: Numeric,
        amplitude: Numeric,
        frequency: Numeric,
        phase: Numeric,
        ac_value: Option<Numeric>,
    )`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** triple_idx, triples, pairs, ac_triples, ac_pairs

---

### `name`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Arc<str>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `branch_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** pairs, ac_triples, triples, ac_pairs

---

### `node0_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `node1_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<usize>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** ac_triples, triples, triple_idx

---

### `triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<Numeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, branch_idx, node1_idx
- **Aufrufer:** Keine

---

### `triple_idx`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Option<TripleIdx<4>>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, node1_idx
- **Aufrufer:** Keine

---

### `ac_triples`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Triples<ComplexNumeric, 4>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** node0_idx, new, branch_idx, node1_idx
- **Aufrufer:** Keine

---

### `pairs`

- **Parameter:** `(&self, time: Option<&Numeric>)`
- **Rückgabewert:** `Pairs<Numeric, 2>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** new, branch_idx
- **Aufrufer:** Keine

---

### `ac_pairs`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Pairs<ComplexNumeric, 2>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** new, branch_idx
- **Aufrufer:** Keine

---

### `set_voltage`

- **Parameter:** `(&mut self, voltage: Numeric)`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## serde.rs

### `process`

- **Parameter:** `(
        &self,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `serde.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## spice.rs

### `process`

- **Parameter:** `(
        element: pest::iterators::Pair<crate::frontends::spice::Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut std::collections::HashMap<std::sync::Arc<str>, usize>,
    )`
- **Rückgabewert:** ``
- **Datei:** `spice.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `create_variable`

- **Parameter:** `(name: &str, unit: Unit, idx: usize)`
- **Rückgabewert:** `Variable`
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** test_name, test_new_vsource_sin_bundle, test_triples

---

### `test_new_vsource_sin_bundle`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_name`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

### `test_triples`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** create_variable
- **Aufrufer:** Keine

---

## commands.rs

### `try_from`

- **Parameter:** `(value: &str)`
- **Rückgabewert:** `Result<Self, FrontendError>`
- **Datei:** `commands.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `from`

- **Parameter:** `(error: SolverError)`
- **Rückgabewert:** `Self`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `run`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<SimulationResults, SimulatorError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** init_solver, execute_command
- **Aufrufer:** Keine

---

### `init_solver`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** run

---

### `execute_command`

- **Parameter:** `(&mut self, comm: &SimulationCommand)`
- **Rückgabewert:** `Result<Sim, SimulatorError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** run_op
- **Aufrufer:** run

---

### `run_op`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<Sim, SimulatorError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** execute_command

---

### `s_nonlinear_elements(&`

- **Parameter:** `self) -`
- **Rückgabewert:** `ol {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `d_var_name(&`

- **Parameter:** `self, solution: Vec<Numeric>) -`
- **Rückgabewert:** `c<(Variable, Numeric)> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `d_complex_var_name(`

- **Parameter:** `&self,
        solution: Vec<Complex<Numeric>>,
    ) -`
- **Rückgabewert:** `c<(Variable, Complex<Numeric>)> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `n_tran(&`

- **Parameter:** `mut self, tstep: &Numeric, tstop: &Numeric) -`
- **Rückgabewert:** `sult<Sim, SimulatorError> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `n_ac(`

- **Parameter:** `&mut self,
        fstart: &Numeric,
        fend: &Numeric,
        steps: &usize,
        ac_option: &ACMode,
    ) -`
- **Rückgabewert:** `sult<Sim, SimulatorError> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `n_dc(`

- **Parameter:** `&mut self,
        srcnam: &Arc<str>,
        vstart: &Numeric,
        vstop: &Numeric,
        vstep: &Numeric,
        _optional: &Option<(Arc<str>, Numeric, Numeric, Numeric)>,
    ) -`
- **Rückgabewert:** `sult<Sim, SimulatorError> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** _vsource_with_name(e
- **Aufrufer:** Keine

---

### `nd_op(&`

- **Parameter:** `mut self) -`
- **Rückgabewert:** `sult<Vec<(Variable, Numeric)>, SimulatorError> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_constant_a_mat(&`

- **Parameter:** `mut self) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_constant_b_vec(&`

- **Parameter:** `mut self) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_time_variant_a_mat(&`

- **Parameter:** `mut self, delta_t:&Numeric) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_time_variant_b_vec(&`

- **Parameter:** `mut self, delta_t:&Numeric) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_nonlinear_a_mat(&`

- **Parameter:** `mut self, x_vec: &[Numeric]) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_nonlinear_b_vec(&`

- **Parameter:** `mut self, x_vec: &[Numeric]) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_ac_a_mat(&`

- **Parameter:** `mut self, freq: Numeric) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `ild_ac_b_vec(&`

- **Parameter:** `mut self, freq: Numeric) {`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `nerate_initial_guess(&`

- **Parameter:** `self) -`
- **Rückgabewert:** `c<Numeric> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `s_converged(&`

- **Parameter:** `self, x_old: &[Numeric], x_new: &[Numeric], tolerance: Numeric) -`
- **Rückgabewert:** `ol {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `t(&`

- **Parameter:** `self, f: &mut fmt::Formatter<'_>) -`
- **Rückgabewert:** `t::Result {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `om(s`

- **Parameter:** `im: Simulation) -`
- **Rückgabewert:** `lf {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `_vsource_with_name(e`

- **Parameter:** `lement: &Element, srcnam: &Arc<str>) -`
- **Rückgabewert:** `ol {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** n_dc(

---

### `t_vsource_value(e`

- **Parameter:** `lement: &mut Element) -`
- **Rückgabewert:** `tion<Numeric> {`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## tests.rs

### `init_sim_rsparse`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `init_sim_faer`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `init_sim_nalgebra`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `run_sim_tran`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `tests.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## faer.rs

### `new`

- **Parameter:** `(vars: usize)`
- **Rückgabewert:** `Result<Self, SolverError>`
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_b`

- **Parameter:** `(&mut self, b_vec: &(usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_b`

- **Parameter:** `(&mut self, b_vec: &(usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<Numeric>, SolverError>`
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** solve
- **Aufrufer:** solve, solve_cplx

---

### `solve_cplx`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<ComplexNumeric>, SolverError>`
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** solve
- **Aufrufer:** Keine

---

### `init`

- **Parameter:** `(&mut self, _a_matrix: Vec<(usize, usize)>, _cplx_a_matrix: Vec<(usize, usize)>)`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `rows`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `cols`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `b_vec_len`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## faer_sparse.rs

### `new`

- **Parameter:** `(vars: usize)`
- **Rückgabewert:** `Result<Self, SolverError>`
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** insert_cplx_a, new, insert_a

---

### `insert_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** Keine

---

### `insert_b`

- **Parameter:** `(&mut self, b_vec: &(usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** Keine

---

### `insert_cplx_b`

- **Parameter:** `(&mut self, b_vec: &(usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<Numeric>, SolverError>`
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** solve
- **Aufrufer:** solve, solve_cplx

---

### `solve_cplx`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<ComplexNumeric>, SolverError>`
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** solve
- **Aufrufer:** Keine

---

### `init`

- **Parameter:** `(&mut self, _a_matrix: Vec<(usize, usize)>, _cplx_a_matrix: Vec<(usize, usize)>)`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `from`

- **Parameter:** `(value: LuError)`
- **Rückgabewert:** `Self`
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `rows`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `cols`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `b_vec_len`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `new`

- **Parameter:** `(vars: usize)`
- **Rückgabewert:** `Result<Self, SolverError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `init`

- **Parameter:** `(&mut self, a_matrix: Vec<(usize, usize)>, cplx_a_matrix: Vec<(usize, usize)>)`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_a`

- **Parameter:** `(&mut self, a_trpl: &(usize, usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_b`

- **Parameter:** `(&mut self, b_pair: &(usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_a`

- **Parameter:** `(&mut self, a_trpl: &(usize, usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_b`

- **Parameter:** `(&mut self, b_pair: &(usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<Numeric>, SolverError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_cplx`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<ComplexNumeric>, SolverError>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## nalgebra.rs

### `new`

- **Parameter:** `(vars: usize)`
- **Rückgabewert:** `Result<NalgebraSolver, SolverError>`
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** solve, solve_cplx

---

### `insert_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_b`

- **Parameter:** `(&mut self, b_vec: &(usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_b`

- **Parameter:** `(&mut self, b_vec: &(usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<Numeric>, SolverError>`
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** solve, new
- **Aufrufer:** solve, solve_cplx

---

### `solve_cplx`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<ComplexNumeric>, SolverError>`
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** solve, new
- **Aufrufer:** Keine

---

### `init`

- **Parameter:** `(&mut self, a_matrix: Vec<(usize, usize)>, cplx_a_matrix: Vec<(usize, usize)>)`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `rows`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `cols`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `b_vec_len`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `a_mat`

- **Parameter:** `(&self)`
- **Rückgabewert:** `&DMatrix<Numeric>`
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## rsparse.rs

### `new`

- **Parameter:** `(vars: usize)`
- **Rückgabewert:** `Result<Self, SolverError>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** init, new, update_from_hashmap

---

### `insert_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_b`

- **Parameter:** `(&mut self, b_vec: &(usize, Numeric))`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_a`

- **Parameter:** `(&mut self, a_mat: &(usize, usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_cplx_b`

- **Parameter:** `(&mut self, b_vec: &(usize, ComplexNumeric))`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<Numeric>, SolverError>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** update_from_hashmap, ipvec
- **Aufrufer:** Keine

---

### `solve_cplx`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `Result<&Vec<ComplexNumeric>, SolverError>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** real_vec_to_complex_vec
- **Aufrufer:** Keine

---

### `init`

- **Parameter:** `(&mut self, a_matrix: Vec<(usize, usize)>, cplx_a_matrix: Vec<(usize, usize)>)`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** Keine

---

### `real_vec_to_complex_vec`

- **Parameter:** `(&self)`
- **Rückgabewert:** `Vec<ComplexNumeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** solve_cplx

---

### `update_from_hashmap`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** new
- **Aufrufer:** solve

---

### `ipvec`

- **Parameter:** `(n: usize, p: &Option<Vec<isize>>, b: &[Numeric], x: &mut [Numeric])`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** solve

---

### `b_vec_len`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `cplx_b_vec_len`

- **Parameter:** `(&self)`
- **Rückgabewert:** `usize`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `a_mat`

- **Parameter:** `(&self)`
- **Rückgabewert:** `&HashMap<(usize, usize), Numeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `a_mat_mut`

- **Parameter:** `(&mut self)`
- **Rückgabewert:** `&mut HashMap<(usize, usize), Numeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `b_vec`

- **Parameter:** `(&self)`
- **Rückgabewert:** `&Vec<Numeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `x_vec`

- **Parameter:** `(&self)`
- **Rückgabewert:** `&Vec<Numeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `cplx_a_mat`

- **Parameter:** `(&self)`
- **Rückgabewert:** `&HashMap<(usize, usize), ComplexNumeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `cplx_b_vec`

- **Parameter:** `(&self)`
- **Rückgabewert:** `&Vec<Numeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `sprs`

- **Parameter:** `(&self)`
- **Rückgabewert:** `&Sprs<Numeric>`
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `print_matrix_from_trpl`

- **Parameter:** `(triple: Trpl<f64>)`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## faer.rs

### `init_solver`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small_2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_no_solution`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_infinite_solutions_dependent`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_complex_small_wo_imag`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve3`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `faer_solve`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve_test`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `newton_raphson_test`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## faer_sparse.rs

### `init_solver`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small_2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_no_solution`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_infinite_solutions_dependent`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_complex_small_wo_imag`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve3`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `faer_solve`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `faer_sparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## mod.rs

### `generate_solvable_system`

- **Parameter:** `(
    n: usize,
    density: Numeric,
)`
- **Rückgabewert:** `(Vec<Vec<Numeric>>, Vec<Numeric>, Vec<Numeric>)`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `calculate_f`

- **Parameter:** `(x: &[Numeric])`
- **Rückgabewert:** `Vec<Numeric>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `calculate_jacobian`

- **Parameter:** `(x: &[Numeric])`
- **Rückgabewert:** `HashMap<(usize, usize), Numeric>`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `norm`

- **Parameter:** `(vec: &[Numeric])`
- **Rückgabewert:** `Numeric`
- **Datei:** `mod.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## nalgebra.rs

### `init_solver`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small_2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_no_solution`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_infinite_solutions_dependent`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_complex_small_wo_imag`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_add_a_mat`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `nalgebra_solve`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `newton_raphson_test`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `nalgebra.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

## rsparse.rs

### `init_solver`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `init_solver_cplx`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small_2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_no_solution`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_infinite_solutions_dependent`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_complex_small_wo_imag`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_add_a`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `solve_small_electrical`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve2`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve3`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_update_from_hashmap_basic`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_update_from_hashmap_empty`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `test_update_from_hashmap_single_element`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `rsparse_solve`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `rsparse_update_sprs`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `rsparse_update_b_vec`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `insert_after_solve_test`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---

### `newton_raphson_test`

- **Parameter:** `()`
- **Rückgabewert:** ``
- **Datei:** `rsparse.rs`
- **Aufgerufene Funktionen:** Keine
- **Aufrufer:** Keine

---


# Source-Code von main.rs

```python
use miette::Result;
use splice::run;

fn main() -> Result<()> {
    run()
}

```
