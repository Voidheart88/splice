use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use log::trace;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use super::{Frontend, FrontendError, Simulation};
use crate::frontends::{Element, ResistorBundle};
use crate::models::{Unit, Variable};

#[derive(Parser, Debug)]
#[grammar = "frontends/pest/kicad.pest"]
pub struct KicadParser;

/// Represents a KiCad component instance
#[derive(Debug, Clone)]
struct KicadComponent {
    reference: String,
    value: String,
    pins: HashMap<String, String>, // pin_number -> net_name
}

/// Represents a KiCad net
#[derive(Debug, Clone)]
struct KicadNet {
    name: String,
    nodes: Vec<(String, String)>, // (reference, pin_number)
}

/// Parsed KiCad schematic data
#[derive(Debug, Default)]
struct KicadSchematic {
    components: Vec<KicadComponent>,
    nets: Vec<KicadNet>,
}

pub struct KicadFrontend {
    pth: String,
}

impl KicadFrontend {
    pub fn new() -> Self {
        Self {
            pth: String::new(),
        }
    }

    pub fn try_new_from_path(path: String) -> Result<Self, FrontendError> {
        if !path.ends_with(".kicad_sch") {
            return Err(FrontendError::ParseError(
                format!("Expected .kicad_sch file, got: {}", path)
            ));
        }

        if !std::path::Path::new(&path).exists() {
            return Err(FrontendError::FileReadError(format!("File not found: {}", path)));
        }

        Ok(Self { pth: path })
    }

    fn parse_schematic(&self) -> Result<KicadSchematic, FrontendError> {
        let mut content = String::new();
        File::open(&self.pth)?.read_to_string(&mut content)?;

        let mut schematic = KicadSchematic::default();
        let parse_result = KicadParser::parse(Rule::KICAD_SCH, &content)
            .map_err(|e| FrontendError::PestError(e.to_string()))?
            .next()
            .ok_or_else(|| FrontendError::ParseError("Empty or invalid KiCad schematic".into()))?;

        for section in parse_result.into_inner() {
            // sch_item wraps the actual content, so we need to go one level deeper
            let inner = if section.as_rule() == Rule::sch_item {
                section.into_inner().next()
            } else {
                Some(section.clone())
            };
            
            if let Some(item) = inner {
                match item.as_rule() {
                    Rule::symbol => self.parse_symbol(item, &mut schematic)?,
                    Rule::net => self.parse_net(item, &mut schematic)?,
                    Rule::version | Rule::generator | Rule::uuid | Rule::lib_symbols => {
                        // Skip metadata
                    }
                    _ => {}
                }
            }
        }

        Ok(schematic)
    }

    fn parse_symbol(&self, symbol: Pair<Rule>, schematic: &mut KicadSchematic) -> Result<(), FrontendError> {
        let mut reference = String::new();
        let mut value = String::new();
        let mut pins = HashMap::new();

        for child in symbol.into_inner() {
            match child.as_rule() {
                Rule::property => {
                    let props: Vec<_> = child.into_inner().collect();
                    if props.len() >= 2 {
                        let key = props[0].as_str().trim_matches('"');
                        let val = props[1].as_str().trim_matches('"');
                        match key {
                            "Reference" => reference = val.to_string(),
                            "Value" => value = val.to_string(),
                            _ => {}
                        }
                    }
                }
                Rule::pin => {
                    let pin_parts: Vec<_> = child.into_inner().collect();
                    if pin_parts.len() >= 1 {
                        let pin_number = pin_parts[0].as_str().trim_matches('"').to_string();
                        for pin_child in pin_parts.into_iter().skip(1) {
                            if pin_child.as_rule() == Rule::net {
                                let net_name = pin_child.into_inner()
                                    .next()
                                    .map(|n| n.as_str().trim_matches('"').to_string())
                                    .unwrap_or_else(|| format!("N-{}", pin_number));
                                pins.insert(pin_number.clone(), net_name);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if !reference.is_empty() {
            schematic.components.push(KicadComponent { reference, value, pins });
        }

        Ok(())
    }

    fn parse_net(&self, net: Pair<Rule>, schematic: &mut KicadSchematic) -> Result<(), FrontendError> {
        let mut net_name = String::new();
        let mut nodes = Vec::new();

        for child in net.into_inner() {
            match child.as_rule() {
                Rule::name => {
                    net_name = child.into_inner()
                        .next()
                        .map(|n| n.as_str().trim_matches('"').to_string())
                        .unwrap_or_default();
                }
                Rule::node => {
                    let mut node_ref = String::new();
                    let mut node_pin = String::new();
                    for node_child in child.into_inner() {
                        match node_child.as_rule() {
                            Rule::comp_ref => {
                                node_ref = node_child.into_inner()
                                    .next()
                                    .map(|r| r.as_str().trim_matches('"').to_string())
                                    .unwrap_or_default();
                            }
                            Rule::pin => {
                                node_pin = node_child.into_inner()
                                    .next()
                                    .map(|p| p.as_str().trim_matches('"').to_string())
                                    .unwrap_or_default();
                            }
                            _ => {}
                        }
                    }
                    if !node_ref.is_empty() {
                        nodes.push((node_ref, node_pin));
                    }
                }
                _ => {}
            }
        }

        if !net_name.is_empty() {
            schematic.nets.push(KicadNet { name: net_name, nodes });
        }

        Ok(())
    }

    fn build_net_map(schematic: &KicadSchematic) -> HashMap<String, String> {
        let mut net_map = HashMap::new();
        for net in &schematic.nets {
            for (ref_name, pin_num) in &net.nodes {
                let key = format!("{}.{}", ref_name, pin_num);
                net_map.insert(key, net.name.clone());
            }
        }
        for comp in &schematic.components {
            for (pin_num, net_name) in &comp.pins {
                if !net_map.contains_key(&format!("{}.{}", comp.reference, pin_num)) {
                    net_map.insert(format!("{}.{}", comp.reference, pin_num), net_name.clone());
                }
            }
        }
        net_map
    }

    pub fn parse_value(value: &str) -> Result<f64, FrontendError> {
        let value = value.trim();
        if let Ok(val) = value.parse::<f64>() {
            return Ok(val);
        }

        let suffixes = [("meg", 1e6), ("k", 1e3), ("m", 1e-3), ("u", 1e-6), ("µ", 1e-6), 
                        ("n", 1e-9), ("p", 1e-12), ("f", 1e-15), ("g", 1e9), ("t", 1e12)];
        
        for (suffix, multiplier) in &suffixes {
            if value.ends_with(suffix) {
                let num_str = &value[..value.len() - suffix.len()];
                if let Ok(num) = num_str.parse::<f64>() {
                    return Ok(num * multiplier);
                }
            }
        }

        value.parse::<f64>().map_err(|_| {
            FrontendError::ParseError(format!("Invalid value: {}", value))
        })
    }

    fn get_node_name(ref_name: &str, pin: &str, net_map: &HashMap<String, String>) -> String {
        let key = format!("{}.{}", ref_name, pin);
        net_map.get(&key).cloned().unwrap_or_else(|| format!("N_{}_{}", ref_name, pin))
    }
}

impl Frontend for KicadFrontend {
    fn simulation(&self) -> Result<Simulation, FrontendError> {
        trace!("Parsing KiCad schematic: {}", self.pth);

        let schematic = self.parse_schematic()?;
        let net_map = Self::build_net_map(&schematic);

        let mut elements = Vec::new();
        let mut variables: Vec<Variable> = Vec::new();
        let mut var_map: HashMap<Arc<str>, usize> = HashMap::new();
        let mut node_idx = 0;

        // Process components
        for comp in &schematic.components {
            let prefix = comp.reference.chars().next().unwrap_or('X');
            
            match prefix {
                'R' => {
                    let node_plus_name = Self::get_node_name(&comp.reference, "1", &net_map);
                    let node_minus_name = Self::get_node_name(&comp.reference, "2", &net_map);
                    let resistance = Self::parse_value(&comp.value)?;

                    // Get or create node_plus variable
                    let node_plus_arc: Arc<str> = node_plus_name.into();
                    let node_plus_idx = if let Some(&idx) = var_map.get(&node_plus_arc) {
                        idx
                    } else {
                        let idx = node_idx;
                        let var = Variable::new(node_plus_arc.clone(), Unit::Volt, idx);
                        var_map.insert(node_plus_arc, idx);
                        variables.push(var);
                        node_idx += 1;
                        idx
                    };

                    // Get or create node_minus variable
                    let node_minus_arc: Arc<str> = node_minus_name.into();
                    let node_minus_idx = if let Some(&idx) = var_map.get(&node_minus_arc) {
                        idx
                    } else {
                        let idx = node_idx;
                        let var = Variable::new(node_minus_arc.clone(), Unit::Volt, idx);
                        var_map.insert(node_minus_arc, idx);
                        variables.push(var);
                        node_idx += 1;
                        idx
                    };

                    let node_plus = variables[node_plus_idx].clone();
                    let node_minus = variables[node_minus_idx].clone();

                    elements.push(Element::Resistor(ResistorBundle::new(
                        comp.reference.clone().into(),
                        Some(node_plus),
                        Some(node_minus),
                        resistance,
                    )));
                }
                'C' | 'L' => {
                    // Not yet implemented
                    trace!("Skipping {} (not yet implemented): {}", prefix, comp.reference);
                }
                'V' | 'I' => {
                    // These require additional setup (branch variables for V, etc.) - skip for now
                    trace!("Skipping {} (not yet implemented): {}", prefix, comp.reference);
                }
                _ => {
                    trace!("Skipping unsupported component: {}", comp.reference);
                }
            }
        }

        // Validate unique element names
        let mut names = HashSet::new();
        for ele in &elements {
            let name = ele.name();
            if !names.insert(name.clone()) {
                return Err(FrontendError::ElementDouble(name.to_string()));
            }
        }

        trace!(
            "KiCad schematic parsed: {} elements, {} nodes",
            elements.len(),
            variables.len()
        );

        Ok(Simulation {
            commands: Vec::new(),
            options: Vec::new(),
            elements,
            variables,
        })
    }
}


