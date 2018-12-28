extern crate rust_data_structure;

use rust_data_structure::bin_tree::{RawBinTree, InsertErr};

#[test]
fn test_base() -> Result<(), InsertErr> {
    let mut t = RawBinTree::<char>::new();
    let arr = ['a', 'b', 'c'];

    assert!(t.empty());
    t.insert_as_root(&'a');
    let root = t.root().unwrap();
    t.insert_as_lc(root, &'b')?;
    t.insert_as_rc(root, &'c')?;

    assert_eq!(t.iter().count(), 3);
    assert_eq!(t.size(), 3);

    for (a, b) in t.iter().zip(arr.iter()) {
        assert_eq!(a, b);
    }

    Ok(())
}

#[test]
fn test_attach() -> Result<(), InsertErr> {
    let mut t = RawBinTree::<char>::new();
    let arr = ['a', 'c', 'b'];
    t.insert_as_root(&'a');
    let root = t.root().unwrap();
    let l = t.insert_as_lc(root, &'b').unwrap();
    let r = t.insert_as_rc(root, &'c').unwrap();
    let l = t.secede(l);
    let r = t.secede(r);
    t.attach_as_lc(root, r)?;
    t.attach_as_rc(root, l)?;
    
    assert_eq!(t.iter().count(), 3);
    assert_eq!(t.size(), 3);

    for (a, b) in t.iter().zip(arr.iter()) {
        assert_eq!(a, b);
    }

    Ok(())
}
