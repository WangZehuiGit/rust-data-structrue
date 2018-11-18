use super::{malloc_val, free};
use std::clone::Clone;
use std::ptr::{self, NonNull};
use std::iter::{Iterator,DoubleEndedIterator};
use std::ops::{Deref, DerefMut};
use std::marker::Copy;

type Link<T> = Option<NonNull<Node<T>>>;

pub struct Iter<T>(Link<T>);

impl<T> Clone for Iter<T> {
    fn clone(&self) -> Iter<T> {
        let Iter(link) = self;
        Iter(*link)
    }
}

impl<T> Copy for Iter<T> {}

impl<T> Deref for Iter<T> {
    type Target = T;

    fn deref(&self) -> &T {
        if let Iter(Some(node)) = self {
            return unsafe {&(node.as_ref().data)};
        } else {
            panic!("error! this is a None.");
        }
    }
}

impl<T> DerefMut for Iter<T> {
    fn deref_mut(&mut self) -> &mut T {
       if let Iter(Some(node)) = self {
            return unsafe {&mut (node.as_mut().data)};
        } else {
            panic!("error! this is a None.");
        }
    }
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Iter(Some(node)) = *self {
            let nxt = unsafe {Some(ptr::read(self.deref_mut() as *mut T))};
            *self = unsafe {Iter(node.as_ref().succ)};
            return nxt;
        } else {
            return None;
        }
    }
}

impl<T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<T> {
        if let Iter(Some(node)) = *self {
            let nxt = unsafe {Some(ptr::read(self.deref_mut() as *mut T))};
            *self = unsafe {Iter(node.as_ref().pred)};
            return nxt;
        } else {
            return None;
        }
    }
}

pub struct Node<T> {
    data: T,
    pred: Link<T>,
    succ: Link<T>
}

impl<T: Clone> Node<T> {
    fn new(value: &T, posi0: Link<T>, posi1: Link<T>) -> Self {
        Node {
            data: value.clone(),
            pred: posi0,
            succ: posi1
        }
    }

    pub fn insert_as_pred(&mut self, value: &T) {
        match self.pred {
            Some(mut node) => unsafe {
                node.as_mut().succ = NonNull::new(
                    malloc_val(&(Node::new(value, Some(node), NonNull::new(self as *mut Self))))
                );
                self.pred = node.as_mut().succ;
            },
            _ => {
                self.pred = NonNull::new(
                    malloc_val(&(Node::new(value, None, NonNull::new(self as *mut Self))))
                );
            }
        }
    }

    pub fn insert_as_succ(&mut self, value: &T) {
        match self.succ {
            Some(mut node) => unsafe {
                node.as_mut().pred = NonNull::new(
                    malloc_val(&(Node::new(value, NonNull::new(self as *mut Self), Some(node))))
                );
                self.succ = node.as_mut().pred;
            },
            _ => {
                self.pred = NonNull::new(
                    malloc_val(&(Node::new(value, NonNull::new(self as *mut Self), None)))
                );
            }
        }
    }
}

pub struct List<T> {
    head: Link<T>,
    trail: Link<T>,
    len: usize
}

impl<T: Clone> List<T> {
    pub fn new() -> Self {
        List::<T> {
            head: None,
            trail: None,
            len: 0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn empty(&self) -> bool {
        if self.len == 0 {
            return true;
        }
        false
    }

    pub fn first(&mut self) -> Iter<T> {
        Iter(self.head)
    }

    pub fn last(&mut self) -> Iter<T> {
        Iter(self.trail)
    }

    pub fn get(&mut self, index: usize) -> Iter<T> {
        if index >= self.len {
            return Iter(None);
        }
        
        let mut it: Iter<T>;

        if index <= self.len/2 {
            it = self.first();
            for _ in 0..index {
                it.next();
            }
        } else {
            it = self.last();
            for _ in 0..(self.len-index) {
                it.next_back();
            }
        }

        it
    }

    pub fn insert(&mut self, index: usize, value: &T) {
        if let Iter(Some(mut node)) = self.get(index) {
            unsafe {
                node.as_mut().insert_as_pred(value);
                if index == 0 {
                    self.head = node.as_ref().pred;
                }
            }
        } else if self.len == 0 {
            let node = NonNull::new(malloc_val(&Node::new(value, None, None)));
            self.head = node;
            self.trail = node;
        } else if index == self.len {
            if let Some(mut end) = self.trail {
                unsafe {
                    end.as_mut().insert_as_succ(value);
                    self.trail = end.as_ref().succ;
                }   
            }
        } else {
            panic!("bound error!");
        }

        self.len += 1;
    }

    pub fn remove(&mut self, lo: usize, hi: usize) {
        if lo >= hi || hi > self.len {
            panic!("bound error!");
        }
        let mut it = self.get(lo);
        let mut begin = self.get(lo);
        begin.next_back();
        unsafe {
            for _ in 0..(hi - lo) {
                let mut tmp = it;
                it.next();
                if let Iter(Some(mut node)) = tmp {
                    free(node.as_mut(), 1).unwrap();
                }
            }
        }
        unsafe {
            if let Iter(Some(mut end)) = it {
                if let Iter(Some(mut node)) = begin {
                    end.as_mut().pred = Some(node);
                    node.as_mut().succ = Some(end);
                } else {
                    end.as_mut().pred = None;
                    self.head = Some(end);
                }
            } else {
                if let Iter(Some(mut node)) = begin {
                    node.as_mut().pred = None;
                    self.trail = Some(node);
                } else {
                    self.head = None;
                    self.trail = None;
                }
            }
        }

        self.len -= hi - lo;
    }
}
