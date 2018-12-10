mod node;

use std::ptr::{self, NonNull};
use std::fmt::{self,Debug};
use std::marker::PhantomData;

use self::node::Node;

type Ptr<T> = Option<NonNull<T>>;
type NodePtr<T> = Ptr<BinNode<T>>;

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
    rc: Option<NonNull<BinNode<T>>>,
}

impl<T> BinNode<T> {
    
}

pub trait PubNode<T>: Sized {
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
}


impl<T> PubNode<T> for BinNode<T> {
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

impl<T> Node<T> for BinNode<T> {
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

pub struct BinTree<T, N: Node<T>> {
    root: Ptr<N>,
    size: usize,
    marker: PhantomData<T>
}

impl<T, N: Node<T>> BinTree<T, N> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size() == 0
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

    pub fn remove(&mut self, mut subtree: NonNull<N>) {
        unsafe {
            subtree.as_mut().set_parent(&None);
        }

        let size = self.size();
        self.size = size - N::remove_at(subtree.as_ptr());
    }
}

impl<T, N: Node<T>> Drop for BinTree<T, N> {
    fn drop(&mut self) {
        let root = self.root;

        if let Some(root) = root {
            N::remove_at(root.as_ptr());
        }
    }
}
