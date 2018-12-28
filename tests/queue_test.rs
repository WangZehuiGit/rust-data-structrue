extern crate rust_data_structure;

use rust_data_structure::queue::*;
use rust_data_structure::list::*;

#[test]
fn test_enqueue_dequeue() {
    let mut q = List::<i32>::new();

    q.enqueue(&34);
    assert_eq!(34, *q.front());
    q.enqueue(&3456);
    assert_eq!(2, q.len());
    assert_eq!(34, q.dequeue());
}
