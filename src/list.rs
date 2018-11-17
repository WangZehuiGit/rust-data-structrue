use super::{malloc_val, free};
use std::clone::Clone;
use std::ptr::{self, NonNull};

type Link<T> = Option<NonNull<Node<T>>>;

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
    head: NonNull<Node<T>>,
    trail: NonNull<Node<T>>,
    len: usize
}

impl<T: Clone> List<T> {
    pub fn new(value: &T) -> Self {
        let link = malloc_val(&(Node::new(value, None, None)));
        List::<T> {
            head: NonNull::new(link).unwrap(),
            trail: NonNull::new(link).unwrap(),
            len: 1
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn first(&mut self) -> &mut Node<T> {
        unsafe {self.head.as_mut()}
    }

    pub fn last(&mut self) -> &mut Node<T> {
        unsafe {self.trail.as_mut()}
    }

    pub fn push_back(&mut self, value: &T) {
        self.last().insert_as_succ(value);
        if let Some(other) = self.last().succ {
            self.trail = other;
            self.len += 1;
        }
    }

    pub fn push_front(&mut self, value: &T) {
        self.first().insert_as_pred(value);
        if let Some(other) = self.first().pred {
            self.head = other;
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) {
        if let Some(other) = self.last().pred {
            free(self.trail.as_ptr(), 1).unwrap();
            self.trail = other;
            self.len -= 1;
        }
    }

    pub fn pop_front(&mut self) {
        if let Some(other) = self.first().succ {
            free(self.head.as_ptr(), 1).unwrap();
            self.head = other;
            self.len -= 1;
        }
    }
}
