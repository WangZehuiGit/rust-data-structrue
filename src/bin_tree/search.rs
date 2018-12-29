use super::{BinNode, BinTree, Ptr, private, Iter};
use std::cmp::Ordering;

pub type BST<T> = BinarySearchTree<T, BinNode<T>>;

pub struct BinarySearchTree<T: Ord, N: private::Node<T>> {
    bin_tree: BinTree<T, N>
}

impl<T: Ord, N: private::Node<T>> BinarySearchTree<T, N> {
    pub fn new() -> Self {
        Self {
            bin_tree: BinTree::new()
        }
    }

    pub fn size(&self) -> usize {
        self.bin_tree.size()
    }

    pub fn search<'a, K: Copy, F>(&mut self, key: K, cmp: F) -> Option<&'a mut T>
    where
        F: Fn(K, &T) -> Ordering,
        Self: 'a
    {
        let mut node = self.bin_tree.root();

        unsafe {
            while let Some(mut parent) = node {
                match cmp(key, parent.as_mut().get()) {
                    Ordering::Equal => {return Some((*parent.as_ptr()).get());},
                    Ordering::Less => {node = parent.as_ref().lc();},
                    Ordering::Greater => {node = parent.as_ref().rc();}
                }
            }
        }

        None
    }

    pub fn insert(&mut self, value: &T) {
        if self.bin_tree.empty() {
            self.bin_tree.insert_as_root(value);
            return;
        }

        let mut node = self.bin_tree.root();
        let mut point = node.unwrap();

        unsafe {
            while let Some(mut parent) = node {
                match value.cmp(parent.as_mut().get()) {
                    Ordering::Equal => {return;},
                    Ordering::Less => {node = parent.as_ref().lc();},
                    Ordering::Greater => {node = parent.as_ref().rc();}
                }

                point = parent;
            }

            if value < point.as_mut().get() {
                self.bin_tree.insert_as_lc(point, value).unwrap();
            } else {
                self.bin_tree.insert_as_rc(point, value).unwrap();
            }
        }
    }

    pub fn remove(&mut self, value: &T) {
        if self.bin_tree.empty() {
            return;
        }

        let mut node = self.bin_tree.root();
        let mut point: Ptr<N> = None;

        unsafe {
            while let Some(mut parent) = node {
                match value.cmp(parent.as_mut().get()) {
                    Ordering::Equal => {point = node; break;},
                    Ordering::Less => {node = parent.as_ref().lc();},
                    Ordering::Greater => {node = parent.as_ref().rc();}
                }
            }

            if let Some(mut point) = point {
                while point.as_ref().has_double_branch() {
                    let succ = point.as_ref().succ().unwrap();
                    N::swap(point, succ);
                    point = succ;
                }

                if point.as_ref().is_leaf() {
                    self.bin_tree.remove(point);
                    return
                }

                let sub = if point.as_ref().has_lc() {
                    self.bin_tree.secede(point.as_ref().lc().unwrap())
                } else {
                    self.bin_tree.secede(point.as_ref().rc().unwrap())
                };

                if let Some(parent) = point.as_ref().parent() {
                    if point.as_ref().is_lc() {
                        self.bin_tree.remove(point);
                        self.bin_tree.attach_as_lc(parent, sub).unwrap();
                    } else {
                        self.bin_tree.remove(point);
                        self.bin_tree.attach_as_rc(parent, sub).unwrap();
                    }
                } else {
                    self.bin_tree = sub;
                }
            }
        }
    }

    pub fn iter<'a>(&'a mut self) -> Iter<'a, T, N>
    where
        T: 'a,
        N: 'a
    {
        self.bin_tree.iter()
    }
}
