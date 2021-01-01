mod color;
mod height;
mod node;
pub mod search;

use super::utility::{free, malloc_val};
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

type Ptr<T> = Option<NonNull<T>>;
type NodePtr<T> = Ptr<BinNode<T>>;
pub type RawBinTree<T> = BinTree<T, BinNode<T>>;

pub trait Node<T>: Sized {
    fn get(&mut self) -> &mut T;
    fn parent(&self) -> Ptr<Self>;
    fn lc(&self) -> Ptr<Self>;
    fn rc(&self) -> Ptr<Self>;
    fn insert_lc(&mut self);
    fn insert_rc(&mut self);

    fn swap(mut a: NonNull<Self>, mut b: NonNull<Self>) {
        unsafe {
            ptr::swap(a.as_mut().get(), b.as_mut().get());
        }
    }

    fn is_root(&self) -> bool {
        self.parent() == None
    }

    fn is_lc(&self) -> bool {
        let self_ptr = self as *const Self;

        unsafe {
            if let Some(node) = self.parent() {
                if let Some(lc) = node.as_ref().lc() {
                    return self_ptr == lc.as_ptr();
                }
            }

            false
        }
    }
    
    fn has_lc(&self) -> bool {
        self.lc() != None
    }

    fn has_rc(&self) -> bool {
        self.rc() != None
    }

    fn is_rc(&self) -> bool {
        !self.is_lc() && !self.is_root()
    }
    fn is_leaf(&self) -> bool {
        !self.has_lc() && !self.has_rc()
    }

    fn has_double_branch(&self) -> bool {
        self.has_lc() && self.has_rc()
    }

    fn succ(&self) -> Ptr<Self> {
        if let Some(mut succ) = self.rc() {
            while let Some(next) = unsafe {succ.as_ref().lc()} {
                succ = next;
            }
            return Some(succ);
        } else {
            if self.is_lc() {
                return self.parent();
            } else {
                if let Some(mut child) = self.parent() {
                    if unsafe {child.as_ref().is_lc()} {
                        return unsafe {child.as_ref().parent()};
                    }
                    while let Some(next) = unsafe {child.as_ref().parent()} {
                        child = next;
                        if unsafe {child.as_ref().is_lc()} {
                            return unsafe {child.as_ref().parent()};
                        }
                    }
                    return None;
                } else {
                    return None;
                }
            }
        }
    }

    fn next(&self) -> Ptr<Self> {
        if self.has_lc() {
            return self.lc();
        } else if self.has_rc() {
            return self.rc();
        } else {
            if let Some(mut child) = self.parent() {
                if self.is_lc() {
                    return unsafe {child.as_ref().rc()};
                }
                while let Some(next) = unsafe {child.as_ref().parent()} {
                    if unsafe {child.as_ref().is_lc()} {
                        return unsafe {next.as_ref().rc()};
                    }
                    child = next;
                }
                return None;
            } else {
                return None;
            }
        }
    }
    
    fn size_of(subtree: NonNull<Self>) -> usize {
        let mut size = 1;

        unsafe {
            if let Some(lc) = subtree.as_ref().lc() {
                size += Self::size_of(lc);
            }
            if let Some(rc) = subtree.as_ref().rc() {
                size += Self::size_of(rc);
            }
        }

        size
    }

    fn remove_at(subtree: *mut Self) -> usize {
        let mut size = 1;

        unsafe {
            if let Some(lc) = (*subtree).lc() {
                size += Self::remove_at(lc.as_ptr());
            }
            if let Some(rc) = (*subtree).rc() {
                size += Self::remove_at(rc.as_ptr());
            }
        }

        free(subtree, 1).unwrap();

        size
    }
}

#[derive(Debug)]
pub struct InsertErr(&'static str);

#[derive(Clone, Copy)]
pub struct Iter<'a, T: 'a, N: 'a + node::Node<T>> {
    ptr: Ptr<N>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T: 'a, N: 'a + node::Node<T>> Iterator for Iter<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        unsafe {
            let next_ptr: Ptr<N>;
            let value: &mut T;

            if let Some(ptr) = self.ptr {
                next_ptr = ptr.as_ref().next();
                value = (*ptr.as_ptr()).get();
            } else {
                return None;
            }

            self.ptr = next_ptr;
            Some(value)
        }
    }
}

pub struct BinNode<T> {
    pub data: T,
    parent: Option<NonNull<BinNode<T>>>,
    lc: Option<NonNull<BinNode<T>>>,
    rc: Option<NonNull<BinNode<T>>>,
}

impl<T> node::Node<T> for BinNode<T> {
    fn get(&mut self) -> &mut T {
        &mut self.data
    }

    fn parent(&self) -> NodePtr<T> {
        self.parent
    }

    fn lc(&self) -> NodePtr<T> {
        self.lc
    }

    fn rc(&self) -> NodePtr<T> {
        self.rc
    }

    fn new(value: &T, parent: NodePtr<T>) -> Self {
        BinNode {
            data: unsafe { ptr::read(value) },
            parent: parent,
            lc: None,
            rc: None,
        }
    }

    fn set_parent(&mut self, value: &NodePtr<T>) {
        let is_lc = self.is_lc();

        unsafe {
            if let Some(mut parent) = self.parent {
                if is_lc {
                    parent.as_mut().lc = None;
                } else {
                    parent.as_mut().rc = None;
                }
            }

            self.parent = ptr::read(value);
        }
    }

    fn set_lc(&mut self, value: &NodePtr<T>) -> Result<(), InsertErr> {
        unsafe {
            if let None = self.lc {
                self.lc = ptr::read(value);
                return Ok(());
            } else {
                Err(InsertErr("left child is not none"))
            }
        }
    }

    fn set_rc(&mut self, value: &NodePtr<T>) -> Result<(), InsertErr> {
        unsafe {
            if let None = self.rc {
                self.rc = ptr::read(value);
                return Ok(());
            } else {
                Err(InsertErr("right child is not none"))
            }
        }
    }
}

pub struct BinTree<T, N: node::Node<T>> {
    root: Ptr<N>,
    size: usize,
    marker: PhantomData<T>,
}

impl<T, N: node::Node<T>> BinTree<T, N> {
    pub fn new() -> Self {
        BinTree {
            root: None,
            size: 0,
            marker: PhantomData,
        }
    }

    pub fn root(&self) -> Ptr<N> {
        self.root
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert_as_root(&mut self, value: &T) {
        self.size = 1;
        self.root = NonNull::new(malloc_val(&N::new(value, None)));
    }

    pub fn insert_as_lc(
        &mut self,
        mut ptr: NonNull<N>,
        value: &T,
    ) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            self.size += 1;

            ptr.as_mut().insert_as_lc(value)?;

            Ok(ptr.as_mut().lc().unwrap())
        }
    }

    pub fn insert_as_rc(
        &mut self,
        mut ptr: NonNull<N>,
        value: &T,
    ) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            self.size += 1;

            ptr.as_mut().insert_as_rc(value)?;

            Ok(ptr.as_mut().rc().unwrap())
        }
    }

    pub fn attach_as_lc(
        &mut self,
        mut node: NonNull<N>,
        subtree: Self,
    ) -> Result<Ptr<N>, InsertErr> {
        unsafe {
            node.as_mut().set_lc(&subtree.root)?;

            if let Some(mut root) = subtree.root {
                self.size += N::size_of(root);
                root.as_mut().set_parent(&Some(node));
            }

            Ok(node.as_ref().lc())
        }
    }

    pub fn attach_as_rc(
        &mut self,
        mut node: NonNull<N>,
        subtree: Self,
    ) -> Result<Ptr<N>, InsertErr> {
        unsafe {
            node.as_mut().set_rc(&subtree.root)?;

            if let Some(mut root) = subtree.root {
                self.size += N::size_of(root);
                root.as_mut().set_parent(&Some(node));
            }

            Ok(node.as_ref().rc())
        }
    }

    pub fn remove(&mut self, mut subtree: NonNull<N>) -> Ptr<N> {
        let parent: Ptr<N>;

        unsafe {
            parent = subtree.as_ref().parent();
            subtree.as_mut().set_parent(&None);
        }

        let size = self.size();
        self.size = size - N::remove_at(subtree.as_ptr());

        parent
    }

    pub fn secede(&mut self, mut node: NonNull<N>) -> Self {
        let size = N::size_of(node);
        self.size -= size;

        unsafe {
            node.as_mut().set_parent(&None);
        }

        Self {
            root: Some(node),
            size: size,
            marker: PhantomData,
        }
    }
}

impl<'a, T: 'a, N: 'a + node::Node<T>> BinTree<T, N> {
    pub fn iter(&'a mut self) -> Iter<'a, T, N> {
        Iter {
            ptr: self.root,
            marker: PhantomData,
        }
    }
}

impl<T, N: node::Node<T>> Drop for BinTree<T, N> {
    fn drop(&mut self) {
        if let Some(root) = self.root {
            unsafe {
                if root.as_ref().parent() == None {
                    N::remove_at(root.as_ptr());
                }
            }
        }
    }
}
