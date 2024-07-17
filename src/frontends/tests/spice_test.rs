use std::sync::Arc;

use crate::{
    frontends::{DiodeBundle, ResistorBundle, VSourceBundle},
    models::{ISourceBundle, Unit, Variable},
    sim::commands::{ACMode, SimulationCommand},
    Frontend, Simulation,
};

use super::super::spice::*;

#[test]
fn parse_resistor1() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_resistor1.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let res = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => todo!(),
        crate::frontends::Element::Inductor(_) => todo!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => todo!(),
        crate::frontends::Element::VSource(_) => todo!(),
        crate::frontends::Element::ISource(_) => todo!(),
    };

    let expected = ResistorBundle::new(
        Arc::from("r1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        20.0,
    );

    assert_eq!(res, &expected);
    assert_eq!(variables.len(), 1)
}

#[test]
fn parse_resistor2() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_resistor2.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let res1 = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => todo!(),
        crate::frontends::Element::Inductor(_) => todo!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => todo!(),
        crate::frontends::Element::VSource(_) => todo!(),
        crate::frontends::Element::ISource(_) => todo!(),
    };
    let res2 = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => todo!(),
        crate::frontends::Element::Inductor(_) => todo!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => todo!(),
        crate::frontends::Element::VSource(_) => todo!(),
        crate::frontends::Element::ISource(_) => todo!(),
    };
    let expected1 = ResistorBundle::new(
        Arc::from("r1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        20.0,
    );
    let expected2 = ResistorBundle::new(
        Arc::from("r2"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("2"), Unit::Volt, 1)),
        20.0,
    );
    assert_eq!(res1, &expected1);
    assert_eq!(res2, &expected2);
    assert_eq!(*variables[0].name(), String::from("1"));
    assert_eq!(*variables[1].name(), String::from("2"));
}
#[test]
fn parse_vsource1() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_vsource1.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let vsource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => todo!(),
    };

    let expected1 = VSourceBundle::new(
        Arc::from("v1"),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        None,
    );

    assert_eq!(vsource, &expected1);
    assert_eq!(*variables[0].name(), String::from("v1#branch"));
    assert_eq!(*variables[1].name(), String::from("1"));
}
#[test]
fn parse_vsource2() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_vsource2.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 3);
    assert_eq!(commands.len(), 1);
    let vsource1 = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => todo!(),
    };
    let vsource2 = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => todo!(),
    };

    let expected1 = VSourceBundle::new(
        Arc::from("v1"),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        None,
    );

    let expected2 = VSourceBundle::new(
        Arc::from("v2"),
        Variable::new(Arc::from("v2#branch"), Unit::Ampere, 2),
        None,
        Some(Variable::new(Arc::from("2"), Unit::Volt, 3)),
        20.0,
        None,
    );

    assert_eq!(vsource1, &expected1);
    assert_eq!(vsource2, &expected2);
    assert_eq!(*variables[0].name(), String::from("v1#branch"));
    assert_eq!(*variables[1].name(), String::from("1"));
    assert_eq!(*variables[2].name(), String::from("v2#branch"));
    assert_eq!(*variables[3].name(), String::from("2"));
}

#[test]
fn parse_vr() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/parse_vr.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let vsource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => todo!(),
    };
    let res = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => todo!(),
    };
    let expected1 = VSourceBundle::new(
        Arc::from("v1"),
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        None,
    );
    let expected2 = ResistorBundle::new(
        Arc::from("r1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        None,
        20.0,
    );
    assert_eq!(vsource, &expected1);
    assert_eq!(res, &expected2);
    assert_eq!(*variables[0].name(), String::from("v1#branch"));
    assert_eq!(*variables[1].name(), String::from("1"));
    assert_eq!(variables.len(), 2);
}

#[test]
#[should_panic]
fn parse_wrong1() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/parse_wrong1.cir".to_string());
    let Simulation {
        variables: _,
        elements,
        commands: _,
    } = parser.simulation().unwrap();

    println!("{elements:?}")
}

#[test]
fn parse_diode1() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/parse_diode1.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let res = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(ele) => ele,
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => todo!(),
    };

    let expected = DiodeBundle::new(
        Arc::from("d1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
    );

    assert_eq!(res, &expected);
    assert_eq!(*variables[0].name(), String::from("1"));
}

#[test]
fn parse_diode2() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/parse_diode2.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let res1 = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(ele) => ele,
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => todo!(),
    };
    let res2 = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(ele) => ele,
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => todo!(),
    };
    let expected1 = DiodeBundle::new(
        Arc::from("d1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
    );
    let expected2 = DiodeBundle::new(
        Arc::from("d2"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        Some(Variable::new(Arc::from("2"), Unit::Volt, 1)),
        None,
    );
    assert_eq!(res1, &expected1);
    assert_eq!(res2, &expected2);
    assert_eq!(*variables[0].name(), String::from("1"));
    assert_eq!(*variables[1].name(), String::from("2"));
}

#[test]
fn parse_regression1() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/regression1.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(variables.len(), 5);
    assert_eq!(elements.len(), 5);
    assert_eq!(commands.len(), 1);
}

#[test]
fn parse_isource1() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_isource1.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let isource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(ele) => ele,
    };

    let expected1 = ISourceBundle::new(
        Arc::from("i1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        5.0,
    );

    assert_eq!(isource, &expected1);
    assert_eq!(*variables[0].name(), String::from("1"));
}

#[test]
fn parse_isource2() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_isource2.cir".to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let isource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(ele) => ele,
    };

    let expected1 = ISourceBundle::new(
        Arc::from("i1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        3.5,
    );

    let resistor = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected2 = ResistorBundle::new(
        Arc::from("r1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
        10.0,
    );

    assert_eq!(isource, &expected1);
    assert_eq!(resistor, &expected2);
    assert_eq!(variables.len(), 1);
    assert_eq!(*variables[0].name(), String::from("1"));
}

#[test]
fn parse_with_include() {
    let main_path = "src/frontends/tests/spice_files/parse_include.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    // Ensure that the included file elements are parsed correctly
    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);

    // Test the first element (ISource)
    let isource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(ele) => ele,
    };

    let expected1 = ISourceBundle::new(
        Arc::from("i1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        3.5,
    );

    assert_eq!(isource, &expected1);

    // Test the second element (Resistor)
    let resistor = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected2 = ResistorBundle::new(
        Arc::from("r1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
        10.0,
    );
    assert_eq!(resistor, &expected2);

    // Ensure that the variables are parsed correctly
    assert_eq!(variables.len(), 1);
    assert_eq!(*variables[0].name(), String::from("1"));
}

#[test]
fn parse_ac() {
    let main_path = "src/frontends/tests/spice_files/parse_ac.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"v1");
    assert_eq!(*elements[1].name(), *"r1");
    assert_eq!(*elements[2].name(), *"c1");

    assert_eq!(
        commands[0],
        SimulationCommand::Ac(1.0, 1000.0, 10, ACMode::Lin)
    )
}

#[test]
fn parse_ac_lin() {
    let main_path = "src/frontends/tests/spice_files/parse_ac_lin.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"v1");
    assert_eq!(*elements[1].name(), *"r1");
    assert_eq!(*elements[2].name(), *"c1");

    assert_eq!(
        commands[0],
        SimulationCommand::Ac(1.0, 1000.0, 10, ACMode::Lin)
    )
}

#[test]
fn parse_ac_dec() {
    let main_path = "src/frontends/tests/spice_files/parse_ac_dec.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"v1");
    assert_eq!(*elements[1].name(), *"r1");
    assert_eq!(*elements[2].name(), *"c1");

    assert_eq!(
        commands[0],
        SimulationCommand::Ac(1.0, 1000.0, 10, ACMode::Dec)
    )
}

#[test]
fn parse_ac_oct() {
    let main_path = "src/frontends/tests/spice_files/parse_ac_oct.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        variables,
        elements,
        commands,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("v1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"v1");
    assert_eq!(*elements[1].name(), *"r1");
    assert_eq!(*elements[2].name(), *"c1");

    assert_eq!(
        commands[0],
        SimulationCommand::Ac(1.0, 1000.0, 10, ACMode::Oct)
    )
}
