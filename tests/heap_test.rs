extern crate rust_data_structure;
use rust_data_structure::heap::*;
use std::cmp::Reverse;

#[test]
fn test_heap() {
    let mut h = Heap::<Reverse<i32>>::new();
    let mut arr = [1234, 3, 6, 0, 54, 531, 213];

    for n in arr.iter() {
        h.insert(&Reverse(*n));
    }
    assert_eq!(arr.len(), h.size());

    arr.sort();
    for n in arr.iter() {
        assert_eq!(*n, h.del_max().0);
    }
}
