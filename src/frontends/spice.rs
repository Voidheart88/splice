use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use log::trace;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::frontends::{Frontend, FrontendError, Simulation};
use crate::models::VSourceBundle;
use crate::sim::commands::{ACMode, SimulationCommand};
use crate::sim::options::SimulationOption;
use crate::spot::*;

use super::{
    CapacitorBundle, DiodeBundle, Element, ISourceBundle, InductorBundle, Mos0Bundle,
    ResistorBundle, Unit, Variable,
};

#[derive(Parser)]
#[grammar = "frontends/pest/spice.pest"]
pub struct SpiceParser;

pub struct SpiceFrontend {
    pth: String,
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
            match pair.as_rule() {
                Rule::DIRECTIVE => self.process_directive(
                    pair,
                    &mut commands,
                    &mut options,
                    &mut elements,
                    &mut variables,
                    &mut var_map,
                ),
                _ => {}
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
    ) {
        for inner in directive.into_inner() {
            match inner.as_rule() {
                Rule::ELE => self.process_element(inner, variables, elements, var_map),
                Rule::COMMAND => {
                    self.process_command(inner, commands, options, elements, variables, var_map)
                }
                _ => {}
            }
        }
    }

    fn process_command(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let command = command.into_inner().nth(0).unwrap();
        match command.as_rule() {
            Rule::CMD_OP => self.process_op(commands),
            Rule::CMD_DC => self.process_dc(command, commands),
            Rule::CMD_AC => self.process_ac(command, commands),
            Rule::CMD_TRAN => self.process_tran(command, commands),
            Rule::CMD_INCLUDE => {
                self.process_include(command, commands, options, elements, variables, var_map)
            }
            Rule::CMD_OUT => self.process_out(command, options),
            _ => {}
        }
    }

    fn process_include(
        &self,
        command: Pair<Rule>,
        commands: &mut Vec<SimulationCommand>,
        options: &mut Vec<SimulationOption>,
        elements: &mut Vec<Element>,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let current_path = Path::new(&self.pth).parent().unwrap();
        let path = command.as_str().split(" ").nth(1).unwrap();
        let path = current_path.join(path);
        let mut circuit_string = String::new();

        File::open(path)
            .unwrap()
            .read_to_string(&mut circuit_string)
            .unwrap();

        trace!("Parse Schematic!");
        let parse_result = SpiceParser::parse(Rule::SPICE, &circuit_string)
            .unwrap()
            .next()
            .unwrap();

        for pair in parse_result.into_inner() {
            match pair.as_rule() {
                Rule::DIRECTIVE => {
                    self.process_directive(pair, commands, options, elements, variables, var_map)
                }
                _ => {}
            }
        }
    }

    fn process_op(&self, commands: &mut Vec<SimulationCommand>) {
        commands.push(SimulationCommand::Op)
    }

    fn process_dc(&self, command: Pair<Rule>, commands: &mut Vec<SimulationCommand>) {
        let mut inner = command.into_inner();

        //extract Name
        let source = inner.next().unwrap().as_str();

        //extract vstart
        let vstart = inner.next().unwrap().as_str().parse::<Numeric>().unwrap();

        //extract vend
        let vend = inner.next().unwrap().as_str().parse::<Numeric>().unwrap();

        //extract vstep
        let vstep = inner.next().unwrap().as_str().parse::<Numeric>().unwrap();

        let src2 = inner.next();
        let src2 = if src2.is_none() {
            commands.push(SimulationCommand::Dc(
                Arc::from(source),
                vstart,
                vend,
                vstep,
                None,
            ));
            return;
        } else {
            src2.unwrap()
        };

        let mut src2 = src2.into_inner();

        //extract Name
        let source2 = src2.next().unwrap().as_str();

        //extract Name
        let vstart2 = src2.next().unwrap().as_str().parse::<Numeric>().unwrap();
        //extract Name
        let vend2 = src2.next().unwrap().as_str().parse::<Numeric>().unwrap();
        //extract Name
        let vstep2 = src2.next().unwrap().as_str().parse::<Numeric>().unwrap();

        commands.push(SimulationCommand::Dc(
            Arc::from(source),
            vstart,
            vend,
            vstep,
            Some((Arc::from(source2), vstart2, vend2, vstep2)),
        ));
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

    fn process_out(&self, option: Pair<Rule>, options: &mut Vec<SimulationOption>) {
        let nodes: Vec<Arc<str>> = option
            .into_inner()
            .map(|inner| Arc::from(inner.as_str()))
            .collect();
        options.push(SimulationOption::Out(nodes));
    }

    fn process_element(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let element = element.into_inner().nth(0).unwrap();
        match element.as_rule() {
            Rule::ELE_VSOURCE => self.process_vsource(element, variables, elements, var_map),
            Rule::ELE_ISOURCE => self.process_isource(element, variables, elements, var_map),
            Rule::ELE_RESISTOR => self.process_resistor(element, variables, elements, var_map),
            Rule::ELE_CAPACITOR => self.process_capacitor(element, variables, elements, var_map),
            Rule::ELE_INDUCTOR => self.process_inductor(element, variables, elements, var_map),
            Rule::ELE_DIODE => self.process_diode(element, variables, elements, var_map),
            Rule::ELE_MOSFET => self.process_mosfet(element, variables, elements, var_map),
            _ => {}
        }
    }

    fn process_vsource(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract Node0
        let node0 = inner.next().unwrap().as_span();
        let node0 = &ele[node0.start() - offset..node0.end() - offset];

        //extract Node1
        let node1 = inner.next().unwrap().as_span();
        let node1 = &ele[node1.start() - offset..node1.end() - offset];

        //extract Value
        let value = inner.next().unwrap().as_span();
        let value = ele[value.start() - offset..value.end() - offset]
            .parse::<Numeric>()
            .unwrap();

        let ac_value = if let Some(val) = inner.next() {
            let val = val.as_str().split(" ").nth(1).unwrap();
            let val = val.parse().unwrap();
            Some(val)
        } else {
            None
        };

        let src = VSourceBundle::new(
            Arc::from(name),
            Self::get_variable(&format!("{name}#branch"), Unit::Ampere, variables, var_map)
                .unwrap(),
            Self::get_variable(node0, Unit::Volt, variables, var_map),
            Self::get_variable(node1, Unit::Volt, variables, var_map),
            value,
            ac_value,
        );

        elements.push(Element::VSource(src));
    }

    /// Vsource
    /// vx node0 node1 value
    fn process_isource(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract Node0
        let node0 = inner.next().unwrap().as_span();
        let node0 = &ele[node0.start() - offset..node0.end() - offset];

        //extract Node1
        let node1 = inner.next().unwrap().as_span();
        let node1 = &ele[node1.start() - offset..node1.end() - offset];

        //extract Value
        let value = inner.next().unwrap().as_span();
        let value = ele[value.start() - offset..value.end() - offset]
            .parse::<Numeric>()
            .unwrap();

        let src = ISourceBundle::new(
            Arc::from(name),
            Self::get_variable(node0, Unit::Volt, variables, var_map),
            Self::get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );

        elements.push(Element::ISource(src));
    }

    fn process_resistor(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract Node0
        let node0 = inner.next().unwrap().as_span();
        let node0 = &ele[node0.start() - offset..node0.end() - offset];

        //extract Node1
        let node1 = inner.next().unwrap().as_span();
        let node1 = &ele[node1.start() - offset..node1.end() - offset];

        //extract Value
        let value = inner.next().unwrap().as_span();
        let value = ele[value.start() - offset..value.end() - offset]
            .parse::<Numeric>()
            .unwrap();

        let res = ResistorBundle::new(
            Arc::from(name),
            Self::get_variable(node0, Unit::Volt, variables, var_map),
            Self::get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );
        elements.push(Element::Resistor(res));
    }

    fn process_capacitor(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract Node0
        let node0 = inner.next().unwrap().as_span();
        let node0 = &ele[node0.start() - offset..node0.end() - offset];

        //extract Node1
        let node1 = inner.next().unwrap().as_span();
        let node1 = &ele[node1.start() - offset..node1.end() - offset];

        //extract Value
        let value = inner.next().unwrap().as_span();
        let value = ele[value.start() - offset..value.end() - offset]
            .parse::<Numeric>()
            .unwrap();

        let cap = CapacitorBundle::new(
            Arc::from(name),
            Self::get_variable(node0, Unit::Volt, variables, var_map),
            Self::get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );
        elements.push(Element::Capacitor(cap));
    }

    fn process_inductor(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract Node0
        let node0 = inner.next().unwrap().as_span();
        let node0 = &ele[node0.start() - offset..node0.end() - offset];

        //extract Node1
        let node1 = inner.next().unwrap().as_span();
        let node1 = &ele[node1.start() - offset..node1.end() - offset];

        //extract Value
        let value = inner.next().unwrap().as_span();
        let value = ele[value.start() - offset..value.end() - offset]
            .parse::<Numeric>()
            .unwrap();

        let ind = InductorBundle::new(
            Arc::from(name),
            Self::get_variable(node0, Unit::Volt, variables, var_map),
            Self::get_variable(node1, Unit::Volt, variables, var_map),
            value,
        );
        elements.push(Element::Inductor(ind));
    }

    fn process_diode(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract Node0
        let node0 = inner.next().unwrap().as_span();
        let node0 = &ele[node0.start() - offset..node0.end() - offset];

        //extract Node1
        let node1 = inner.next().unwrap().as_span();
        let node1 = &ele[node1.start() - offset..node1.end() - offset];

        let dio = DiodeBundle::new(
            Arc::from(name),
            Self::get_variable(node0, Unit::Volt, variables, var_map),
            Self::get_variable(node1, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Diode(dio));
    }

    fn process_mosfet(
        &self,
        element: Pair<Rule>,
        variables: &mut Vec<Variable>,
        elements: &mut Vec<Element>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) {
        let ele = element.as_str();
        let offset = element.as_span().start();
        let mut inner = element.into_inner();
        //extract Name
        let name = inner.next().unwrap().as_span().end() - offset;
        let name = &ele[0..name];

        //extract gate node
        let gate_node = inner.next().unwrap().as_span();
        let gate_node = &ele[gate_node.start() - offset..gate_node.end() - offset];

        //extract drain node
        let drain_node = inner.next().unwrap().as_span();
        let drain_node = &ele[drain_node.start() - offset..drain_node.end() - offset];

        //extract drain node
        let source_node = inner.next().unwrap().as_span();
        let source_node = &ele[source_node.start() - offset..source_node.end() - offset];

        let mosfet = Mos0Bundle::new(
            Arc::from(name),
            Self::get_variable(gate_node, Unit::Volt, variables, var_map),
            Self::get_variable(drain_node, Unit::Volt, variables, var_map),
            Self::get_variable(source_node, Unit::Volt, variables, var_map),
            None,
        );
        elements.push(Element::Mos0(mosfet));
    }

    fn get_variable(
        inp: &str,
        unit: Unit,
        variables: &mut Vec<Variable>,
        var_map: &mut HashMap<Arc<str>, usize>,
    ) -> Option<Variable> {
        if inp == "0" {
            return None;
        }

        let inp_arc = Arc::from(inp);

        if let Some(&index) = var_map.get(&inp_arc) {
            return Some(variables[index].clone());
        }

        let new_variable = Variable::new(inp_arc.clone(), unit, variables.len());
        var_map.insert(inp_arc, variables.len());
        variables.push(new_variable.clone());

        Some(new_variable)
    }
}

impl From<pest::error::Error<Rule>> for FrontendError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        FrontendError::PestError(format!("{}", value))
    }
}
