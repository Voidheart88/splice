use std::sync::Arc;

use crate::{
    frontends::{DiodeBundle, ResistorBundle, VSourceBundle},
    models::{Element, ISourceBundle, Unit, Variable},
    sim::commands::{ACMode, SimulationCommand},
    Frontend, Simulation,
};

use super::super::spice::*;

#[test]
fn parse_resistor1() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_resistor1.cir".to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let res = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected = ResistorBundle::new(
        Arc::from("R1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        20.0,
    );

    assert_eq!(res, &expected);
    assert_eq!(variables.len(), 1);
}

#[test]
fn parse_resistor2() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_resistor2.cir".to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let res1 = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };
    let res2 = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };
    let expected1 = ResistorBundle::new(
        Arc::from("R1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        20.0,
    );
    let expected2 = ResistorBundle::new(
        Arc::from("R2"),
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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let vsource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected1 = VSourceBundle::new(
        Arc::from("V1"),
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        None,
    );

    assert_eq!(vsource, &expected1);
    assert_eq!(*variables[0].name(), String::from("V1#branch"));
    assert_eq!(*variables[1].name(), String::from("1"));
}
#[test]
fn parse_vsource2() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_vsource2.cir".to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 3);
    assert_eq!(commands.len(), 1);
    let vsource1 = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => panic!(),
    };
    let vsource2 = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected1 = VSourceBundle::new(
        Arc::from("V1"),
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        None,
    );

    let expected2 = VSourceBundle::new(
        Arc::from("V2"),
        Variable::new(Arc::from("V2#branch"), Unit::Ampere, 2),
        None,
        Some(Variable::new(Arc::from("2"), Unit::Volt, 3)),
        20.0,
        None,
    );

    assert_eq!(vsource1, &expected1);
    assert_eq!(vsource2, &expected2);
    assert_eq!(*variables[0].name(), String::from("V1#branch"));
    assert_eq!(*variables[1].name(), String::from("1"));
    assert_eq!(*variables[2].name(), String::from("V2#branch"));
    assert_eq!(*variables[3].name(), String::from("2"));
}

#[test]
fn parse_vr() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/parse_vr.cir".to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let vsource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(ele) => ele,
        crate::frontends::Element::ISource(_) => panic!(),
    };
    let res = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };
    let expected1 = VSourceBundle::new(
        Arc::from("V1"),
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        None,
    );
    let expected2 = ResistorBundle::new(
        Arc::from("R1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        None,
        20.0,
    );
    assert_eq!(vsource, &expected1);
    assert_eq!(res, &expected2);
    assert_eq!(*variables[0].name(), String::from("V1#branch"));
    assert_eq!(*variables[1].name(), String::from("1"));
    assert_eq!(variables.len(), 2);
}

#[test]
#[should_panic]
fn parse_wrong1() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/parse_wrong1.cir".to_string());
    let Simulation {
        commands: _optional,
        options: _,
        elements,
        variables: _,
    } = parser.simulation().unwrap();

    println!("{elements:?}")
}

#[test]
fn parse_diode1() {
    let parser = SpiceFrontend::new("src/frontends/tests/spice_files/parse_diode1.cir".to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let res = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(ele) => ele,
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected = DiodeBundle::new(
        Arc::from("D1"),
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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let res1 = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(ele) => ele,
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };
    let res2 = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(ele) => ele,
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };
    let expected1 = DiodeBundle::new(
        Arc::from("D1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
    );
    let expected2 = DiodeBundle::new(
        Arc::from("D2"),
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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(variables.len(), 5);
    assert_eq!(elements.len(), 5);
    assert_eq!(commands.len(), 1);

    println!("{:?}", elements);
}

#[test]
fn parse_isource1() {
    let parser =
        SpiceFrontend::new("src/frontends/tests/spice_files/parse_isource1.cir".to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 1);
    assert_eq!(commands.len(), 0);
    let isource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(ele) => ele,
    };

    let expected1 = ISourceBundle::new(
        Arc::from("I1"),
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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(elements.len(), 2);
    assert_eq!(commands.len(), 0);
    let isource = match &elements[0] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(_) => panic!(),
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(ele) => ele,
    };

    let expected1 = ISourceBundle::new(
        Arc::from("I1"),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        3.5,
    );

    let resistor = match &elements[1] {
        crate::frontends::Element::Capacitor(_) => panic!(),
        crate::frontends::Element::Inductor(_) => panic!(),
        crate::frontends::Element::Resistor(ele) => ele,
        crate::frontends::Element::Diode(_) => panic!(),
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected2 = ResistorBundle::new(
        Arc::from("R1"),
        Some(Variable::new(Arc::from("1"), Unit::Volt, 0)),
        None,
        10.0,
    );

    assert_eq!(isource, &expected1);
    assert_eq!(resistor, &expected2);
    assert_eq!(variables.len(), 1);
    assert_eq!(*variables[0].name(), *"1");
}

#[test]
fn parse_with_include() {
    let main_path = "src/frontends/tests/spice_files/parse_include.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
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
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(ele) => ele,
    };

    let expected1 = ISourceBundle::new(
        Arc::from("I1"),
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
        crate::frontends::Element::Mos0(_) => panic!(),
        crate::frontends::Element::VSource(_) => panic!(),
        crate::frontends::Element::ISource(_) => panic!(),
    };

    let expected2 = ResistorBundle::new(
        Arc::from("R1"),
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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"R1");
    assert_eq!(*elements[2].name(), *"C1");

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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"R1");
    assert_eq!(*elements[2].name(), *"C1");

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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"R1");
    assert_eq!(*elements[2].name(), *"C1");

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
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"R1");
    assert_eq!(*elements[2].name(), *"C1");

    assert_eq!(
        commands[0],
        SimulationCommand::Ac(1.0, 1000.0, 10, ACMode::Oct)
    )
}

#[test]
fn parse_vsource_ac_option() {
    let main_path = "src/frontends/tests/spice_files/parse_vsource_ac.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"R1");
    assert_eq!(*elements[2].name(), *"R2");

    let ele = elements[0].clone();
    let exp = Element::VSource(VSourceBundle::new(
        Arc::from("V1"),
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0),
        None,
        Some(Variable::new(Arc::from("1"), Unit::Volt, 1)),
        10.0,
        Some(1.0),
    ));
    assert_eq!(ele, exp);
    assert_eq!(
        commands[0],
        SimulationCommand::Ac(1.0, 1000.0, 10, ACMode::Lin)
    )
}

#[test]
fn parse_dc_single() {
    let main_path = "src/frontends/tests/spice_files/parse_dc_0.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(variables[2], Variable::new(Arc::from("2"), Unit::Volt, 2));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"R1");
    assert_eq!(*elements[2].name(), *"R2");

    assert_eq!(
        commands[0],
        SimulationCommand::Dc(Arc::from("V1"), 1.0, 10.0, 0.1, None)
    )
}

#[test]
fn parse_dc_double() {
    let main_path = "src/frontends/tests/spice_files/parse_dc_1.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(
        variables[2],
        Variable::new(Arc::from("V2#branch"), Unit::Ampere, 2)
    );
    assert_eq!(variables[3], Variable::new(Arc::from("3"), Unit::Volt, 3));
    assert_eq!(variables[4], Variable::new(Arc::from("2"), Unit::Volt, 4));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"V2");
    assert_eq!(*elements[2].name(), *"R1");
    assert_eq!(*elements[3].name(), *"R2");
    assert_eq!(*elements[4].name(), *"R3");

    assert_eq!(
        commands[0],
        SimulationCommand::Dc(
            Arc::from("V1"),
            1.0,
            10.0,
            0.1,
            Some((Arc::from("V2"), 1.0, 10.0, 0.1))
        )
    )
}

#[test]
fn parse_mosfet() {
    let main_path = "src/frontends/tests/spice_files/parse_mosfet.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        commands,
        options: _,
        elements,
        variables,
    } = parser.simulation().unwrap();

    assert_eq!(
        variables[0],
        Variable::new(Arc::from("V1#branch"), Unit::Ampere, 0)
    );
    assert_eq!(variables[1], Variable::new(Arc::from("1"), Unit::Volt, 1));
    assert_eq!(
        variables[2],
        Variable::new(Arc::from("V2#branch"), Unit::Ampere, 2)
    );
    assert_eq!(variables[3], Variable::new(Arc::from("2"), Unit::Volt, 3));

    assert_eq!(*elements[0].name(), *"V1");
    assert_eq!(*elements[1].name(), *"V2");
    assert_eq!(*elements[2].name(), *"M1");

    assert_eq!(
        commands[0],
        SimulationCommand::Dc(Arc::from("V0"), 0.0, 5.0, 0.1, None)
    )
}

#[test]
fn parse_out1() {
    let main_path = "src/frontends/tests/spice_files/parse_out1.cir";

    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        commands: _,
        options: _,
        elements: _,
        variables: _,
    } = parser.simulation().unwrap();
}

#[test]
fn parse_minimal_circuit() {
    let main_path = "src/frontends/tests/spice_files/parse_minimal_circuit.cir";
    let parser = SpiceFrontend::new(main_path.to_string());

    let Simulation {
        commands,
        options,
        elements,
        variables,
    } = parser.simulation().unwrap();

    println!("{:?}", commands);
    println!("{:?}", options);
    println!("{:?}", elements);
    println!("{:?}", variables);
}
