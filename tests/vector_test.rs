extern crate rust_data_structrue;
use rust_data_structrue::vector::*;

#[test]
fn test_base() {
    let v = Vector::<i32>::new();
    assert_eq!(v.capacity(), 8);
    assert_eq!(v.len(), 0);
    assert_eq!(v.empty(), true);
}

#[test]
fn test_index() {
    let mut v = Vector::from_slice(&[1, 2, 3, 4]);
    assert_eq!(v[3], 4);
    v[2] = 4;
    assert_eq!(v[2], 4);
}

#[test]
fn test_iter() {
    let mut l = Vector::from_slice(&[1234, 3, 3, 3, 6, 0, 54, 531, 213, 0, 0]);
    let arr = [1, 1, 1, 1, 1, 1, 54, 531, 213, 0, 0];

    for n in l.iter().take(6) {
        *n = 1;
    }

    for (n1, n2) in l.iter().zip(arr.iter()) {
        assert_eq!(n1, n2);
    }

    for (n1, n2) in l.iter().rev().zip(arr.iter().rev()) {
        assert_eq!(n1, n2);
    }
}

#[test]
fn test_find() {
    let v = Vector::from_slice(&[1,34,354,23]);
    let i = v.find(&354);
    
    assert_eq!(i, Some(2));
}

#[test]
fn test_insert() {
    let mut v = Vector::from_slice(&[1231, 423, 12, 6]);
    v.insert(2, &8);
    assert_eq!(v[2], 8);
    assert_eq!(v[4], 6);
    assert_eq!(v.len(), 5);
    assert_eq!(v.capacity(), 8);
}

#[test]
fn test_remove() {
    let mut v = Vector::from_slice(&[132, 5, 632, 90, 666]);
    v.remove(1, 3);
    assert_eq!(v[1], 90);
    assert_eq!(v.len(), 3);
}

#[test]
fn test_eq() {
    let v = Vector::from_slice(&[32, 5]);
    let o = v.clone();

    assert_eq!(v, o);
}

#[test]
fn test_deref() {
    let mut v = Vector::from_slice(&[324, 53, 6, 36]);
    let o = Vec::from(&[324, 53, 6, 36][..]);

    assert_eq!(*v, *o);
    (*v)[3] = 9;
    assert_eq!(v[3], 9);
}
