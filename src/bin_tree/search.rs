use super::{BinNode, BinTree, Ptr, private, Iter, Node};
use super::height::{HeightBinNode, UpdateHeight};
use std::cmp::Ordering;
use std::ptr::NonNull;

pub type BST<T> = BinarySearchTree<T, BinNode<T>>;
type HBN<T> = HeightBinNode<T, BinNode<T>>;

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
        if let Some(node) = self.search_node(key, cmp) {
            return unsafe {Some((*node.as_ptr()).get())};
        }

        None
    }

    pub fn insert(&mut self, value: &T) -> Ptr<N> {
        if self.bin_tree.empty() {
            self.bin_tree.insert_as_root(value);
            return None;
        }

        let mut node = self.bin_tree.root();
        let mut point = node.unwrap();

        unsafe {
            while let Some(mut parent) = node {
                match value.cmp(parent.as_mut().get()) {
                    Ordering::Equal => {return None;},
                    Ordering::Less => {node = parent.as_ref().lc();},
                    Ordering::Greater => {node = parent.as_ref().rc();}
                }

                point = parent;
            }

            if value < point.as_mut().get() {
                return Some(self.bin_tree.insert_as_lc(point, value).unwrap());
            } else {
                return Some(self.bin_tree.insert_as_rc(point, value).unwrap());
            }
        }
    }

    pub fn remove(&mut self, value: &T) -> Ptr<N> {
        if self.bin_tree.empty() {
            return None;
        }

        let point = self.search_node(value, |a, b| a.cmp(b));

        unsafe {
            if let Some(mut point) = point {
                while point.as_ref().has_double_branch() {
                    let succ = point.as_ref().succ().unwrap();
                    N::swap(point, succ);
                    point = succ;
                }

                if point.as_ref().is_leaf() {
                    return self.bin_tree.remove(point);
                }

                let sub = if point.as_ref().has_lc() {
                    self.bin_tree.secede(point.as_ref().lc().unwrap())
                } else {
                    self.bin_tree.secede(point.as_ref().rc().unwrap())
                };

                if let Some(parent) = point.as_ref().parent() {
                    if point.as_ref().is_lc() {
                        let node = self.bin_tree.remove(point);
                        self.bin_tree.attach_as_lc(parent, sub).unwrap();
                        return node;
                    } else {
                        let node = self.bin_tree.remove(point);
                        self.bin_tree.attach_as_rc(parent, sub).unwrap();
                        return node;
                    }
                } else {
                    self.bin_tree = sub;
                    return None;
                }
            }

            None
        }
    }

    pub fn iter<'a>(&'a mut self) -> Iter<'a, T, N>
    where
        T: 'a,
        N: 'a
    {
        self.bin_tree.iter()
    }

    fn search_node<'a, K: Copy, F>(&mut self, key: K, cmp: F) -> Ptr<N>
    where
        F: Fn(K, &T) -> Ordering,
        Self: 'a
    {
        let mut node = self.bin_tree.root();

        unsafe {
            while let Some(mut parent) = node {
                match cmp(key, parent.as_mut().get()) {
                    Ordering::Equal => {return node;},
                    Ordering::Less => {node = parent.as_ref().lc();},
                    Ordering::Greater => {node = parent.as_ref().rc();}
                }
            }
        }

        None
    }
}

pub struct AVLTree<T: Ord> {
    bst: BinarySearchTree<T, HBN<T>>
}

impl<T: Ord> AVLTree<T> {
    fn is_balanced(node: NonNull<HBN<T>>) -> bool {
        unsafe {
            HBN::stature(node.as_ref().lc()) == HBN::stature(node.as_ref().rc())
        }
    }

    fn bal_fac(node: NonNull<HBN<T>>) -> usize {
        unsafe {
            let a = HBN::stature(node.as_ref().lc());
            let b = HBN::stature(node.as_ref().rc());

            if a < b {
                return b - a;
            }

            a - b
        }
    }

    fn is_avl_balanced(node: NonNull<HBN<T>>) -> bool {
        Self::bal_fac(node) < 2
    }

    fn taller_child(node: NonNull<HBN<T>>) -> Ptr<HBN<T>> {
        unsafe {
            match HBN::stature(node.as_ref().lc()).cmp(&HBN::stature(node.as_ref().rc())) {
                Ordering::Greater => node.as_ref().lc(),
                Ordering::Less => node.as_ref().rc(),
                Ordering::Equal => if node.as_ref().is_lc() {
                    node.as_ref().lc()
                } else {
                    node.as_ref().rc()
                }
            }
        }
    }
}
