use std::{fs::File, io::Read};

use super::super::yaml::*;

#[test]
fn process_minimal() {
    let mut input = File::open("src/frontends/tests/yaml_files/simple.yaml").unwrap();
    let mut input_string = String::new();
    input.read_to_string(&mut input_string).unwrap();
    let _: YamlCircuit = serde_yml::from_str(&input_string).unwrap();
}

#[test]
fn process_full() {
    let mut input = File::open("src/frontends/tests/yaml_files/full.yaml").unwrap();
    let mut input_string = String::new();
    input.read_to_string(&mut input_string).unwrap();
    let _: YamlCircuit = serde_yml::from_str(&input_string).unwrap();
}
