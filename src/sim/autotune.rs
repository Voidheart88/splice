use log::info;

use crate::models::Element;
use crate::sim::commands::SimulationCommand;
use crate::sim::options::{IntegrationMethod, SimulationOption};
use crate::spot::Numeric;

/// Represents a time constant analysis result
#[derive(Debug, Clone)]
struct TimeConstantAnalysis {
    min_time_constant: Option<Numeric>,
    max_time_constant: Option<Numeric>,
    has_rc_circuits: bool,
    has_rl_circuits: bool,
}

/// Represents circuit characteristics for autotuning
#[derive(Debug, Clone)]
struct CircuitCharacteristics {
    time_constants: TimeConstantAnalysis,
    has_nonlinear_elements: bool,
    has_capacitors: bool,
    has_inductors: bool,
    has_diodes: bool,
    has_mosfets: bool,
}

/// Analyzes the circuit and suggests optimal simulation settings
pub fn analyze_circuit_and_suggest_settings(
    elements: &[Element],
    commands: &[SimulationCommand],
) -> (Vec<SimulationOption>, Vec<SimulationCommand>) {
    let mut _options: Vec<SimulationOption> = Vec::new();
    let mut modified_commands = commands.to_vec();
    let mut options = Vec::new();

    info!("Analyzing circuit for optimal simulation settings...");

    // Perform comprehensive circuit analysis
    let circuit_analysis = analyze_circuit_elements(elements);

    // Analyze simulation commands
    let has_transient = commands
        .iter()
        .any(|cmd| matches!(cmd, SimulationCommand::Tran(_, _)));
    let has_ac = commands
        .iter()
        .any(|cmd| matches!(cmd, SimulationCommand::Ac(_, _, _, _)));
    let has_dc = commands
        .iter()
        .any(|cmd| matches!(cmd, SimulationCommand::Dc(_, _, _, _, _)));

    info!("Circuit analysis:");
    info!(
        "  Nonlinear elements: {}",
        circuit_analysis.has_nonlinear_elements
    );
    info!("  Capacitors: {}", circuit_analysis.has_capacitors);
    info!("  Inductors: {}", circuit_analysis.has_inductors);
    info!("  Diodes: {}", circuit_analysis.has_diodes);
    info!("  MOSFETs: {}", circuit_analysis.has_mosfets);
    info!(
        "  Simulation types - Transient: {}, AC: {}, DC: {}",
        has_transient, has_ac, has_dc
    );

    // Display time constant analysis
    if let Some(min_tc) = circuit_analysis.time_constants.min_time_constant {
        info!("  Min time constant: {:.2e} s", min_tc);
    }
    if let Some(max_tc) = circuit_analysis.time_constants.max_time_constant {
        info!("  Max time constant: {:.2e} s", max_tc);
    }

    // Suggest integration method based on circuit characteristics
    let integration_method = suggest_integration_method(
        circuit_analysis.has_nonlinear_elements,
        circuit_analysis.has_capacitors,
        circuit_analysis.has_inductors,
    );
    options.push(SimulationOption::IntegrationMethod(
        integration_method.clone(),
    ));

    // Set optimal timestep for transient simulations
    if has_transient {
        if let Some(suggested_timestep) = suggest_optimal_timestep(&circuit_analysis) {
            info!(
                "  -> Setting optimal timestep: {:.2e} s for transient analysis",
                suggested_timestep
            );
            // Update the Tran command with the suggested timestep
            modified_commands = commands
                .iter()
                .map(|cmd| {
                    if let SimulationCommand::Tran(_, tstop) = cmd {
                        SimulationCommand::Tran(suggested_timestep, *tstop)
                    } else {
                        cmd.clone()
                    }
                })
                .collect();
        }
    }

    // Suggest AC analysis parameters if applicable
    if has_ac {
        suggest_ac_parameters_based_on_circuit(&circuit_analysis);
    }

    info!("Autotune suggestions applied:");
    info!("  Integration method: {:?}", integration_method);

    (options, modified_commands)
}

/// Analyzes circuit elements and extracts characteristics
fn analyze_circuit_elements(elements: &[Element]) -> CircuitCharacteristics {
    let mut characteristics = CircuitCharacteristics {
        time_constants: TimeConstantAnalysis {
            min_time_constant: None,
            max_time_constant: None,
            has_rc_circuits: false,
            has_rl_circuits: false,
        },
        has_nonlinear_elements: false,
        has_capacitors: false,
        has_inductors: false,
        has_diodes: false,
        has_mosfets: false,
    };

    // Collect resistors, capacitors, and inductors for time constant analysis
    let mut resistors = Vec::new();
    let mut capacitors = Vec::new();
    let mut inductors = Vec::new();

    for element in elements {
        extract_element_values(
            element,
            &mut resistors,
            &mut capacitors,
            &mut inductors,
            &mut characteristics,
        );
    }

    // Analyze time constants
    analyze_rc_time_constants(
        &resistors,
        &capacitors,
        &mut characteristics.time_constants,
    );

    analyze_rl_time_constants(
        &resistors,
        &inductors,
        &mut characteristics.time_constants,
    );

    characteristics
}

/// Helper function to extract element values and update characteristics
fn extract_element_values(
    element: &Element,
    resistors: &mut Vec<Numeric>,
    capacitors: &mut Vec<Numeric>,
    inductors: &mut Vec<Numeric>,
    characteristics: &mut CircuitCharacteristics,
) {
    match element {
        Element::Resistor(res) => resistors.push(res.value()),
        Element::Capacitor(cap) => {
            capacitors.push(cap.value);
            characteristics.has_capacitors = true;
        }
        Element::Inductor(ind) => {
            inductors.push(ind.value);
            characteristics.has_inductors = true;
        }
        Element::Diode(_) => {
            characteristics.has_diodes = true;
            characteristics.has_nonlinear_elements = true;
        }
        Element::Mos0(_) => {
            characteristics.has_mosfets = true;
            characteristics.has_nonlinear_elements = true;
        }
        _ => {}
    }
}

/// Updates time constant analysis with new values
fn update_time_constants(analysis: &mut TimeConstantAnalysis, new_time_constants: &[Numeric]) {
    for &tc in new_time_constants {
        // Filter out extremely small or large values that might be numerical artifacts
        update_min_max_time_constants(analysis, tc);
    }
}

/// Helper function to analyze RC time constants
fn analyze_rc_time_constants(
    resistors: &[Numeric],
    capacitors: &[Numeric],
    time_constants: &mut TimeConstantAnalysis,
) {
    if !resistors.is_empty() && !capacitors.is_empty() {
        time_constants.has_rc_circuits = true;
        let rc_time_constants: Vec<Numeric> = resistors
            .iter()
            .flat_map(|&r| capacitors.iter().map(move |&c| r * c))
            .collect();

        update_time_constants(time_constants, &rc_time_constants);
    }
}

/// Helper function to analyze RL time constants
fn analyze_rl_time_constants(
    resistors: &[Numeric],
    inductors: &[Numeric],
    time_constants: &mut TimeConstantAnalysis,
) {
    if !resistors.is_empty() && !inductors.is_empty() {
        time_constants.has_rl_circuits = true;
        let rl_time_constants: Vec<Numeric> = inductors
            .iter()
            .flat_map(|&l| resistors.iter().map(move |&r| l / r))
            .collect();

        update_time_constants(time_constants, &rl_time_constants);
    }
}

/// Helper function to update min and max time constants
fn update_min_max_time_constants(analysis: &mut TimeConstantAnalysis, tc: Numeric) {
    if tc > 1e-15 && tc < 1e3 {
        // Update min time constant
        match analysis.min_time_constant {
            Some(min_tc) if tc < min_tc => analysis.min_time_constant = Some(tc),
            None => analysis.min_time_constant = Some(tc),
            Some(_) => {}
        }

        // Update max time constant
        match analysis.max_time_constant {
            Some(max_tc) if tc > max_tc => analysis.max_time_constant = Some(tc),
            None => analysis.max_time_constant = Some(tc),
            Some(_) => {}
        }
    }
}

/// Suggests the optimal integration method based on circuit characteristics
fn suggest_integration_method(
    has_nonlinear_elements: bool,
    has_capacitors: bool,
    has_inductors: bool,
) -> IntegrationMethod {
    // Default to Backward Euler for stability, especially with nonlinear elements
    // Trapezoidal method can be more accurate but may be unstable with nonlinearities
    if has_nonlinear_elements {
        info!("  -> Using Backward Euler for stability with nonlinear elements");
        return IntegrationMethod::BackwardEuler;
    }

    // For circuits with capacitors and inductors, trapezoidal can be more accurate
    if has_capacitors && has_inductors {
        info!("  -> Using Trapezoidal for better accuracy with LC circuits");
        return IntegrationMethod::Trapezoidal;
    }

    // For circuits with only capacitors, trapezoidal can be a good choice
    if has_capacitors {
        info!("  -> Using Trapezoidal for better accuracy with capacitive circuits");
        return IntegrationMethod::Trapezoidal;
    }

    // Default to Backward Euler for general stability
    info!("  -> Using Backward Euler as default for stability");
    IntegrationMethod::BackwardEuler
}

/// Suggests optimal timestep based on circuit time constants
fn suggest_optimal_timestep(analysis: &CircuitCharacteristics) -> Option<Numeric> {
    analysis.time_constants.min_time_constant.map(|min_tc| {
        // Use 1/20 to 1/50 of the smallest time constant as a good starting point
        // This ensures we capture the fastest dynamics in the circuit
        let suggested_timestep = min_tc / 20.0;

        // Apply reasonable bounds
        suggested_timestep.clamp(1e-12, 1e-6)
    })
}

/// Suggests AC analysis parameters based on circuit characteristics
fn suggest_ac_parameters_based_on_circuit(analysis: &CircuitCharacteristics) {
    if let (Some(min_tc), Some(max_tc)) = (
        analysis.time_constants.min_time_constant,
        analysis.time_constants.max_time_constant,
    ) {
        // Calculate frequency range based on time constants
        // f_min = 1/(2π * max_time_constant)
        // f_max = 1/(2π * min_time_constant)
        let f_min = 1.0 / (2.0 * std::f64::consts::PI * max_tc);
        let f_max = 1.0 / (2.0 * std::f64::consts::PI * min_tc);

        info!(
            "  -> Suggested AC frequency range: {:.2e} Hz to {:.2e} Hz",
            f_min, f_max
        );

        // Suggest logarithmic scale for wide frequency ranges
        if f_max / f_min > 100.0 {
            info!("  -> Suggested AC mode: Decade (logarithmic scale) for wide frequency range");
        }
    }
}

/// Analyzes transient simulation parameters and suggests optimal settings
pub fn analyze_transient_parameters(tstep: Numeric, tstop: Numeric) -> (Numeric, Numeric) {
    // For now, just return the original parameters
    // In the future, we can add more sophisticated analysis
    (tstep, tstop)
}

/// Analyzes AC simulation parameters and suggests optimal settings
pub fn analyze_ac_parameters(
    fstart: Numeric,
    fend: Numeric,
    steps: usize,
) -> (Numeric, Numeric, usize) {
    // For now, just return the original parameters
    // In the future, we can add more sophisticated analysis
    (fstart, fend, steps)
}
