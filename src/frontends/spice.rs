use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use log::trace;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use super::{
    CCCSBundle, CCVSBundle, CapacitorBundle, CoupledInductorsBundle, DiodeBundle, Element,
    GainBundle, ISourceBundle, InductorBundle, Mos0Bundle, ResistorBundle, VCCSBundle, VCVSBundle,
    Variable,
};
use crate::frontends::{Frontend, FrontendError, Simulation};
use crate::models::vsource_sine::VSourceSinBundle;
use crate::models::vsource_step::VSourceStepBundle;
use crate::models::VSourceBundle;
use crate::sim::commands::{ACMode, SimulationCommand};
use crate::sim::options::SimulationOption;
use crate::spot::*;

#[derive(Parser, Debug)]
#[grammar = "frontends/pest/spice.pest"]
pub struct SpiceParser;

#[derive(Debug)]
pub struct SpiceFrontend {
    pth: String,
}

impl SpiceFrontend {
    /// Create a Simulation directly from SPICE code string for benchmarking
    /// This avoids the overhead of creating temporary files
    pub fn parse_spice_code(spice_code: &str) -> Result<Simulation, FrontendError> {
        let mut commands = Vec::new();
        let mut options = Vec::new();
        let mut elements = Vec::new();
        let mut variables = Vec::new();
        let mut var_map = HashMap::new();

        let parse_result = SpiceParser::parse(Rule::SPICE, spice_code)?
            .next()
            .ok_or(FrontendError::ParseError("unexpected file end".into()))?;
        for pair in parse_result.into_inner() {
            if pair.as_rule() == Rule::DIRECTIVE {
                // Create a dummy SpiceFrontend instance to call process_directive
                let dummy_frontend = SpiceFrontend { pth: String::new() };
                dummy_frontend.process_directive(
                    pair,
                    &mut commands,
                    &mut options,
                    &mut elements,
                    &mut variables,
                    &mut var_map,
                )?;
            }
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
            commands,
            options,
            elements,
            variables,
        })
    }
}

impl Frontend for SpiceFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        let mut circuit_string = String::new();
        File::open(&self.pth)?.read_to_string(&mut circuit_string)?;
        trace!("Parse Schematic!");
        let parse_result = SpiceParser::parse(Rule::SPICE, &circuit_string)?.next();
        let parse_result = match parse_result {
            Some(res) => res,
            None => return Err(FrontendError::ParseError("unexpected file end".into())),
        };

        let mut commands = Vec::new();
        let mut options = Vec::new();
        let mut elements = Vec::new();
        let mut variables = Vec::new();
        let mut var_map = HashMap::new();

        for pair in parse_result.into_inner() {
            if pair.as_rule() == Rule::DIRECTIVE {
                self.process_directive(
                    pair,
                    &mut commands,
                    &mut options,
                    &mut elements,
                    &mut variables,
                    &mut var_map,
                )?
            }
        }

        trace!("Check Schematic!");
        // Ensure all element names are unique
        let mut names = HashSet::new();
        for ele in &elements {
            let ele_name = ele.name();
            if !names.insert(ele_name.clone()) {
                return Err(FrontendError::ElementDouble(ele_name.to_string()));
            }
        }

        Ok(Simulation {
            commands,
            options,
            elements,
            variables,
        })
    }
}

impl SpiceFrontend {
    pub fn new(pth: String) -> Self {
        SpiceFrontend { pth }
    }

    fn process_directive(
        &self,
        directive: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        for inner in directive.into_inner() {
            match inner.as_rule() {
                Rule::ELEMENT => self.process_element(inner, variables, elements, var_map)?,
                Rule::COMMAND => {
                    self.process_command(inner, commands, options, elements, variables, var_map)?
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn process_command(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        let command = command
            .into_inner()
            .nth(0)
            .ok_or_else(|| FrontendError::ParseError("Missing command in directive".into()))?;

        match command.as_rule() {
            Rule::CMD_OP => self.process_op(commands),
            Rule::CMD_DC => self.process_dc(command, commands)?,
            Rule::CMD_AC => self.process_ac(command, commands)?,
            Rule::CMD_TRAN => self.process_tran(command, commands)?,
            Rule::CMD_INCLUDE => {
                self.process_include(command, commands, options, elements, variables, var_map)?
            }
            Rule::CMD_OUT => self.process_out(command, options)?,
            _ => {}
        }
        Ok(())
    }

    fn process_include(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        let current_path = Path::new(&self.pth)
            .parent()
            .ok_or_else(|| FrontendError::IoError("Invalid file path".into()))?;

        let path = command.as_str().split(" ").nth(1).ok_or_else(|| {
            FrontendError::ParseError(format!(
                "Missing path in .include directive: '{}'",
                command.as_str()
            ))
        })?;

        let full_path = current_path.join(path);
        let mut circuit_string = String::new();

        let mut file = File::open(&full_path).map_err(|e| {
            FrontendError::IoError(format!(
                "Failed to open included file '{}': {}",
                full_path.display(),
                e
            ))
        })?;

        file.read_to_string(&mut circuit_string).map_err(|e| {
            FrontendError::IoError(format!(
                "Failed to read included file '{}': {}",
                full_path.display(),
                e
            ))
        })?;

        trace!("Parse Schematic!");
        let parse_result = SpiceParser::parse(Rule::SPICE, &circuit_string)
            .map_err(|e| {
                FrontendError::PestError(format!(
                    "Parse error in included file '{}': {}",
                    full_path.display(),
                    e
                ))
            })?
            .next()
            .ok_or_else(|| {
                FrontendError::ParseError(format!(
                    "Empty parse result for included file '{}'",
                    full_path.display()
                ))
            })?;

        for pair in parse_result.into_inner() {
            if pair.as_rule() == Rule::DIRECTIVE {
                self.process_directive(pair, commands, options, elements, variables, var_map)?
            }
        }
        Ok(())
    }

    fn process_op(&self, commands: &mut Vec<SimulationCommand>) {
        commands.push(SimulationCommand::Op)
    }

    fn process_dc(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
    ) -> Result<(), FrontendError> {
        let mut inner = command.into_inner();

        let source = inner
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing source in .dc command".into()))?
            .as_str();

        let vstart = inner
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing vstart in .dc command".into()))?
            .as_str()
            .parse::<Numeric>()
            .map_err(|_| FrontendError::ParseError("Invalid vstart value".into()))?;

        let vend = inner
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing vend in .dc command".into()))?
            .as_str()
            .parse::<Numeric>()
            .map_err(|_| FrontendError::ParseError("Invalid vend value".into()))?;

        let vstep = inner
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing vstep in .dc command".into()))?
            .as_str()
            .parse::<Numeric>()
            .map_err(|_| FrontendError::ParseError("Invalid vstep value".into()))?;

        let src2 = inner.next();
        match src2 {
            None => {
                commands.push(SimulationCommand::Dc(
                    Arc::from(source),
                    vstart,
                    vend,
                    vstep,
                    None,
                ));
            }
            Some(src) => {
                let mut src2 = src.into_inner();

                let source2 = src2
                    .next()
                    .ok_or_else(|| {
                        FrontendError::ParseError("Missing second source in .dc command".into())
                    })?
                    .as_str();

                let vstart2 = src2
                    .next()
                    .ok_or_else(|| {
                        FrontendError::ParseError("Missing vstart2 in .dc command".into())
                    })?
                    .as_str()
                    .parse::<Numeric>()
                    .map_err(|_| FrontendError::ParseError("Invalid vstart2 value".into()))?;

                let vend2 = src2
                    .next()
                    .ok_or_else(|| {
                        FrontendError::ParseError("Missing vend2 in .dc command".into())
                    })?
                    .as_str()
                    .parse::<Numeric>()
                    .map_err(|_| FrontendError::ParseError("Invalid vend2 value".into()))?;

                let vstep2 = src2
                    .next()
                    .ok_or_else(|| {
                        FrontendError::ParseError("Missing vstep2 in .dc command".into())
                    })?
                    .as_str()
                    .parse::<Numeric>()
                    .map_err(|_| FrontendError::ParseError("Invalid vstep2 value".into()))?;

                commands.push(SimulationCommand::Dc(
                    Arc::from(source),
                    vstart,
                    vend,
                    vstep,
                    Some((Arc::from(source2), vstart2, vend2, vstep2)),
                ));
            }
        }
        Ok(())
    }

    fn process_ac(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
    ) -> Result<(), FrontendError> {
        let mut values = command.into_inner();

        let fstart = values
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing fstart in .ac command".into()))?
            .as_str()
            .parse::<f64>()
            .map_err(|_| FrontendError::ParseError("Invalid fstart value".into()))?;

        let fend = values
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing fend in .ac command".into()))?
            .as_str()
            .parse::<f64>()
            .map_err(|_| FrontendError::ParseError("Invalid fend value".into()))?;

        let step = values
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing step in .ac command".into()))?
            .as_str()
            .parse::<usize>()
            .map_err(|_| {
                FrontendError::ParseError("Invalid step value - must be an integer".into())
            })?;

        let mode = match values.next() {
            Some(mode) => mode
                .as_str()
                .try_into()
                .map_err(|_| FrontendError::ParseError("Invalid AC mode".into()))?,
            None => ACMode::default(),
        };

        commands.push(SimulationCommand::Ac(fstart, fend, step, mode));
        Ok(())
    }

    fn process_tran(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
    ) -> Result<(), FrontendError> {
        let mut inner = command.into_inner();

        let tstep = inner
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing tstep in .tran command".into()))?
            .as_str()
            .parse::<Numeric>()
            .map_err(|_| FrontendError::ParseError("Invalid tstep value".into()))?;

        let tstop = inner
            .next()
            .ok_or_else(|| FrontendError::ParseError("Missing tstop in .tran command".into()))?
            .as_str()
            .parse::<Numeric>()
            .map_err(|_| FrontendError::ParseError("Invalid tstop value".into()))?;

        commands.push(SimulationCommand::Tran(tstep, tstop));
        Ok(())
    }

    fn process_out(
        &self,
        option: Pair<Rule>,
        options: &mut Vec<SimulationOption>,
    ) -> Result<(), FrontendError> {
        let nodes: Vec<Arc<str>> = option
            .into_inner()
            .map(|inner| Arc::from(inner.as_str()))
            .collect();

        if nodes.is_empty() {
            return Err(FrontendError::ParseError(
                "Empty .out directive - no nodes specified".into(),
            ));
        }

        options.push(SimulationOption::Out(nodes));
        Ok(())
    }

    fn process_element(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        let element = element
            .into_inner()
            .nth(0)
            .ok_or_else(|| FrontendError::ParseError("Missing element in directive".into()))?;

        match element.as_rule() {
            Rule::ELE_VSOURCE_SIN => {
                VSourceSinBundle::process(element, variables, elements, var_map)?
            }
            Rule::ELE_VSOURCE_STEP => {
                VSourceStepBundle::process(element, variables, elements, var_map)?
            }
            Rule::ELE_VSOURCE => VSourceBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_ISOURCE => ISourceBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_RESISTOR => ResistorBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_CAPACITOR => CapacitorBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_INDUCTOR => InductorBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_COUPLED_INDUCTORS => {
                CoupledInductorsBundle::process(element, variables, elements, var_map)?
            }
            Rule::ELE_DIODE => DiodeBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_MOSFET => Mos0Bundle::process(element, variables, elements, var_map)?,
            Rule::ELE_GAIN => GainBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_VCVS => VCVSBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_VCCS => VCCSBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_CCCS => CCCSBundle::process(element, variables, elements, var_map)?,
            Rule::ELE_CCVS => CCVSBundle::process(element, variables, elements, var_map)?,
            _ => {}
        }
        Ok(())
    }
}

impl From<pest::error::Error<Rule>> for FrontendError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        FrontendError::PestError(format!("{value}"))
    }
}

pub(crate) trait ProcessSpiceElement {
    fn process(
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError>;
}

impl ProcessSpiceElement for CoupledInductorsBundle {
    fn process(
        element: Pair<Rule>,
        _variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        _var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Result<(), FrontendError> {
        let mut inner = element.into_inner();
        let name = inner.next().unwrap().as_str();
        let inductor1_name = inner.next().unwrap().as_str();
        let inductor2_name = inner.next().unwrap().as_str();
        let coupling_factor = inner.next().unwrap().as_str().parse::<Numeric>()?;

        // Create the coupled inductors bundle
        let coupled_inductors = CoupledInductorsBundle::new(
            Arc::from(name),
            Arc::from(inductor1_name),
            Arc::from(inductor2_name),
            coupling_factor,
        );

        elements.push(Element::CoupledInductors(coupled_inductors));
        Ok(())
    }
}
