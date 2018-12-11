mod private;
pub mod height;
pub mod color;

use super::malloc_val;
use std::ptr::{self, NonNull};
use std::fmt::{self,Debug};
use std::ops::Fn;
use std::marker::PhantomData;

type Ptr<T> = Option<NonNull<T>>;
type NodePtr<T> = Ptr<BinNode<T>>;
pub type RawBinTree<T> = BinTree<T, BinNode<T>>;

pub struct InsertErr(&'static str);

impl Debug for InsertErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let InsertErr(s) = self;

        fmt::Debug::fmt(s, f)
    }
}

pub struct BinNode<T> {
    pub data: T,
    parent: Option<NonNull<BinNode<T>>>,
    lc: Option<NonNull<BinNode<T>>>,
    rc: Option<NonNull<BinNode<T>>>
}

pub trait Node<T>: Sized {
    fn get(&mut self) -> &mut T;
    fn parent(&self) -> Ptr<Self>;
    fn lc(&self) -> Ptr<Self>;
    fn rc(&self) -> Ptr<Self>;

    fn is_root(&self) -> bool {
        self.parent() == None
    }

    fn is_lc(&self) -> bool {
        let self_ptr = self as *const Self;

        unsafe {
            if let Some(node) = self.parent() {
                return self_ptr == node.as_ref().lc().unwrap().as_ptr();
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

    fn map<F: Copy, R>(node: Ptr<Self>, func: F) -> Box<Vec<R>>
    where
        F: Fn(&T) -> R
    {
        let mut r = Box::new(Vec::<R>::new());

        unsafe {
            if let Some(mut node) = node {
                r.push(func(node.as_mut().get()));
                r.append(&mut Self::map(node.as_ref().lc(), func));
                r.append(&mut Self::map(node.as_ref().rc(), func));
            }
        }

        r
    }

    fn size_of(subtree: NonNull<Self>) -> usize {
        Self::map(Some(subtree), &|_: &T| ()).len()
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
            data: unsafe {ptr::read(value)},
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
    marker: PhantomData<T>
}

impl<T, N: private::Node<T>> BinTree<T, N> {
    pub fn new() -> Self {
        BinTree {
            root: None,
            size: 0,
            marker: PhantomData
        }
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

    pub fn insert_as_lc(&mut self, mut ptr: NonNull<N>, value: &T) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            self.size += 1;

            ptr.as_mut().insert_as_lc(value)?;

            Ok(ptr.as_mut().lc().unwrap())
        }
    }

    pub fn insert_as_rc(&mut self, mut ptr: NonNull<N>, value: &T) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            self.size += 1;

            ptr.as_mut().insert_as_rc(value)?;

            Ok(ptr.as_mut().rc().unwrap())
        }
    }

    pub fn attach_as_lc (
        &mut self,
        mut node: NonNull<N>,
        subtree: Self
    ) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            node.as_mut().set_lc(&subtree.root)?;

            if let Some(mut root) = subtree.root {
                self.size += N::size_of(root);
                root.as_mut().set_parent(&Some(node));
            }

            Ok(node.as_ref().lc().unwrap())
        }
    }

    pub fn attach_as_rc (
        &mut self,
        mut node: NonNull<N>,
        subtree: Self
    ) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            node.as_mut().set_rc(&subtree.root)?;

            if let Some(mut root) = subtree.root {
                self.size += N::size_of(root);
                root.as_mut().set_parent(&Some(node));
            }

            Ok(node.as_ref().rc().unwrap())
        }
    }

    pub fn remove(&mut self, mut subtree: NonNull<N>) {
        unsafe {
            subtree.as_mut().set_parent(&None);
        }

        let size = self.size();
        self.size = size - N::remove_at(subtree.as_ptr());
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
            marker: PhantomData
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
