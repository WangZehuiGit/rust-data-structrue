extern crate rust_data_structrue;

use rust_data_structrue::stack::*;
use rust_data_structrue::vector::*;

#[test]
fn test_base() {
    let s = Vector::<i32>::new();

    assert_eq!(0, s.size());
    assert!(s.empty());
}

#[test]
fn test_pop_push() {
    let mut s = Vector::new();

    s.push(&43);
    assert_eq!(*s.top(), 43);
    s.push(&64);
    assert_eq!(s.size(), 2);
    assert_eq!(64, s.pop());
    assert_eq!(1, s.size());
}
