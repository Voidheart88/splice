use crate::frontends::spice::SpiceFrontend;
use crate::models::Unit;



#[test]
fn test_node_units_parsing() {
    let spice_code = "
* Test circuit with node units
V1 0 1 5
R1 1 2 100

.node_units 1=V 2=V
";

    let result = SpiceFrontend::parse_spice_code(spice_code);
    assert!(result.is_ok(), "Failed to parse SPICE code with node units: {:?}", result);

    let simulation = result.unwrap();
    let variables = simulation.variables;
    
    // Find nodes 1 and 2 and check their units
    let node_1 = variables.iter().find(|v| v.name().as_ref() == "1");
    let node_2 = variables.iter().find(|v| v.name().as_ref() == "2");
    
    assert!(node_1.is_some(), "Node 1 not found");
    assert!(node_2.is_some(), "Node 2 not found");
    
    let node_1 = node_1.unwrap();
    let node_2 = node_2.unwrap();
    
    assert_eq!(node_1.unit(), Unit::Volt, "Node 1 should have Volt unit");
    assert_eq!(node_2.unit(), Unit::Volt, "Node 2 should have Volt unit");
}

#[test]
fn test_node_units_multiple_units() {
    let spice_code = "
* Test circuit with multiple unit types
V1 0 1 5
R1 1 2 100
I1 0 3 0.01

.node_units 1=V 2=V 3=A
";

    let result = SpiceFrontend::parse_spice_code(spice_code);
    assert!(result.is_ok(), "Failed to parse SPICE code with multiple units: {:?}", result);

    let simulation = result.unwrap();
    let variables = simulation.variables;
    
    // Find nodes and check their units
    let node_1 = variables.iter().find(|v| v.name().as_ref() == "1");
    let node_2 = variables.iter().find(|v| v.name().as_ref() == "2");
    let node_3 = variables.iter().find(|v| v.name().as_ref() == "3");
    
    assert!(node_1.is_some(), "Node 1 not found");
    assert!(node_2.is_some(), "Node 2 not found");
    assert!(node_3.is_some(), "Node 3 not found");
    
    let node_1 = node_1.unwrap();
    let node_2 = node_2.unwrap();
    let node_3 = node_3.unwrap();
    
    assert_eq!(node_1.unit(), Unit::Volt, "Node 1 should have Volt unit");
    assert_eq!(node_2.unit(), Unit::Volt, "Node 2 should have Volt unit");
    assert_eq!(node_3.unit(), Unit::Ampere, "Node 3 should have Ampere unit");
}

#[test]
fn test_node_units_unknown_unit() {
    let spice_code = "
* Test circuit with unknown unit
V1 0 1 5

.node_units 1=XYZ
";

    let result = SpiceFrontend::parse_spice_code(spice_code);
    assert!(result.is_err(), "Should fail with unknown unit");
}

#[test]
fn test_node_units_nonexistent_node() {
    let spice_code = "
* Test circuit with non-existent node
V1 0 1 5

.node_units 99=V
";

    let result = SpiceFrontend::parse_spice_code(spice_code);
    assert!(result.is_err(), "Should fail with non-existent node");
}