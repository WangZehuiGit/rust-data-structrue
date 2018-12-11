use super::{Ptr, InsertErr};
use super::super::{malloc_val, free};
use super::color::{Color, GetColor};
use super::height::GetHeight;
use std::ptr::NonNull;

pub trait Node<T>: super::Node<T> {
    fn new(value: &T, parent: Ptr<Self>) -> Self;
    fn set_parent(&mut self, value: &Ptr<Self>);
    fn set_lc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr>;
    fn set_rc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr>;

    fn insert_as_lc(&mut self, value: &T) -> Result<(), InsertErr> {
        let parent: *mut Self = self;

        self.set_lc (
            &NonNull::new (
                malloc_val(
                    &Self::new(value, NonNull::new(parent))
                )
            )
        )?;

        Ok(())
    }

    fn insert_as_rc(&mut self, value: &T) -> Result<(), InsertErr> {
        let parent: *mut Self = self;

        self.set_rc (
            &NonNull::new (
                malloc_val(
                    &Self::new(value, NonNull::new(parent))
                )
            )
        )?;

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

pub trait NodeColor<T>: GetColor<T> {
    fn set_color(&mut self, color: Color);
}

pub trait NodeHeight<T>: GetHeight<T> {
    fn set_height(&mut self, value: &usize);
}
