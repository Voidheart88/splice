/// Tests for IntegratorBundle
use super::*;

#[test]
fn test_integrator_creation() {
    let integrator = IntegratorBundle::new("INT1".into(), None, None, 1.0, 0.0);
    assert_eq!(integrator.name(), "INT1".into());
}

#[test]
fn test_integrator_with_gain() {
    let integrator = IntegratorBundle::new("INT1".into(), None, None, 2.5, 0.0);
    assert_eq!(integrator.gain(), 2.5);
}

#[test]
fn test_integrator_initial_condition() {
    let integrator = IntegratorBundle::new("INT1".into(), None, None, 1.0, 5.0);
    assert_eq!(integrator.initial_condition(), 5.0);
    assert_eq!(integrator.previous_output(), 5.0);
}

#[test]
fn test_integrator_input_output_idx() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let integrator = IntegratorBundle::new(
        "INT1".into(),
        Some(input.clone()),
        Some(output.clone()),
        1.0,
        0.0,
    );

    assert_eq!(integrator.input_idx(), Some(0));
    assert_eq!(integrator.output_idx(), Some(1));
}

#[test]
fn test_integrator_triple_idx() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let integrator = IntegratorBundle::new("INT1".into(), Some(input), Some(output), 1.0, 0.0);

    let triple_idx = integrator.triple_idx();
    assert!(triple_idx.is_some());
    let idx = triple_idx.unwrap();
    assert_eq!(idx.len(), 2); // output->output and output->input
}

#[test]
fn test_integrator_triples() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let integrator = IntegratorBundle::new("INT1".into(), Some(input), Some(output), 1.0, 0.0);

    let delta_t = 0.001; // 1ms time step
    let triples = integrator.triples(Some(&delta_t));

    assert_eq!(triples.len(), 2);
    let data = triples.data();

    // output[n] - dt * gain * input[n] = output[n-1]
    // [output_idx, output_idx] = 1
    // [output_idx, input_idx] = -dt * gain
    assert_eq!(data[0], (1, 1, 1.0)); // output->output = 1
    assert_eq!(data[1], (1, 0, -0.001)); // output->input = -dt*gain
}

#[test]
fn test_integrator_triples_with_gain() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let integrator = IntegratorBundle::new(
        "INT1".into(),
        Some(input),
        Some(output),
        2.0, // gain = 2
        0.0,
    );

    let delta_t = 0.001;
    let triples = integrator.triples(Some(&delta_t));

    let data = triples.data();
    assert_eq!(data[0], (1, 1, 1.0)); // output->output = 1
    assert_eq!(data[1], (1, 0, -0.002)); // output->input = -dt*gain = -0.001*2
}

#[test]
fn test_integrator_triples_no_delta_t() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let integrator = IntegratorBundle::new("INT1".into(), Some(input), Some(output), 1.0, 0.0);

    let triples = integrator.triples(None);
    assert_eq!(triples.len(), 0); // No delta_t means no triples
}

#[test]
fn test_integrator_triples_missing_input() {
    use crate::models::Unit;

    let output = Variable::new("out".into(), Unit::Volt, 1);

    let integrator = IntegratorBundle::new(
        "INT1".into(),
        None, // No input
        Some(output),
        1.0,
        0.0,
    );

    let delta_t = 0.001;
    let triples = integrator.triples(Some(&delta_t));
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_integrator_triples_missing_output() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);

    let integrator = IntegratorBundle::new(
        "INT1".into(),
        Some(input),
        None, // No output
        1.0,
        0.0,
    );

    let delta_t = 0.001;
    let triples = integrator.triples(Some(&delta_t));
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_integrator_pairs() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let mut integrator = IntegratorBundle::new(
        "INT1".into(),
        Some(input),
        Some(output),
        1.0,
        5.0, // initial condition = 5
    );

    // Simulate some integration by setting previous_output
    integrator.previous_output = 3.5;

    let x_vec = vec![0.0, 2.0, 0.0]; // Dummy solution vector
    let pairs = integrator.pairs(&x_vec);

    assert_eq!(pairs.len(), 1);
    let data = pairs.data();
    assert_eq!(data[0], (1, 3.5)); // RHS = previous_output
}

#[test]
fn test_integrator_pairs_no_output() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);

    let integrator = IntegratorBundle::new("INT1".into(), Some(input), None, 1.0, 0.0);

    let x_vec = vec![0.0, 2.0, 0.0];
    let pairs = integrator.pairs(&x_vec);
    assert_eq!(pairs.len(), 0);
}

#[test]
fn test_integrator_update_previous_output() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let mut integrator = IntegratorBundle::new("INT1".into(), Some(input), Some(output), 1.0, 0.0);

    // Initial previous_output should be initial_condition
    assert_eq!(integrator.previous_output(), 0.0);

    // Update with a solution vector where output is at index 1 with value 7.5
    let x_vec = vec![1.0, 7.5, 3.0];
    integrator.update_previous_output(&x_vec);

    assert_eq!(integrator.previous_output(), 7.5);
}

#[test]
fn test_integrator_reset() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let mut integrator = IntegratorBundle::new(
        "INT1".into(),
        Some(input),
        Some(output),
        1.0,
        10.0, // initial condition = 10
    );

    // Change previous_output
    integrator.previous_output = 5.0;
    assert_eq!(integrator.previous_output(), 5.0);

    // Reset should restore to initial condition
    integrator.reset();
    assert_eq!(integrator.previous_output(), 10.0);
}

#[test]
fn test_integrator_ac_triples() {
    use crate::models::Unit;

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let integrator = IntegratorBundle::new("INT1".into(), Some(input), Some(output), 2.0, 0.0);

    let freq = 1000.0; // 1kHz
    let ac_triples = integrator.ac_triples(freq);

    assert_eq!(ac_triples.len(), 1);
    let data = ac_triples.data();
    // Simplified AC model: just the gain
    assert_eq!(data[0].0, 1); // output_idx
    assert_eq!(data[0].1, 0); // input_idx
    assert_eq!(data[0].2.re, -2.0); // -gain
    assert_eq!(data[0].2.im, 0.0);
}

#[test]
fn test_integrator_simulation_step() {
    use crate::models::Unit;

    // Simulate a simple integration: ∫ 1 dt = t
    // With dt=0.1, we expect: 0, 0.1, 0.2, 0.3, ...

    let input = Variable::new("in".into(), Unit::Volt, 0);
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let mut integrator = IntegratorBundle::new("INT1".into(), Some(input), Some(output), 1.0, 0.0);

    let delta_t = 0.1;
    let mut expected_output = 0.0;

    for step in 0..10 {
        // Get triples for this time step
        let triples = integrator.triples(Some(&delta_t));
        assert_eq!(triples.len(), 2);

        // Get pairs (RHS) - should be previous output
        let x_vec = vec![0.0, expected_output];
        let pairs = integrator.pairs(&x_vec);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs.data()[0].1, expected_output);

        // Simulate solving: output = previous_output + dt * input
        // For constant input = 1: output[n] = output[n-1] + dt
        expected_output += delta_t;

        // Update previous output for next step
        integrator.previous_output = expected_output;

        assert!((integrator.previous_output() - (step as Numeric + 1.0) * delta_t).abs() < 1e-10);
    }
}
