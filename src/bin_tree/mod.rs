pub mod color;
pub mod height;
mod private;
pub mod search;

use super::malloc_val;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

type Ptr<T> = Option<NonNull<T>>;
type NodePtr<T> = Ptr<BinNode<T>>;
pub type RawBinTree<T> = BinTree<T, BinNode<T>>;

#[derive(Debug)]
pub struct InsertErr(&'static str);

#[derive(Clone, Copy)]
pub struct Iter<'a, T: 'a, N: 'a + private::Node<T>> {
    ptr: Ptr<N>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T: 'a, N: 'a + private::Node<T>> Iterator for Iter<'a, T, N> {
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

pub trait Node<T>: Sized {
    fn get(&mut self) -> &mut T;
    fn parent(&self) -> Ptr<Self>;
    fn lc(&self) -> Ptr<Self>;
    fn rc(&self) -> Ptr<Self>;

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

    fn is_rc(&self) -> bool {
        !self.is_lc() && !self.is_root()
    }

    fn has_lc(&self) -> bool {
        self.lc() != None
    }

    fn has_rc(&self) -> bool {
        self.rc() != None
    }

    fn is_leaf(&self) -> bool {
        !self.has_lc() && !self.has_rc()
    }

    fn has_double_branch(&self) -> bool {
        self.has_lc() && self.has_rc()
    }

    fn succ(&self) -> Ptr<Self> {
        let mut succ: Ptr<Self>;

        unsafe {
            if let Some(mut node) = self.rc() {
                succ = self.rc();
                while let Some(next) = node.as_ref().lc() {
                    succ = Some(next);
                    node = next;
                }
            } else {
                succ = None;

                if let Some(mut node) = self.parent() {
                    if self.is_lc() {
                        succ = Some(node);
                    } else {
                        while let Some(next) = node.as_ref().parent() {
                            node = next;

                            if node.as_ref().is_lc() {
                                succ = Some(next);
                                break;
                            }
                        }
                    }
                }
            }
        }

        return succ;
    }

    fn next(&self) -> Ptr<Self> {
        let mut succ: Ptr<Self>;

        unsafe {
            if let Some(_) = self.lc() {
                succ = self.lc();
            } else if let Some(_) = self.rc() {
                succ = self.rc();
            } else {
                succ = None;
                let mut node = self;

                while let Some(parent) = node.parent() {
                    if node.is_lc() {
                        if let Some(rc) = parent.as_ref().rc() {
                            succ = Some(rc);
                            break;
                        }
                    }
                    node = &*parent.as_ptr();
                }
            }
        }

        return succ;
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
}

impl<T> Node<T> for BinNode<T> {
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
}

impl<T> private::Node<T> for BinNode<T> {
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

pub struct BinTree<T, N: private::Node<T>> {
    root: Ptr<N>,
    size: usize,
    marker: PhantomData<T>,
}

impl<T, N: private::Node<T>> BinTree<T, N> {
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

impl<'a, T: 'a, N: 'a + private::Node<T>> BinTree<T, N> {
    pub fn iter(&'a mut self) -> Iter<'a, T, N> {
        Iter {
            ptr: self.root,
            marker: PhantomData,
        }
    }
}

impl<T, N: private::Node<T>> Drop for BinTree<T, N> {
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
