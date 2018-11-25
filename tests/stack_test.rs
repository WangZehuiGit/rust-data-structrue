extern crate rust_data_structrue;

use rust_data_structrue::stack::*;

#[test]
fn test_base() {
    let s = StackVector::<i32>::new();

    assert_eq!(0, s.size());
    assert!(s.empty());
}

#[test]
fn test_pop_push() {
    let mut s = StackVector::new();

    s.push(&43);
    assert_eq!(*s.top(), 43);
    s.push(&64);
    assert_eq!(s.size(), 2);
    assert_eq!(64, s.pop());
    assert_eq!(1, s.size());
}