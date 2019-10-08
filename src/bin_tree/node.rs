use super::super::{free, malloc_val};
use super::color::{Color, GetColor};
use super::height::GetHeight;
use super::{InsertErr, Ptr};
use std::ptr::{self, NonNull};

pub trait Node<T>: Sized {
    fn new(value: &T, parent: Ptr<Self>) -> Self;
    fn set_parent(&mut self, value: &Ptr<Self>);
    fn set_lc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr>;
    fn set_rc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr>;
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

    fn insert_as_lc(&mut self, value: &T) -> Result<(), InsertErr> {
        let parent: *mut Self = self;

        self.set_lc(&NonNull::new(malloc_val(&Self::new(
            value,
            NonNull::new(parent),
        ))))?;

        Ok(())
    }

    fn insert_as_rc(&mut self, value: &T) -> Result<(), InsertErr> {
        let parent: *mut Self = self;

        self.set_rc(&NonNull::new(malloc_val(&Self::new(
            value,
            NonNull::new(parent),
        ))))?;

        Ok(())
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

pub trait ColorNode<T>: GetColor<T> + Node<T> {
    fn set_color(&mut self, color: Color);
}

pub trait HeightNode<T>: GetHeight<T> + Node<T> {
    fn set_height(&mut self, value: &usize);
}
