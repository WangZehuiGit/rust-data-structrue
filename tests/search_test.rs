extern crate rust_data_structure;

use rust_data_structure::list::List;
use rust_data_structure::search::{binary_search, Ordered};

#[test]
fn test_binary_search() {
    let mut arr = [1234, 3, 3, 3, 6, 0, 54, 531, 213, 0, 0];
    let mut l = List::new();

    for n in arr.iter() {
        l.push(n);
    }

    for n in &mut arr {
        assert_eq!(binary_search(&mut l, *n, |a, b| a.cmp(b), 0, 11), Some(n));
    }

    assert_eq!(binary_search(&mut l, 777, |a, b| a.cmp(b), 0, 11), None);
}
