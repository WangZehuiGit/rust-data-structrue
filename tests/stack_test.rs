extern crate rust_data_structure;

use rust_data_structure::stack::*;
use rust_data_structure::vector::*;

#[test]
fn test_pop_push() {
    let mut s = Vector::new();

    s.push(&43);
    assert_eq!(*s.top(), 43);
    s.push(&64);
    assert_eq!(s.len(), 2);
    assert_eq!(64, s.pop());
    assert_eq!(1, s.len());
}
