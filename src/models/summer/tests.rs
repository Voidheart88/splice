/// Tests for SummerBundle
use super::*;

#[test]
fn test_summer_creation() {
    let summer = SummerBundle::new("SUM1".into(), vec![], None, None);
    assert_eq!(summer.name(), "SUM1".into());
}

#[test]
fn test_summer_num_inputs() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
        Variable::new("in3".into(), Unit::Volt, 2),
    ];

    let summer = SummerBundle::new("SUM1".into(), inputs, None, None);
    assert_eq!(summer.num_inputs(), 3);
}

#[test]
fn test_summer_with_gains() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
    ];
    let gains = vec![2.0, 3.0];

    let summer = SummerBundle::new("SUM1".into(), inputs, None, Some(gains.clone()));
    assert_eq!(summer.gains(), &gains);
}

#[test]
fn test_summer_default_gains() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
    ];

    let summer = SummerBundle::new("SUM1".into(), inputs, None, None);
    assert_eq!(summer.gains().len(), 2);
    assert_eq!(summer.gains()[0], 1.0);
    assert_eq!(summer.gains()[1], 1.0);
}

#[test]
fn test_summer_input_output_indices() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
        Variable::new("in3".into(), Unit::Volt, 2),
    ];
    let output = Variable::new("out".into(), Unit::Volt, 3);

    let summer = SummerBundle::new("SUM1".into(), inputs, Some(output), None);

    assert_eq!(summer.input_indices(), vec![0, 1, 2]);
    assert_eq!(summer.output_idx(), Some(3));
}

#[test]
fn test_summer_triple_idx() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
    ];
    let output = Variable::new("out".into(), Unit::Volt, 2);

    let summer = SummerBundle::new("SUM1".into(), inputs, Some(output), None);

    let triple_idx = summer.triple_idx();
    assert!(triple_idx.is_some());
    let idx = triple_idx.unwrap();
    assert_eq!(idx.len(), 3); // output->output + 2 inputs
}

#[test]
fn test_summer_triples_empty() {
    let summer = SummerBundle::new("SUM1".into(), vec![], None, None);
    let triples = summer.triples();
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_summer_triples_no_output() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
    ];

    let summer = SummerBundle::new("SUM1".into(), inputs, None, None);
    let triples = summer.triples();
    assert_eq!(triples.len(), 0);
}

#[test]
fn test_summer_triples_single_input() {
    use crate::models::Unit;

    let inputs = vec![Variable::new("in1".into(), Unit::Volt, 0)];
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let summer = SummerBundle::new("SUM1".into(), inputs, Some(output), None);
    let triples = summer.triples();

    assert_eq!(triples.len(), 2);
    let data = triples.data();
    assert_eq!(data[0], (1, 1, 1.0)); // output->output = 1
    assert_eq!(data[1], (1, 0, -1.0)); // output->input = -gain
}

#[test]
fn test_summer_triples_multiple_inputs() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
        Variable::new("in3".into(), Unit::Volt, 2),
    ];
    let output = Variable::new("out".into(), Unit::Volt, 3);
    let gains = vec![2.0, 3.0, 4.0];

    let summer = SummerBundle::new("SUM1".into(), inputs, Some(output), Some(gains));
    let triples = summer.triples();

    assert_eq!(triples.len(), 4); // 1 output + 3 inputs
    let data = triples.data();
    assert_eq!(data[0], (3, 3, 1.0)); // output->output = 1
    assert_eq!(data[1], (3, 0, -2.0)); // output->in1 = -2.0
    assert_eq!(data[2], (3, 1, -3.0)); // output->in2 = -3.0
    assert_eq!(data[3], (3, 2, -4.0)); // output->in3 = -4.0
}

#[test]
fn test_summer_triples_equation() {
    // Verify the equation: output = gain1*in1 + gain2*in2 + gain3*in3
    // Rearranged: output - gain1*in1 - gain2*in2 - gain3*in3 = 0
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
    ];
    let output = Variable::new("out".into(), Unit::Volt, 2);
    let gains = vec![1.5, 2.5];

    let summer = SummerBundle::new("SUM1".into(), inputs, Some(output), Some(gains));
    let triples = summer.triples();

    let data = triples.data();
    // Equation: 1*out - 1.5*in1 - 2.5*in2 = 0
    // Or: out = 1.5*in1 + 2.5*in2
    assert_eq!(data[0], (2, 2, 1.0)); // coefficient of output
    assert_eq!(data[1], (2, 0, -1.5)); // coefficient of in1
    assert_eq!(data[2], (2, 1, -2.5)); // coefficient of in2
}

#[test]
fn test_summer_pairs() {
    use crate::models::Unit;

    let inputs = vec![Variable::new("in1".into(), Unit::Volt, 0)];
    let output = Variable::new("out".into(), Unit::Volt, 1);

    let summer = SummerBundle::new("SUM1".into(), inputs, Some(output), None);
    let x_vec = vec![1.0, 2.0];
    let pairs = summer.pairs(&x_vec);

    assert_eq!(pairs.len(), 0); // No constant term
}

#[test]
fn test_summer_ac_triples() {
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
    ];
    let output = Variable::new("out".into(), Unit::Volt, 2);
    let gains = vec![2.0, 3.0];

    let summer = SummerBundle::new("SUM1".into(), inputs, Some(output), Some(gains));
    let ac_triples = summer.ac_triples();

    assert_eq!(ac_triples.len(), 3);
    let data = ac_triples.data();
    assert_eq!(data[0].0, 2); // output_idx
    assert_eq!(data[0].1, 2); // output_idx
    assert_eq!(data[0].2.re, 1.0); // coefficient = 1
    assert_eq!(data[0].2.im, 0.0);

    assert_eq!(data[1].0, 2); // output_idx
    assert_eq!(data[1].1, 0); // in1_idx
    assert_eq!(data[1].2.re, -2.0); // -gain1
    assert_eq!(data[1].2.im, 0.0);

    assert_eq!(data[2].0, 2); // output_idx
    assert_eq!(data[2].1, 1); // in2_idx
    assert_eq!(data[2].2.re, -3.0); // -gain2
    assert_eq!(data[2].2.im, 0.0);
}

#[test]
fn test_summer_add_input() {
    use crate::models::Unit;

    let output = Variable::new("out".into(), Unit::Volt, 0);
    let mut summer = SummerBundle::new("SUM1".into(), vec![], Some(output), None);

    assert_eq!(summer.num_inputs(), 0);

    let input1 = Variable::new("in1".into(), Unit::Volt, 1);
    summer.add_input(input1, 2.5);

    assert_eq!(summer.num_inputs(), 1);
    assert_eq!(summer.gains()[0], 2.5);

    let input2 = Variable::new("in2".into(), Unit::Volt, 2);
    summer.add_input_unit_gain(input2);

    assert_eq!(summer.num_inputs(), 2);
    assert_eq!(summer.gains()[1], 1.0);
}

#[test]
fn test_summer_inverter() {
    // Test using summer as an inverter: output = -1 * input
    use crate::models::Unit;

    let inputs = vec![Variable::new("in".into(), Unit::Volt, 0)];
    let output = Variable::new("out".into(), Unit::Volt, 1);
    let gains = vec![-1.0];

    let summer = SummerBundle::new("INV1".into(), inputs, Some(output), Some(gains));
    let triples = summer.triples();

    let data = triples.data();
    // Equation: out - (-1)*in = 0  =>  out = -in
    assert_eq!(data[0], (1, 1, 1.0)); // output coefficient
    assert_eq!(data[1], (1, 0, 1.0)); // input coefficient (negative of -1)
}

#[test]
fn test_summer_weighted_sum() {
    // Test a weighted sum: output = 0.5*in1 + 0.3*in2 + 0.2*in3
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
        Variable::new("in3".into(), Unit::Volt, 2),
    ];
    let output = Variable::new("out".into(), Unit::Volt, 3);
    let gains = vec![0.5, 0.3, 0.2];

    let summer = SummerBundle::new("WSUM1".into(), inputs, Some(output), Some(gains));
    let triples = summer.triples();

    let data = triples.data();
    assert_eq!(data.len(), 4);
    assert_eq!(data[0], (3, 3, 1.0)); // output
    assert_eq!(data[1], (3, 0, -0.5)); // -0.5*in1
    assert_eq!(data[2], (3, 1, -0.3)); // -0.3*in2
    assert_eq!(data[3], (3, 2, -0.2)); // -0.2*in3
}

#[test]
fn test_summer_differential() {
    // Test differential: output = in1 - in2
    use crate::models::Unit;

    let inputs = vec![
        Variable::new("in1".into(), Unit::Volt, 0),
        Variable::new("in2".into(), Unit::Volt, 1),
    ];
    let output = Variable::new("out".into(), Unit::Volt, 2);
    let gains = vec![1.0, -1.0];

    let summer = SummerBundle::new("DIFF1".into(), inputs, Some(output), Some(gains));
    let triples = summer.triples();

    let data = triples.data();
    // Equation: out - 1*in1 - (-1)*in2 = 0  =>  out = in1 - in2
    assert_eq!(data[0], (2, 2, 1.0)); // output
    assert_eq!(data[1], (2, 0, -1.0)); // -1*in1
    assert_eq!(data[2], (2, 1, 1.0)); // -(-1)*in2 = +1*in2
}
