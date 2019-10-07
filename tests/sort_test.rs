extern crate rand;
extern crate rust_data_structure;

//use rust_data_structure::sort::Sort;
use rust_data_structure::vector::{Sort, Vector};

fn make_vec(size: usize) -> Vector<i32> {
    let mut a = Vector::new();

    for _ in 0..size {
        let len = a.len();
        a.insert(len, &rand::random::<i32>());
    }

    a
}

fn do_test(size: usize) -> (Vector<i32>, Vector<i32>) {
    let a = make_vec(size);
    let mut b = a.clone();

    b.sort();

    (a, b)
}

#[test]
fn test_bubble_sort() {
    let (mut a, b) = do_test(5000);
    a.bubble_sort(|a, b| a.cmp(b));
    assert_eq!(a, b);
}

#[test]
fn test_selection_sort() {
    let (mut a, b) = do_test(5000);
    a.selection_sort(|a, b| a.cmp(b));
    assert_eq!(a, b);
}

#[test]
fn test_insertion_sort() {
    let (mut a, b) = do_test(10000);
    a.insertion_sort(|a, b| a.cmp(b));
    assert_eq!(a, b);
}

#[test]
fn test_shell_sort() {
    let (mut a, b) = do_test(10000);
    a.shell_sort(|a, b| a.cmp(b));
    assert_eq!(a, b);
}

#[test]
fn test_merge_sort() {
    let (mut a, b) = do_test(10000);
    a.merge_sort(|a, b| a.cmp(b));
    assert_eq!(a, b);
}

#[test]
fn test_quick_sort() {
    let (mut a, b) = do_test(10000);
    a.quick_sort(|a, b| a.cmp(b));
    assert_eq!(a, b);
}
