use super::*;

// --------------------------------Triples Tests--------------------------------
#[test]
fn test_addition_no_overlap() {
    let triples1 = Triples::Vec(vec![(Row(1), Col(1), 1.0), (Row(2), Col(2), 2.0)]);
    let triples2 = Triples::Vec(vec![(Row(3), Col(3), 3.0), (Row(4), Col(4), 4.0)]);
    let expected = Triples::Vec(vec![
        (Row(1), Col(1), 1.0),
        (Row(2), Col(2), 2.0),
        (Row(3), Col(3), 3.0),
        (Row(4), Col(4), 4.0),
    ]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_with_overlap() {
    let triples1 = Triples::Vec(vec![(Row(1), Col(1), 1.0), (Row(2), Col(2), 2.0)]);
    let triples2 = Triples::Vec(vec![(Row(1), Col(1), 3.0), (Row(2), Col(2), 4.0)]);
    let expected = Triples::Vec(vec![(Row(1), Col(1), 4.0), (Row(2), Col(2), 6.0)]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_mixed() {
    let triples1 = Triples::Vec(vec![
        (Row(1), Col(1), 1.0),
        (Row(2), Col(2), 2.0),
        (Row(3), Col(3), 3.0),
    ]);
    let triples2 = Triples::Vec(vec![
        (Row(1), Col(1), 3.0),
        (Row(4), Col(4), 4.0),
        (Row(3), Col(3), 3.0),
    ]);
    let expected = Triples::Vec(vec![
        (Row(1), Col(1), 4.0),
        (Row(2), Col(2), 2.0),
        (Row(3), Col(3), 6.0),
        (Row(4), Col(4), 4.0),
    ]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_empty() {
    let triples1 = Triples::Vec(vec![]);
    let triples2 = Triples::Vec(vec![(Row(1), Col(1), 1.0), (Row(2), Col(2), 2.0)]);
    let expected = Triples::Vec(vec![(Row(1), Col(1), 1.0), (Row(2), Col(2), 2.0)]);

    assert_eq!(triples1 + triples2, expected);

    let triples1 = Triples::Vec(vec![(Row(1), Col(1), 1.0), (Row(2), Col(2), 2.0)]);
    let triples2 = Triples::Vec(vec![]);
    let expected = Triples::Vec(vec![(Row(1), Col(1), 1.0), (Row(2), Col(2), 2.0)]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_addition_both_empty() {
    let triples1 = Triples::Vec(vec![]);
    let triples2 = Triples::Vec(vec![]);
    let expected = Triples::Vec(vec![]);

    assert_eq!(triples1 + triples2, expected);
}

#[test]
fn test_len() {
    let single = Triples::Single((Row(1), Col(1), 1.0));
    assert_eq!(single.len(), 1);

    let double = Triples::Double([(Row(1), Col(1), 1.0), (Row(2), Col(2), 2.0)]);
    assert_eq!(double.len(), 2);

    let quad = Triples::Quad([
        (Row(1), Col(1), 1.0),
        (Row(2), Col(2), 2.0),
        (Row(3), Col(3), 3.0),
        (Row(4), Col(4), 4.0),
    ]);
    assert_eq!(quad.len(), 4);

    let vec_triples = Triples::Vec(vec![
        (Row(1), Col(1), 1.0),
        (Row(2), Col(2), 2.0),
        (Row(3), Col(3), 3.0),
    ]);
    assert_eq!(vec_triples.len(), 3);
}

// --------------------------------Doubles Tests--------------------------------

#[test]
fn test_doubles_addition_no_overlap() {
    let doubles1 = Doubles::from(vec![(Row(1), 1.0), (Row(2), 2.0)]);
    let doubles2 = Doubles::from(vec![(Row(3), 3.0), (Row(4), 4.0)]);
    let expected = Doubles::from(vec![
        (Row(1), 1.0),
        (Row(2), 2.0),
        (Row(3), 3.0),
        (Row(4), 4.0),
    ]);

    assert_eq!(doubles1 + doubles2, expected);
}

#[test]
fn test_doubles_addition_with_overlap() {
    let doubles1 = Doubles::from(vec![(Row(1), 1.0), (Row(2), 2.0)]);
    let doubles2 = Doubles::from(vec![(Row(1), 3.0), (Row(2), 4.0)]);
    let expected = Doubles::from(vec![(Row(1), 4.0), (Row(2), 6.0)]);

    assert_eq!(doubles1 + doubles2, expected);
}

#[test]
fn test_doubles_addition_mixed() {
    let doubles1 = Doubles::from(vec![(Row(1), 1.0), (Row(2), 2.0), (Row(3), 3.0)]);
    let doubles2 = Doubles::from(vec![(Row(1), 3.0), (Row(4), 4.0), (Row(3), 3.0)]);
    let expected = Doubles::from(vec![
        (Row(1), 4.0),
        (Row(2), 2.0),
        (Row(3), 6.0),
        (Row(4), 4.0),
    ]);

    assert_eq!(doubles1 + doubles2, expected);
}

#[test]
fn test_doubles_addition_empty() {
    let doubles1 = Doubles::from(vec![]);
    let doubles2 = Doubles::from(vec![(Row(1), 1.0), (Row(2), 2.0)]);
    let expected = Doubles::from(vec![(Row(1), 1.0), (Row(2), 2.0)]);

    assert_eq!(doubles1 + doubles2, expected);

    let doubles1 = Doubles::from(vec![(Row(1), 1.0), (Row(2), 2.0)]);
    let doubles2 = Doubles::from(vec![]);
    let expected = Doubles::from(vec![(Row(1), 1.0), (Row(2), 2.0)]);

    assert_eq!(doubles1 + doubles2, expected);
}

#[test]
fn test_doubles_addition_both_empty() {
    let doubles1 = Doubles::from(vec![]);
    let doubles2 = Doubles::from(vec![]);
    let expected = Doubles::from(vec![]);

    assert_eq!(doubles1 + doubles2, expected);
}
