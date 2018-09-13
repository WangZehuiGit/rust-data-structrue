extern crate rust_data_structrue;

use rust_data_structrue::vector;

#[test]
fn test_capacity() {
    let v = vector::new::<i32>();
    if v.capacity() != 32 {
        panic!();
    }
}