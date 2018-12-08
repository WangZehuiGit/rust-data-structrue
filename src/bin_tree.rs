use std::ptr::{self, NonNull};
use super::{malloc_val, free};

enum Color {
    RED,
    BLACK
}

type Ptr<T> = Option<NonNull<T>>;
type NodePtr<T> = Ptr<BinNode<T>>;

pub struct BinNode<T> {
    pub data: T,
    parent: Option<NonNull<BinNode<T>>>,
    lc: Option<NonNull<BinNode<T>>>,
    rc: Option<NonNull<BinNode<T>>>,
    height: usize
}

impl<T> BinNode<T> {
    fn new (value: &T, parent: NodePtr<T>, lc: NodePtr<T>, rc: NodePtr<T>, height: usize) -> Self {
        BinNode {
            data: unsafe {ptr::read(value)},
            parent: parent,
            lc: lc,
            rc: rc,
            height: height
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent == None
    }

    pub fn is_lc(&self) -> bool {
        let self_ptr = self as *const BinNode<T>;

        unsafe {
            if let Some(node) = self.parent {
                return self_ptr == node.as_ref().lc.unwrap().as_ptr();
            }

            false
        }
    }

    pub fn is_rc(&self) -> bool {
        !self.is_lc() && !self.is_root()
    }

    pub fn has_lc(&self) -> bool {
        self.lc != None
    }

    pub fn has_rc(&self) -> bool {
        self.rc != None
    }

    pub fn is_leaf(&self) -> bool {
        !self.has_lc() && !self.has_rc()
    }

    pub fn insert_as_lc(&mut self, value: &T) -> Result<(), &str>{
        if let None = self.lc {
            self.lc = NonNull::new (
                malloc_val (
                    &BinNode::new(value, NonNull::new(self), None, None, 0)
                )
            );
            return Ok(());
        } else {
            Err("left child is not none")
        }
    }

    pub fn insert_as_rc(&mut self, value: &T) -> Result<(), &str>{
        if let None = self.rc {
            self.rc = NonNull::new (
                malloc_val (
                    &BinNode::new (value, NonNull::new(self), None, None, 0)
                )
            );
            return Ok(());
        } else {
            Err("right child is not none")
        }
    }

    pub fn succ(&self) -> NodePtr<T> {
        let mut succ: NodePtr<T>;

        unsafe {
            if let Some(mut node) = self.rc {
                succ = self.rc;
                while let Some(next) = node.as_ref().lc {
                    succ = Some(next);
                    node = next;
                }
            } else {
                succ = None;
                if let Some(mut node) = self.parent {
                    if self.is_lc() {
                        succ = Some(node);
                    } else {
                        while let Some(next) = node.as_ref().parent {
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
}

pub trait UpdateHeight {
    type Node;

    fn update_height(node: NonNull<Self::Node>) -> usize;
    fn parent(node: NonNull<Self::Node>) -> Ptr<Self::Node>;

    fn update_height_above(mut node: NonNull<Self::Node>) {
        Self::update_height(node);

        while let Some(parent) = Self::parent(node) {
            Self::update_height(parent);
            node = parent;
        }
    }
}

pub struct BinTree<T> {
    root: NodePtr<T>,
    parent: Option<NonNull<BinTree<T>>>,
    height: usize,
    size: usize
}
