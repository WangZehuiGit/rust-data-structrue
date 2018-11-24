extern crate rust_data_structrue;

use rust_data_structrue::queue::*;

#[test]
fn test_base() {
    let q = QueueList::<i32>::new();

    assert_eq!(0, q.size());
    assert!(q.empty());
}

#[test]
fn test_enqueue_dequeue() {
    let mut q = QueueList::<i32>::new();

    q.enqueue(&34);
    assert_eq!(34, *q.front());
    q.enqueue(&3456);
    assert_eq!(2, q.size());
    assert_eq!(34, q.dequeue());
}
