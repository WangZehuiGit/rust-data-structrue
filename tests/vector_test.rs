extern crate rust_data_structrue;

use rust_data_structrue::vector;

#[test]
fn test_base() {
    let v = vector::new::<i32>();
    assert_eq!(v.capacity(), 32);
    assert_eq!(v.len(), 0);
    assert_eq!(v.empty(), true);
}
