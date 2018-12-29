pub use super::sort::Sort;
use super::{malloc, free};
use super::stack::Stack;
use super::search::Ordered;
use std::ptr;
use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::cmp::PartialEq;
use std::marker::PhantomData;
use std::iter::DoubleEndedIterator;
use std::fmt;

type    Rank = usize;
const    DEFAULT_CAPACITY: usize = 8;

#[derive(Clone, Copy)]
pub struct Iter<'a, T: 'a> {
    ptr: *mut T,
    end: *mut T,
    marker: PhantomData<&'a mut T>
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        unsafe {
            let ptr = self.ptr;
            self.ptr = self.ptr.add(1);
            Some(&mut (*ptr))
        }
    }
}

impl<'a, T: 'a> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == self.ptr {
            return None;
        }

        unsafe {
            self.end = self.end.sub(1);
            Some(&mut (*self.end))
        }
    }
}

pub struct Vector<T> {
    len: Rank,
    capacity: usize,
    ptr: *mut T
}    

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            ptr: malloc(DEFAULT_CAPACITY).unwrap(),
            len: 0 as Rank,
            capacity: DEFAULT_CAPACITY,
        }
    }

    pub fn from_slice(slice: &[T]) -> Vector<T> {
        let new_ptr = malloc(slice.len()).unwrap();
        unsafe {ptr::copy(slice.as_ptr(), new_ptr, slice.len());}
        Vector {
            ptr: new_ptr,
            len: slice.len(),
            capacity: slice.len()
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> Rank {
        self.len
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }

    pub fn map<F>(&mut self, mut func: F, lo: usize, hi: usize)
    where
        F: FnMut(&mut T)
    {
        for i in lo..hi {
            func(self.index_mut(i));
        }
    }

    pub fn insert(&mut self, rank: Rank, value: &T) {
        self.expand();
        self.len += 1;
        for i in (rank..(self.len-1)).rev() {
            unsafe {self[i+1] = ptr::read(&self[i]);}
        }
        unsafe {self[rank] = ptr::read(value);}
    }

    pub fn remove(&mut self, mut lo: Rank, mut hi: Rank) {
        let size = hi - lo;
        while hi < self.len {
            unsafe {self[lo] = ptr::read(&self[hi]);}
            lo += 1;
            hi += 1;
        }
        self.shrink();
        self.len -= size;
    }

    fn expand(&mut self) {
        if (self.len as usize) < self.capacity {
            return;
        }
        unsafe {
            let new_ptr = malloc(self.capacity * 2).unwrap();
            ptr::copy(self.ptr, new_ptr, self.len);
            free(self.ptr, self.capacity).expect("vector.rs 123 error when free\n");
            self.ptr = new_ptr;
        }
        self.capacity *= 2;
    }

    fn shrink(&mut self) {
        if self.capacity < 2*DEFAULT_CAPACITY || self.len*4 > self.capacity {
            return;
        }
        unsafe {
            let new_ptr = malloc(self.capacity / 2).unwrap();
            ptr::copy(self.ptr, new_ptr, self.len);
            free(self.ptr, self.capacity).unwrap();
            self.ptr = new_ptr;
        }
        self.capacity /= 2;
    }

    pub fn swap(&mut self, i: Rank, j: Rank) {
        unsafe {
            ptr::swap(&mut self[i], &mut self[j]);
        }
    }
}

impl<T: PartialEq> Vector<T> {
    pub fn find(&self, e: &T) -> Option<Rank> {
        for i in 0..self.len {
            if self[i] == *e {
                return Option::Some(i);
            }
        }
        Option::None
    }
}

impl<'a, T: 'a> Vector<T> {
    pub fn iter(&mut self) -> Iter<'a, T> {
        Iter {
            ptr: self.ptr,
            end: unsafe {self.ptr.add(self.len)},
            marker: PhantomData
        }
    }
}

impl<T> Clone for Vector<T> {
    fn clone(&self) -> Self {
        let new_ptr = malloc(self.capacity).unwrap();

        unsafe {ptr::copy(self.ptr, new_ptr, self.len);}
        Vector {
            ptr: new_ptr,
            len: self.len,
            capacity: self.capacity
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        free(self.ptr, self.capacity).unwrap();
    }
}

impl<T> Index<Rank> for Vector<T> {
    type Output = T;

    fn index(&self, i: Rank) -> &T {
        if i >= self.len {
            panic!("array bound!");
        }
        unsafe {&(*(self.ptr.add(i)))}
    }
}

impl<T> IndexMut<Rank> for Vector<T> {
    fn index_mut(&mut self, i: Rank) -> &mut T {
        if i >= self.len {
            panic!("array bound!");
        }
        unsafe {&mut (*(self.ptr.add(i)))}
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {std::slice::from_raw_parts(self.ptr, self.len)}
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {std::slice::from_raw_parts_mut(self.ptr, self.len)}
    }
}

impl<T> PartialEq for Vector<T>
where
    T: PartialEq 
{
    fn eq(&self, other: &Vector<T>) -> bool {
        if self.len != other.len {
            return false;
        }
        for i in 0..self.len {
            if self[i] != other[i] {
                return false;
            }
        }
        true
    }
}

impl<T> fmt::Debug for Vector<T>
where
    T: fmt::Debug 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut it = 0..self.len;

        s.push_str(&format!("[{:?}", self[it.next().unwrap()]));
        
        for i in it {
            s.push_str(&format!(", {:?}", self[i]));
        }
        s.push(']');

        write!(f, "{}", s)
    }
}

impl<T> Stack<T> for Vector<T> {
    fn push(&mut self, value: &T) {
        let size = self.len();
        self.insert(size, value);
    }

    fn pop(&mut self) -> T {
        let size = self.len();

        if size == 0 {
            panic!("this stack is empty");
        }

        unsafe {
            let out = ptr::read(&self[size - 1]);

            self.remove(size - 1, size);

            out
        }
    }

    fn top(&mut self) -> &mut T {
        let size = self.len();

        if size == 0 {
            panic!("this stack is empty");
        }

        self.index_mut(size - 1)
    }
}

impl<'a, T: 'a + Copy> Sort<Iter<'a, T>> for Vector<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn iter(&mut self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T: Ord + Copy> Ordered<T> for Vector<T> {
    fn push(&mut self, value: &T) {
        let len = self.len();
        self.insert(len, value);
        self.insertion_sort(|a, b| a.cmp(b));
    }
}
