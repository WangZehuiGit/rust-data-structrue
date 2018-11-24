use super::vector::Vector;
use std::ops::IndexMut;
use std::ptr;

pub trait Stack<T> {
    fn size(&self) -> usize;
    fn empty(&self) -> bool;
    fn push(&mut self, value: &T);
    fn pop(&mut self) -> T;
    fn top(&mut self) -> &mut T;
}

pub struct StackVector<T> {
    vec: Vector<T>
}

impl<T> Stack<T> for StackVector<T> {
    fn size(&self) -> usize {
        self.vec.len()
    }

    fn empty(&self) -> bool {
        self.vec.empty()
    }

    fn push(&mut self, value: &T) {
        let size = self.size();
        self.vec.insert(size, value);
    }

    fn pop(&mut self) -> T {
        let size = self.size();

        if size == 0 {
            panic!("this stack is empty");
        }

        unsafe {
            let out = ptr::read(&self.vec[size - 1]);

            self.vec.remove(size - 1, size);

            out
        }
    }

    fn top(&mut self) -> &mut T {
        let size = self.size();

        if size == 0 {
            panic!("this stack is empty");
        }

        self.vec.index_mut(size - 1)
    }
}

impl<T> StackVector<T> {
    pub fn new() -> Self {
        StackVector {
            vec: Vector::new()
        }
    }
}
