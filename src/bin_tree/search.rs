use super::height::{HeightBinNode, UpdateHeight};
use super::node::Node;
use super::{BinNode, BinTree, Iter, Ptr};
use std::cmp::Ordering;
use std::ptr::NonNull;

pub type BST<T> = BinarySearchTree<T, BinNode<T>>;
type SubTree<T> = BinTree<T, HeightBinNode<T>>;

pub struct BinarySearchTree<T: Ord, N: Node<T>> {
    bin_tree: BinTree<T, N>,
}

impl<T: Ord, N: Node<T>> BinarySearchTree<T, N> {
    pub fn new() -> Self {
        Self {
            bin_tree: BinTree::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.bin_tree.size()
    }

    pub fn search<'a, K: Copy, F>(&mut self, key: K, cmp: F) -> Option<&'a mut T>
    where
        F: Fn(K, &T) -> Ordering,
        Self: 'a,
    {
        if let Some(node) = self.search_node(key, cmp) {
            return unsafe { Some((*node.as_ptr()).get()) };
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
                    Ordering::Equal => {
                        return None;
                    }
                    Ordering::Less => {
                        node = parent.as_ref().lc();
                    }
                    Ordering::Greater => {
                        node = parent.as_ref().rc();
                    }
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
        N: 'a,
    {
        self.bin_tree.iter()
    }

    fn search_node<'a, K: Copy, F>(&mut self, key: K, cmp: F) -> Ptr<N>
    where
        F: Fn(K, &T) -> Ordering,
        Self: 'a,
    {
        let mut node = self.bin_tree.root();

        unsafe {
            while let Some(mut parent) = node {
                match cmp(key, parent.as_mut().get()) {
                    Ordering::Equal => {
                        return node;
                    }
                    Ordering::Less => {
                        node = parent.as_ref().lc();
                    }
                    Ordering::Greater => {
                        node = parent.as_ref().rc();
                    }
                }
            }
        }

        None
    }
}

pub struct AVLTree<T: Ord> {
    bst: BinarySearchTree<T, HeightBinNode<T>>,
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        Self {
            bst: BinarySearchTree::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.bst.size()
    }

    pub fn search<'a, K: Copy, F>(&mut self, key: K, cmp: F) -> Option<&'a mut T>
    where
        F: Fn(K, &T) -> Ordering,
        Self: 'a,
    {
        self.bst.search(key, cmp)
    }

    pub fn insert(&mut self, value: &T) -> Ptr<HeightBinNode<T>> {
        unsafe {
            let node = self.bst.insert(value);
            return self.balance(node);
        }
    }

    pub fn remove(&mut self, value: &T) -> Ptr<HeightBinNode<T>> {
        unsafe {
            let node = self.bst.remove(value);
            return self.balance(node);
        }
    }

    pub fn iter<'a>(&'a mut self) -> Iter<'a, T, HeightBinNode<T>>
    where
        T: 'a,
        HeightBinNode<T>: 'a,
    {
        self.bst.iter()
    }

    fn bal_fac(node: NonNull<HeightBinNode<T>>) -> usize {
        unsafe {
            let a = HeightBinNode::stature(node.as_ref().lc());
            let b = HeightBinNode::stature(node.as_ref().rc());

            if a < b {
                return b - a;
            }

            a - b
        }
    }

    fn is_avl_balanced(node: NonNull<HeightBinNode<T>>) -> bool {
        Self::bal_fac(node) < 2
    }

    fn taller_child(node: NonNull<HeightBinNode<T>>) -> Ptr<HeightBinNode<T>> {
        unsafe {
            match HeightBinNode::stature(node.as_ref().lc())
                .cmp(&HeightBinNode::stature(node.as_ref().rc()))
            {
                Ordering::Greater => node.as_ref().lc(),
                Ordering::Less => node.as_ref().rc(),
                Ordering::Equal => {
                    if node.as_ref().is_lc() {
                        node.as_ref().lc()
                    } else {
                        node.as_ref().rc()
                    }
                }
            }
        }
    }

    fn connect34(
        &mut self,
        a: SubTree<T>,
        mut b: SubTree<T>,
        c: SubTree<T>,
        t0: SubTree<T>,
        t1: SubTree<T>,
        t2: SubTree<T>,
        t3: SubTree<T>,
    ) -> SubTree<T> {
        let root = b.root().unwrap();
        let lc = b.attach_as_lc(root, a).unwrap();
        let rc = b.attach_as_rc(root, c).unwrap();

        if let Some(lc) = lc {
            b.attach_as_lc(lc, t0).unwrap();
            b.attach_as_rc(lc, t1).unwrap();
        }
        if let Some(rc) = rc {
            b.attach_as_lc(rc, t2).unwrap();
            b.attach_as_rc(rc, t3).unwrap();
        }

        b
    }

    fn secede(tree: &mut SubTree<T>, node: Ptr<HeightBinNode<T>>) -> SubTree<T> {
        if let Some(node) = node {
            return tree.secede(node);
        } else {
            return BinTree::new();
        }
    }

    fn balance_node(&mut self, node: NonNull<HeightBinNode<T>>) -> SubTree<T> {
        let node0 = Self::taller_child(node).unwrap();
        let node1 = Self::taller_child(node0).unwrap();
        let mut a: SubTree<T>;
        let mut b: SubTree<T>;
        let mut c: SubTree<T>;
        let t0: SubTree<T>;
        let t1: SubTree<T>;
        let t2: SubTree<T>;
        let t3: SubTree<T>;

        unsafe {
            if node0.as_ref().is_lc() {
                c = self.bst.bin_tree.secede(node);
                t3 = Self::secede(&mut c, node.as_ref().rc());
                if node1.as_ref().is_lc() {
                    b = c.secede(node0);
                    t2 = Self::secede(&mut b, node0.as_ref().rc());
                    a = b.secede(node1);
                    t0 = Self::secede(&mut a, node1.as_ref().lc());
                    t1 = Self::secede(&mut a, node1.as_ref().rc());
                } else {
                    a = c.secede(node0);
                    t0 = Self::secede(&mut a, node0.as_ref().lc());
                    b = a.secede(node1);
                    t1 = Self::secede(&mut b, node1.as_ref().lc());
                    t2 = Self::secede(&mut b, node1.as_ref().rc());
                }
            } else {
                a = self.bst.bin_tree.secede(node);
                t0 = Self::secede(&mut a, node.as_ref().lc());
                if node1.as_ref().is_lc() {
                    c = a.secede(node0);
                    t3 = Self::secede(&mut c, node0.as_ref().rc());
                    b = c.secede(node1);
                    t1 = Self::secede(&mut b, node1.as_ref().lc());
                    t2 = Self::secede(&mut b, node1.as_ref().rc());
                } else {
                    b = a.secede(node0);
                    t1 = Self::secede(&mut b, node0.as_ref().lc());
                    c = b.secede(node1);
                    t2 = Self::secede(&mut c, node1.as_ref().lc());
                    t3 = Self::secede(&mut c, node1.as_ref().rc());
                }
            }
        }

        return self.connect34(a, b, c, t0, t1, t2, t3);
    }

    unsafe fn balance(&mut self, node: Ptr<HeightBinNode<T>>) -> Ptr<HeightBinNode<T>> {
        if let Some(node) = node {
            let out = Some(node);
            let mut option = node.as_ref().parent();
            while let Some(node) = option {
                if !Self::is_avl_balanced(node) {
                    let is_lc = node.as_ref().is_lc();
                    let parent = node.as_ref().parent();
                    let balanced = self.balance_node(node);
                    if let Some(parent) = parent {
                        if is_lc {
                            self.bst
                                .bin_tree
                                .attach_as_lc(parent, balanced)
                                .expect(&format!("{:?} {:?}", parent.as_ref().lc(), Some(node)));
                        } else {
                            self.bst
                                .bin_tree
                                .attach_as_rc(parent, balanced)
                                .expect(&format!("{:?} {:?}", parent.as_ref().rc(), Some(node)));
                        }
                    } else {
                        self.bst.bin_tree = balanced;
                    }

                    return out;
                }

                option = node.as_ref().parent();
            }
        }

        None
    }
}
