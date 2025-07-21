use num::One;

use crate::spot::*;
use crate::models::Triples;


#[test]
fn init_one_triple() {
    let triplet = (0,0,Numeric::one());
    let triples = Triples::new(&[triplet]);
    
    assert_eq!(triples.len(), 1);
    assert_eq!(triples.data(), [(0,0,Numeric::one())]);
}

#[test]
fn init_two_triple() {
    let triplet_1 = (0,0,Numeric::one());
    let triplet_2 = (1,1,Numeric::one()+Numeric::one());
    let triples = Triples::new(&[triplet_1,triplet_2]);
    
    assert_eq!(triples.len(), 2);
    assert_eq!(triples.data(), [(0,0,Numeric::one()),(0,0,Numeric::one()+Numeric::one())]);
}

#[test]
fn init_complex_two_triple() {
    let triplet_1 = (0,0,ComplexNumeric::one());
    let triplet_2 = (1,1,ComplexNumeric::one()+ComplexNumeric::one());
    let triples = Triples::new(&[triplet_1,triplet_2]);
    
    assert_eq!(triples.len(), 2);
    assert_eq!(triples.data(), [(0,0,ComplexNumeric::one()),(0,0,ComplexNumeric::one()+ComplexNumeric::one())]);
}

#[test]
fn iterate_over_triples() {
    let triplet_1 = (0,0,Numeric::one());
    let triplet_2 = (1,1,Numeric::one()+Numeric::one());
    let triples: Triples<f64, 2> = Triples::new(&[triplet_1,triplet_2]);
    
    let values: Vec<&(usize,usize,Numeric)> = triples.iter().map(|val| val).collect();
    
    assert_eq!(values[0], &triplet_1);
    assert_eq!(values[1], &triplet_2);
}

#[test]
fn flat_iterate_over_triples() {
    let triplet_1 = (0,0,Numeric::one());
    let triplet_2 = (1,1,Numeric::one()+Numeric::one());
    let triples_1: Triples<f64, 2> = Triples::new(&[triplet_1,triplet_2]);
    let triples_2: Triples<f64, 2> = Triples::new(&[triplet_2,triplet_1]);
    
    let triples_vec = vec!(triples_1,triples_2);
    
    let values: Vec<&(usize,usize,Numeric)> = triples_vec
        .iter()
        .flat_map(|triples| triples.iter())
        .map(|val| val)
        .collect();
    
    assert_eq!(values[0], &triplet_1);
    assert_eq!(values[1], &triplet_2);
    assert_eq!(values[2], &triplet_2);
    assert_eq!(values[3], &triplet_1);
}