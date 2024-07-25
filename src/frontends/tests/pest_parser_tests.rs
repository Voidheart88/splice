use pest::Parser;

use super::super::spice::*;

#[test]
fn process_minimal_vsource() {
    let input = "v 0 1 10";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_whitespace() {
    let input = "R1 0 \t 1 10";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_minimal_resistor() {
    let input = "r 1 2 100";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_resistor_case() {
    let input = "R 1 2 100";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_vsource_case() {
    let input = "V1 1 2 100";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    let input = "v1 1 2 100";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_resistor_with_designator() {
    let input = "R1 1 2 100";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_minimal_diode() {
    let input = "d 2 3";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_op_command() {
    let input = ".op";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_op_command_case() {
    let input = ".oP";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_dc_command() {
    let input = ".dc V1 0 10 1";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_ac_command() {
    let input = ".ac 1 10 100";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_tran_command() {
    let input = ".tran 0.1 1";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_with_suffix1() {
    let input = ".tran 1e-3 1";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_with_suffix2() {
    let input = "R1 0 1 10e3";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_node() {
    let input = "R1 INPUT OUTPUT 10e3";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_lines() {
    let input = "V1 0 N0 10\nR1 N0 N1 10";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_empty_lines() {
    let input = "V1 0 N0 10\n\nR1 N0 N1 10\n ";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_comment() {
    let input = "*Testcomment";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_diode_regression1() {
    let input = "D1 0 1\nD2 1 2";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_include() {
    let input = ".include included.cir";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_ac_vsource() {
    let input = "V1 0 1 10 AC 1";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_mos0() {
    let input = "M0 1 2 3";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_out_option1() {
    let input = ".out 1";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_out_option2() {
    let input = ".out 1 2 3 4";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}

#[test]
fn process_out_option3() {
    let input = ".out 1 2 3 4\n";
    SpiceParser::parse(Rule::SPICE, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
}
