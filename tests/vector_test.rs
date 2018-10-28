extern crate rust_data_structrue;

use rust_data_structrue::vector;

#[test]
fn test_base() {
    let v = vector::new::<i32>();
    assert_eq!(v.capacity(), 32);
    assert_eq!(v.len(), 0);
    assert_eq!(v.empty(), true);
}

#[test]
fn test_index() {
    let mut v = vector::from_slice(&[1, 2, 3, 4]);
    assert_eq!(v[3], 4);
    v[2] = 4;
    assert_eq!(v[2], 4);
}

#[test]
fn test_find() {
    let v = vector::from_slice(&[1,34,354,23]);
    let i = v.find(&354);
    
    assert_eq!(i, Some(2));
}