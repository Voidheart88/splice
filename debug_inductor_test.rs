use std::sync::Arc;
use splice::models::inductor::InductorBundle;
use splice::spot::Variable;
use splice::Numeric;

fn main() {
    let node0 = Variable::new(Arc::from("Node0"), splice::Unit::Volt, 0);
    let node1 = Variable::new(Arc::from("Node1"), splice::Unit::Volt, 1);
    let mut inductor_bundle =
        InductorBundle::new(Arc::from("InductorBundle18"), Some(node0), Some(node1), 2.0);
    
    // Set a previous current
    inductor_bundle.update_previous_current(0.5);
    
    let delta_t = 0.01;
    let pairs = inductor_bundle.pairs(&delta_t);
    let expected_resistance = delta_t / 2.0; // delta_t / L
    let expected_rhs = expected_resistance * 0.5; // (delta_t/L) * I_prev
    
    println!("Expected resistance: {}", expected_resistance);
    println!("Expected RHS: {}", expected_rhs);
    println!("Actual pairs[0].1: {}", pairs[0].1);
    println!("Actual pairs[1].1: {}", pairs[1].1);
    println!("Pairs length: {}", pairs.len());
}