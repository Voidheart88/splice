use super::super::kicad::KicadFrontend;
use super::super::Frontend;

#[test]
fn test_kicad_parse_resistor() {
    let frontend = KicadFrontend::try_new_from_path("src/frontends/tests/kicad_files/test_resistor.kicad_sch".to_string())
        .expect("Failed to create KiCad frontend");
    
    let simulation = frontend.simulation()
        .expect("Failed to parse KiCad schematic");
    
    assert_eq!(simulation.elements.len(), 1, "Should have 1 element");
    
    // Check that we have a resistor
    match &simulation.elements[0] {
        crate::frontends::Element::Resistor(resistor) => {
            assert_eq!(resistor.name().as_ref(), "R1");
            assert_eq!(resistor.value(), 10000.0);
        }
        _ => panic!("Expected Resistor element"),
    }
    
    // Should have 2 nodes: N-001, GND
    assert_eq!(simulation.variables.len(), 2, "Should have 2 nodes");
}

#[test]
fn test_kicad_parse_multiple_resistors() {
    let frontend = KicadFrontend::try_new_from_path("src/frontends/tests/kicad_files/test_multiple_resistors.kicad_sch".to_string())
        .expect("Failed to create KiCad frontend");
    
    let simulation = frontend.simulation()
        .expect("Failed to parse KiCad schematic");
    
    assert_eq!(simulation.elements.len(), 2, "Should have 2 elements");
    
    // Verify we have two resistors
    let mut r1_found = false;
    let mut r2_found = false;
    
    for ele in &simulation.elements {
        match ele {
            crate::frontends::Element::Resistor(r) => {
                if r.name().as_ref() == "R1" {
                    r1_found = true;
                    assert_eq!(r.value(), 1000.0);
                } else if r.name().as_ref() == "R2" {
                    r2_found = true;
                    assert_eq!(r.value(), 2000.0);
                }
            }
            _ => {}
        }
    }
    
    assert!(r1_found, "Should have R1");
    assert!(r2_found, "Should have R2");
    
    // Should have 3 nodes: N-001, N-002, GND
    assert_eq!(simulation.variables.len(), 3, "Should have 3 nodes");
}

#[test]
fn test_kicad_invalid_extension() {
    let result = KicadFrontend::try_new_from_path("test.txt".to_string());
    
    assert!(result.is_err(), "Should reject non-.kicad_sch files");
}

#[test]
fn test_kicad_file_not_found() {
    let result = KicadFrontend::try_new_from_path("nonexistent.kicad_sch".to_string());
    
    assert!(result.is_err(), "Should reject nonexistent files");
}

#[test]
fn test_kicad_value_parsing_with_suffixes() {
    // Test various KiCad value formats
    assert_eq!(KicadFrontend::parse_value("10k").unwrap(), 10000.0);
    assert_eq!(KicadFrontend::parse_value("100n").unwrap(), 100e-9);
    assert_eq!(KicadFrontend::parse_value("1u").unwrap(), 1e-6);
    assert_eq!(KicadFrontend::parse_value("4.7u").unwrap(), 4.7e-6);
    assert_eq!(KicadFrontend::parse_value("5").unwrap(), 5.0);
    assert_eq!(KicadFrontend::parse_value("2.2meg").unwrap(), 2.2e6);
    assert_eq!(KicadFrontend::parse_value("10p").unwrap(), 10e-12);
    assert_eq!(KicadFrontend::parse_value("1f").unwrap(), 1e-15);
}
