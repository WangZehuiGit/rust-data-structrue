extern crate rust_data_structure;
use rust_data_structure::bin_tree::search::{Search, AVLTree};

#[test]
fn test_avl() {
    let mut avl = AVLTree::<i32>::new();
    let mut arr = [1234, 3, 13, 23, 6, 30, 54, 531, 213, 40, 10];

    for n in arr.iter() {
        avl.insert(n);
    }

    for n in &mut arr {
        assert_eq!(avl.search(*n, |a, b| a.cmp(b)), Some(n));
    }

    assert_eq!(avl.search(777, |a, b| a.cmp(b)), None);
    avl.remove(&23);
    avl.remove(&2);
    assert_eq!(10, avl.size());
    assert_eq!(avl.search(23, |a, b| a.cmp(b)), None);
    assert_eq!(avl.search(6, |a, b| a.cmp(b)), Some(&mut 6));
}
