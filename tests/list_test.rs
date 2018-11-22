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
    assert_eq!(l.first().unwrap().data, 1);
    l.insert(1, &7);
    assert_eq!(l.last().unwrap().data, 7);
    l.remove(0, 2);
    assert!(l.empty());
}

#[test]
fn test_iter() {
    let mut l = list![21, 5, 534, 0, 23];
    let arr = [21, 5, 534, 0, 23];

    for (i, n) in l.iter().enumerate() {
        assert_eq!(arr[i], *n);
    }

    for n in l.iter() {
        *n = 1;
    }

    for n in l.iter() {
        assert_eq!(1, *n);
    }
}
