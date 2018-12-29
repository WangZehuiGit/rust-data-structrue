extern crate rust_data_structure;
use rust_data_structure::bin_tree::search::BST;

#[test]
fn test_bst() {
    let mut bst = BST::<i32>::new();
    let mut arr = [1234, 3, 13, 23, 6, 30, 54, 531, 213, 40, 10];

    for n in arr.iter() {
        bst.insert(n);
    }

    for n in &mut arr {
        assert_eq!(bst.search(*n, |a, b| a.cmp(b)), Some(n));
    }

    assert_eq!(bst.search(777, |a, b| a.cmp(b)), None);
    bst.remove(&23);
    bst.remove(&2);
    assert_eq!(10, bst.size());
    assert_eq!(bst.search(23, |a, b| a.cmp(b)), None);
    assert_eq!(bst.search(6, |a, b| a.cmp(b)), Some(&mut 6));
}
