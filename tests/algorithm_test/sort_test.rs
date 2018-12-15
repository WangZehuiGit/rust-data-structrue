extern crate rust_algorithm;
extern crate rand;
use rust_algorithm::sort::*;

fn make_vec(size: usize) -> Vec<i32> {
    let mut a = Vec::new();

    for _ in 0..size {
        a.push(rand::random::<i32>());
    }

    a
}

fn do_test(sort_func: fn (&mut [i32], fn (&i32, &i32) -> std::cmp::Ordering), size: usize) {
    let mut a = make_vec(size);
    let mut b = a.clone();

    b.sort();
    sort_func(&mut a, |x, y| x.cmp(y));

    assert_eq!(b, a);
}

#[test]
fn test_bubble_sort() {
    do_test(bubble_sort, 5000);
}

#[test]
fn test_seletion_sort() {
    do_test(selection_sort, 5000);
}

#[test]
fn test_insertion_sort() {
    do_test(insertion_sort, 10000);
}

#[test]
fn test_shell_sort() {
    do_test(shell_sort, 50000);
}

#[test]
fn test_merge_sort() {
    do_test(merge_sort, 50000);
}

#[test]
fn test_quick_sort() {
    do_test(quick_sort, 50000);
}
