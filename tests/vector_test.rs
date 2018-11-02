extern crate rust_data_structrue;

use rust_data_structrue::vector;

#[test]
fn test_base() {
    let v = vector::new::<i32>();
    assert_eq!(v.capacity(), 8);
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

#[test]
fn test_insert() {
    let mut v = vector::from_slice(&[1231, 423, 12, 6]);
    v.insert(2, &8);
    assert_eq!(v[2], 8);
    assert_eq!(v[4], 6);
    assert_eq!(v.len(), 5);
    assert_eq!(v.capacity(), 8);
}

#[test]
fn test_remove() {
    let mut v = vector::from_slice(&[132, 5, 632, 90]);
    v.remove(1, 3);
    assert_eq!(v[1], 90);
    assert_eq!(v.len(), 2);
}

#[test]
fn test_eq() {
    let v = vector::from_slice(&[32, 5]);
    let o = v.clone();

    assert_eq!(v, o);
}