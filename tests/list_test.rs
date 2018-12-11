#[macro_use]
extern crate rust_data_structrue;

use rust_data_structrue::list::*;

#[test]
fn test_base() {
    let l = List::<i32>::new();

    assert_eq!(l.len(), 0);
    assert!(l.empty());
}

#[test]
fn test_insert_remove() {
    let mut l = List::<i32>::new();

    l.insert(0, &1);
    assert_eq!(l[0], 1);
    l.insert(1, &7);
    assert_eq!(l[1], 7);
    l.remove(0, 2);
    assert!(l.empty());
}

#[test]
fn test_index() {
    let mut l = list![21, 5, 534, 0, 23];
    let arr = [21, 5, 534, 0, 23];

    for i in 0..l.len() {
        assert_eq!(arr[i], l[i]);
    }

    for i in 0..l.len() {
        l[i] = 1;
    }

    for i in 0..l.len() {
        assert_eq!(1, l[i]);
    }
}

#[test]
fn test_find() {
    let mut l = list![23, 63, 11, 40, 2346];

    if let Some(index) = l.find(&11, 0, 5) {
        assert_eq!(l[index], 11);
    } else {
        panic!("error in test_find!");
    }
}

#[test]
fn test_deduplicate() {
    let mut l = list![1234, 3, 3, 3, 6, 0, 54, 531, 213, 0, 0];
    let arr = [1234, 3, 6, 0, 54, 531, 213];

    l.deduplicate();
    assert_eq!(l.len(), arr.len());
    for i in 0..l.len() {
        assert_eq!(arr[i], l[i]);
    }
}

#[test]
fn test_map() {
    let mut l = list![1234, 3, 3, 3, 6, 0, 54, 531, 213, 0, 0];
    let arr = [1, 1, 1, 1, 1, 1, 54, 531, 213, 0, 0];
    let mut it = arr.iter();

    let _ = l.map(|_: &i32| 1, 0, 6)
             .iter().map(|x: &i32| {assert_eq!(it.next().unwrap(), x)});
}
