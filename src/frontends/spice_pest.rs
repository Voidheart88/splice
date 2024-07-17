use std::{fs::File, io::Read, sync::Arc};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    sim::commands::{ACMode, SimulationCommand},
    Frontend, FrontendError, Simulation,
};

use super::{Element, Variable};

#[derive(Parser)]
#[grammar = "frontends/pest/spice.pest"]
pub struct SpiceParser;

pub struct SpicePestFrontend {
    pth: String,
}

impl Frontend for SpicePestFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        let mut circuit_string = String::new();
        File::open(&self.pth)?.read_to_string(&mut circuit_string)?;

        let parse_result = SpiceParser::parse(Rule::SPICE, &circuit_string)?.next();
        let parse_result = match parse_result {
            Some(res) => res,
            None => return Err(FrontendError::ParseError("unexpected file end".into())),
        };

        let mut variables = Vec::new();
        let mut elements = Vec::new();
        let mut commands = Vec::new();

        for pair in parse_result.into_inner() {
            match pair.as_rule() {
                Rule::DIRECTIVE => {
                    self.process_directive(pair, &mut variables, &mut elements, &mut commands)
                }
                _ => {}
            }
        }

        Ok(Simulation {
            variables,
            elements,
            commands,
        })
    }
}

impl SpicePestFrontend {
    pub fn new(pth: String) -> Self {
        SpicePestFrontend { pth }
    }

    fn process_directive(
        &self,
        directive: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        commands: &mut Vec<SimulationCommand>,
    ) {
        for inner in directive.into_inner() {
            match inner.as_rule() {
                Rule::ELE => self.process_element(inner, variables, elements),
                Rule::COMMAND => self.process_command(inner, commands),
                _ => {}
            }
        }
    }

    fn process_command(&self, command: Pair<Rule>, commands: &mut Vec<SimulationCommand>) {
        let command = command.into_inner().nth(0).unwrap();
        match command.as_rule() {
            Rule::CMD_OP => self.process_op(commands),
            Rule::CMD_DC => self.process_dc(command, commands),
            Rule::CMD_AC => self.process_ac(command, commands),
            Rule::CMD_TRAN => self.process_tran(command, commands),
            _ => {}
        }
    }

    fn process_op(&self, commands: &mut Vec<SimulationCommand>) {
        commands.push(SimulationCommand::Op)
    }

    fn process_dc(&self, command: Pair<Rule>, commands: &mut Vec<SimulationCommand>) {
        let mut values = command.into_inner();

        let source = Arc::from("");
        let vstart = values.next().unwrap().as_str().parse().unwrap();
        let vend = values.next().unwrap().as_str().parse().unwrap();
        let vstep = values.next().unwrap().as_str().parse().unwrap();

        commands.push(SimulationCommand::Dc(source, vstart, vend, vstep, None))
    }

    fn process_ac(&self, command: Pair<Rule>, commands: &mut Vec<SimulationCommand>) {
        let mut values = command.into_inner();
        let fstart = values.next().unwrap().as_str().parse().unwrap();
        let fend = values.next().unwrap().as_str().parse().unwrap();
        let step = values.next().unwrap().as_str().parse().unwrap();
        let mode = match values.next() {
            Some(mode) => mode.as_str().try_into().unwrap(),
            None => ACMode::default(),
        };
        commands.push(SimulationCommand::Ac(fstart, fend, step, mode))
    }

    fn process_tran(&self, _: Pair<Rule>, commands: &mut Vec<SimulationCommand>) {
        commands.push(SimulationCommand::Tran)
    }

    fn process_element(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
    ) {
        let element = element.into_inner().nth(0).unwrap();
        match element.as_rule() {
            Rule::ELE_VSOURCE => self.process_vsource(element, variables, elements),
            Rule::ELE_RESISTOR => self.process_resistor(element, variables, elements),
            Rule::ELE_CAPACITOR => self.process_capacitor(element, variables, elements),
            Rule::ELE_INDUCTOR => self.process_inductor(element, variables, elements),
            Rule::ELE_DIODE => self.process_diode(element, variables, elements),
            _ => {}
        }
    }

    fn process_vsource(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
    ) {
        todo!()
    }

    fn process_resistor(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
    ) {
        todo!()
    }

    fn process_capacitor(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
    ) {
        todo!()
    }

    fn process_inductor(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
    ) {
        todo!()
    }

    fn process_diode(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
    ) {
        todo!()
    }
}

impl From<pest::error::Error<Rule>> for FrontendError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        FrontendError::PestError(format!("{}", value))
    }
}
