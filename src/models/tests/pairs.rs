use crate::models::Pairs;

#[test]
fn test_pairs_creation() {
    let pairs = Pairs::new(&[(0, 1.0), (1, 2.0)]);
    assert_eq!(pairs.len(), 2);
    assert!(!pairs.is_empty());
}

#[test]
fn test_pairs_indexing() {
    let pairs = Pairs::new(&[(0, 1.0), (1, 2.0)]);
    assert_eq!(pairs[0], (0, 1.0));
    assert_eq!(pairs[1], (1, 2.0));
}

#[test]
#[should_panic(expected = "Index 2 out of bounds")]
fn test_pairs_index_out_of_bounds() {
    let pairs = Pairs::new(&[(0, 1.0), (1, 2.0)]);
    let _ = &pairs[2];
}

#[test]
fn test_pairs_iteration() {
    let pairs = Pairs::new(&[(0, 1.0), (1, 2.0), (2, 3.0)]);
    let mut iter = pairs.into_iter();
    
    assert_eq!(iter.next(), Some(&(0, 1.0)));
    assert_eq!(iter.next(), Some(&(1, 2.0)));
    assert_eq!(iter.next(), Some(&(2, 3.0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_pairs_size_hint() {
    let pairs = Pairs::new(&[(0, 1.0), (1, 2.0), (2, 3.0)]);
    let iter = pairs.into_iter();
    assert_eq!(iter.size_hint(), (3, Some(3)));
}

#[test]
fn test_pairs_exact_size() {
    let pairs = Pairs::new(&[(0, 1.0), (1, 2.0)]);
    let iter = pairs.into_iter();
    assert_eq!(iter.len(), 2);
}

#[test]
fn test_pairs_empty() {
    let pairs: Pairs<f64, 2> = Pairs::new(&[]);
    assert_eq!(pairs.len(), 0);
    assert!(pairs.is_empty());
    assert_eq!(pairs.into_iter().count(), 0);
}

#[test]
fn test_pairs_capacity() {
    let pairs = Pairs::new(&[(0, 1.0), (1, 2.0)]);
    // This should compile and work fine - capacity is 2, we're using 2 elements
    assert_eq!(pairs.len(), 2);
}

#[test]
#[should_panic(expected = "Initial data length exceeds the capacity N")]
fn test_pairs_capacity_exceeded() {
    // This should panic - trying to create Pairs with capacity 2 but 3 elements
    let _ = Pairs::<f64, 2>::new(&[(0, 1.0), (1, 2.0), (2, 3.0)]);
}
