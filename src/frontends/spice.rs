use std::{collections::HashSet, fs::File, io::Read, path::Path, sync::Arc};

use itertools::Itertools;

use crate::{sim::commands::SimulationCommand, FrontendError, Simulation};

use super::{
    CapacitorBundle, DiodeBundle, Element, Frontend, ISourceBundle, InductorBundle, ResistorBundle,
    Unit, VSourceBundle, Variable,
};

pub struct SpiceFrontend {
    pth: String,
}

impl Frontend for SpiceFrontend {
    /// Reads the circuit description from a file, processes it, and constructs a `Simulation` object.
    ///
    /// # Errors
    ///
    /// Returns a `FrontendError` if there are any issues during file reading, preprocessing,
    /// or element processing.
    ///
    /// # Returns
    ///
    /// Returns a `Simulation` object representing the circuit if successful.
    ///
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        // Read the circuit file content
        let mut circuit_string = String::new();
        File::open(&self.pth)?.read_to_string(&mut circuit_string)?;

        // Preprocess the circuit description
        let current_path = Path::new(&self.pth).parent().unwrap();
        let preprocessed = self.preprocess(circuit_string, current_path)?;

        let mut variables = Vec::new();
        let mut commands = Vec::new();
        let mut elements = Vec::new();

        for line in preprocessed.lines() {
            match line.trim().chars().next() {
                Some('.') => commands.push(self.process_command(line)?),
                Some(ch) => {
                    let element = match ch {
                        'r' => self.process_resistor(line, &mut variables)?,
                        'c' => self.process_capacitor(line, &mut variables)?,
                        'l' => self.process_inductor(line, &mut variables)?,
                        'd' => self.process_diode(line, &mut variables)?,
                        'v' => self.process_vsource(line, &mut variables)?,
                        'i' => self.process_isource(line, &mut variables)?,
                        _ => continue,
                    };
                    elements.push(element);
                }
                None => continue,
            };
        }

        // Ensure all element names are unique
        let mut names = HashSet::new();
        for ele in &elements {
            let ele_name = ele.name();
            if !names.insert(ele_name.clone()) {
                return Err(FrontendError::ElementDouble(ele_name.to_string()));
            }
        }

        Ok(Simulation {
            variables,
            elements,
            commands,
        })
    }
}

impl SpiceFrontend {
    pub fn new(pth: String) -> Self {
        SpiceFrontend { pth }
    }

    /// Preprocesses the input string.
    ///
    /// This method removes unnecessary characters, concatenates lines that start with '+',
    /// and processes `.include` directives to include external files.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to be preprocessed.
    ///
    /// # Returns
    ///
    /// The preprocessed string.
    ///
    fn preprocess(&self, input: String, current_path: &Path) -> Result<String, FrontendError> {
        let mut result = String::new();
        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            let line = line.trim();

            // Check for include directive
            if line.starts_with(".include") {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                if tokens.len() == 2 {
                    let included_path = tokens[1];
                    let full_included_path = current_path.join(included_path);
                    let included_content = self.read_include(&full_included_path)?;
                    let preprocessed_include =
                        self.preprocess(included_content, &full_included_path.parent().unwrap())?;
                    result.push_str("\n");
                    result.push_str(&preprocessed_include);
                } else {
                    return Err(FrontendError::ParseError("Invalid .include syntax".into()));
                }
            } else if line.starts_with('+') {
                result.push_str(" ");
                result.push_str(&line[1..].to_lowercase());
            } else if !line.is_empty() {
                result.push_str("\n");
                result.push_str(&line.to_lowercase());
            }
        }

        if result.starts_with('\n') {
            result.remove(0);
        }

        Ok(result)
    }

    fn read_include(&self, path: &Path) -> Result<String, FrontendError> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Processes a line of input to determine the simulation command.
    ///
    /// The function checks the input string for specific keywords (".op", ".dc", ".ac", ".tran")
    /// and returns the corresponding `SimulationCommand`. If no keyword is found, it defaults to ".op".
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to the input string.
    ///
    /// # Returns
    ///
    /// Returns a `SimulationCommand` enum representing the simulation command.
    ///
    /// Possible Commands:
    /// .op
    /// .dc srcnam vstart vstop vincr [src2 start2 stop2 incr2]
    ///
    /// Example:
    /// .dc v1 0 1 0.1
    fn process_command(&self, input: &str) -> Result<SimulationCommand, FrontendError> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens[0].contains(".op") {
            Ok(SimulationCommand::Op)
        } else if input.contains(".dc") {
            if tokens.len() < 5 {
                return Err(FrontendError::ParseCommandError(
                    "Not enough tokens to parse".into(),
                ));
            }
            let srcnam = Arc::new(tokens[1].into());
            let vstart = tokens[2].parse::<f64>()?;
            let vstop = tokens[3].parse::<f64>()?;
            let vincr = tokens[4].parse::<f64>()?;
            Ok(SimulationCommand::Dc(srcnam, vstart, vstop, vincr, None))
        } else if input.contains(".ac") {
            Ok(SimulationCommand::Ac)
        } else if input.contains(".tran") {
            Ok(SimulationCommand::Tran)
        } else {
            Ok(SimulationCommand::Op)
        }
    }

    fn process_vsource(
        &self,
        input: &str,
        variables: &mut Vec<Variable>,
    ) -> Result<Element, FrontendError> {
        let token: Vec<&str> = input.split_whitespace().collect();
        let name = Arc::new(token[0].into());

        let branch = Self::add_variable(&format!("{name}#branch"), Unit::Ampere, variables);

        let node0 = Self::add_variable(token[1], Unit::Volt, variables);
        let node1 = Self::add_variable(token[2], Unit::Volt, variables);

        let value = match token[3].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                return Err(FrontendError::ParseError(format!(
                    "Could not parse {input}"
                )))
            }
        };

        Ok(Element::VSource(VSourceBundle::new(
            name,
            branch.unwrap(),
            node0,
            node1,
            value,
        )))
    }

    fn process_resistor(
        &self,
        input: &str,
        variables: &mut Vec<Variable>,
    ) -> Result<Element, FrontendError> {
        let token = input.split(" ").collect_vec();
        let name = Arc::new(token[0].into());
        let node0 = Self::add_variable(token[1], Unit::Volt, variables);
        let node1 = Self::add_variable(token[2], Unit::Volt, variables);
        let value = match token[3].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                return Err(FrontendError::ParseError(format!(
                    "Could not parse {input}"
                )))
            }
        };

        Ok(Element::Resistor(ResistorBundle::new(
            name, node0, node1, value,
        )))
    }

    fn process_inductor(
        &self,
        input: &str,
        variables: &mut Vec<Variable>,
    ) -> Result<Element, FrontendError> {
        let token = input.split(" ").collect_vec();
        let name = Arc::new(token[0].into());
        let node0 = Self::add_variable(token[1], Unit::Volt, variables);
        let node1 = Self::add_variable(token[2], Unit::Volt, variables);
        let value = match token[3].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                return Err(FrontendError::ParseError(format!(
                    "Could not parse {input}"
                )))
            }
        };

        Ok(Element::Inductor(InductorBundle::new(
            name, node0, node1, value,
        )))
    }

    fn process_capacitor(
        &self,
        input: &str,
        variables: &mut Vec<Variable>,
    ) -> Result<Element, FrontendError> {
        let token = input.split(" ").collect_vec();
        let name = Arc::new(token[0].into());
        let node0 = Self::add_variable(token[1], Unit::Volt, variables);
        let node1 = Self::add_variable(token[2], Unit::Volt, variables);
        let value = match token[3].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                return Err(FrontendError::ParseError(format!(
                    "Could not parse {input}"
                )))
            }
        };

        Ok(Element::Capacitor(CapacitorBundle::new(
            name, node0, node1, value,
        )))
    }

    fn process_diode(
        &self,
        input: &str,
        variables: &mut Vec<Variable>,
    ) -> Result<Element, FrontendError> {
        let token = input.split(" ").collect_vec();
        let name = Arc::new(token[0].into());
        let node0 = Self::add_variable(token[1], Unit::Volt, variables);
        let node1 = Self::add_variable(token[2], Unit::Volt, variables);

        Ok(Element::Diode(DiodeBundle::new(name, node0, node1, None)))
    }

    fn process_isource(
        &self,
        input: &str,
        variables: &mut Vec<Variable>,
    ) -> Result<Element, FrontendError> {
        let token = input.split_whitespace().collect_vec();
        let name = Arc::new(token[0].into());
        let node0 = Self::add_variable(token[1], Unit::Volt, variables);
        let node1 = Self::add_variable(token[2], Unit::Volt, variables);
        let value = match token[3].parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                return Err(FrontendError::ParseError(format!(
                    "Could not parse {input}"
                )))
            }
        };

        Ok(Element::ISource(ISourceBundle::new(
            name, node0, node1, value,
        )))
    }

    fn add_variable(inp: &str, unit: Unit, variables: &mut Vec<Variable>) -> Option<Variable> {
        if inp == "0" {
            return None;
        }

        let inp_rc = Arc::new(inp.to_string()); // Umwandlung von &str in Rc<String>

        for variable in variables.iter() {
            if variable.name() == inp_rc {
                return Some(variable.clone());
            }
        }

        let new_variable = Variable::new(inp_rc.clone(), unit, variables.len());
        variables.push(new_variable.clone());

        Some(new_variable)
    }
}
