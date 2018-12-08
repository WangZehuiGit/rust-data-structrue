use std::ptr::{self, NonNull};
use std::cmp::max;
use std::ops::Drop;
use std::fmt::{self,Debug};
use super::{malloc_val, free};

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
    height: usize
}

impl<T> BinNode<T> {
    fn new(value: &T, parent: NodePtr<T>, lc: NodePtr<T>, rc: NodePtr<T>, height: usize) -> Self {
        BinNode {
            data: unsafe {ptr::read(value)},
            parent: parent,
            lc: lc,
            rc: rc,
            height: height
        }
    }
}

pub trait Node<T> {
    fn parent(&self) -> Ptr<Self>;
    fn lc(&self) -> Ptr<Self>;
    fn rc(&self) -> Ptr<Self>;
    fn height(&self) -> usize;
    fn set_parent(&mut self, value: &Ptr<Self>);
    fn set_lc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr>;
    fn set_rc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr>;
    fn set_height(&mut self, value: &usize);
    fn malloc(value: &T, parent: *mut Self) -> *mut Self;
    fn free(ptr: *mut Self);

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

    fn insert_as_lc(&mut self, value: &T) -> Result<(), InsertErr> {
        let parent: *mut Self = self;

        self.set_lc (
            &NonNull::new (
                Self::malloc(value, parent)
            )
        )?;
        
        Ok(())
    }

    fn insert_as_rc(&mut self, value: &T) -> Result<(), InsertErr> {
        let parent: *mut Self = self;

        self.set_rc (
            &NonNull::new (
                Self::malloc(value, parent)
            )
        )?;

        Ok(())
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

        Self::free(subtree);

        size
    }
}

impl<T> Node<T> for BinNode<T> {
    fn malloc(value: &T, parent: *mut Self) -> *mut Self {
        malloc_val (
            &Self::new(value, NonNull::new(parent), None, None, 0)
        )
    }

    fn free(ptr: *mut Self) {
        free(ptr, 1).unwrap();
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

    fn height(&self) -> usize {
        self.height
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

    fn set_height(&mut self, value: &usize) {
        unsafe {
            self.height = ptr::read(value);
        }
    }
}

pub trait UpdateHeight<T, N: Node<T>> {
    fn stature(Ptr<N>) -> usize;

    fn update_height(mut node: NonNull<N>) -> usize {
        unsafe {
            let (lc, rc) = (
                node.as_ref().lc(),
                node.as_ref().rc()
            );

            node.as_mut().set_height(&max(Self::stature(lc), Self::stature(rc)));

            node.as_ref().height()
        }
    }
    
    fn update_height_above(mut node: NonNull<N>) {
        Self::update_height(node);

        unsafe{
            while let Some(parent) = node.as_ref().parent() {
                Self::update_height(parent);
                node = parent;
            }
        }
    }

    
}

pub trait Tree<T, N: Node<T>>: UpdateHeight<T, N> {
    fn size(&self) -> usize;
    fn set_size(&mut self, value: &usize);
    fn root(&self) -> Ptr<N>;

    fn empty(&self) -> bool {
        self.size() == 0
    }

    fn insert_as_lc(&mut self, mut ptr: NonNull<N>, value: &T) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            let size = self.size();
            self.set_size(&(size+1));

            ptr.as_mut().insert_as_lc(value)?;

            Ok(ptr.as_mut().lc().unwrap())
        }
    }

    fn insert_as_rc(&mut self, mut ptr: NonNull<N>, value: &T) -> Result<NonNull<N>, InsertErr> {
        unsafe {
            let size = self.size();
            self.set_size(&(size+1));

            ptr.as_mut().insert_as_rc(value)?;

            Ok(ptr.as_mut().rc().unwrap())
        }
    }

    fn remove(&mut self, mut subtree: NonNull<N>) {
        unsafe {
            if let Some(parent) = subtree.as_ref().parent() {
                Self::update_height_above(parent);
            }

            subtree.as_mut().set_parent(&None);
        }

        let size = self.size();
        self.set_size(&(size - N::remove_at(subtree.as_ptr())));
    }
}

pub struct BinTree<T> {
    root: NodePtr<T>,
    size: usize
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree {
            root: None,
            size: 0
        }
    }

    pub fn insert_as_root(&mut self, value: &T) -> NonNull<BinNode<T>> {
        self.root = NonNull::new (
            BinNode::malloc(value, 0 as *mut BinNode<T>)
        );

        self.size += 1;
        self.root.unwrap()
    }
}

//impl<T> Tree<T, BinNode<T>> for BinTree<T> {
//    fn size(&self) -> usize {
//        self.size
//    }
//
//    fn set_size(&mut self, value: &usize) {
//        self.size = *value;
//    }
//
//    fn root(&self) -> NodePtr<T> {
//        self.root
//    }
//}
