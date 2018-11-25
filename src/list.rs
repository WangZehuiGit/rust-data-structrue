use super::{malloc_val, free};
use std::clone::Clone;
use std::ptr::{self, NonNull};
use std::iter::Iterator;
use std::marker::{Copy, PhantomData};
use std::ops::Drop;
use std::cmp::PartialEq;

type Link<T> = Option<NonNull<Node<T>>>;

pub struct Iter<'a, T: 'a>(Link<T>, PhantomData<&'a T>);

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Iter<'a, T> {
        let Iter(link, PhantomData) = self;
        Iter(*link, PhantomData)
    }
}

impl<'a, T> Copy for Iter<'a, T> {}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        let succ: Link<T>;
        let nxt: Option<&'a mut T>;
        if let Iter(Some(node), PhantomData) = self {
            nxt = unsafe {Some(&mut (*node.as_ptr()).data)};
            unsafe {
                succ = node.as_ref().succ();
            }
        } else {
            return None;
        }

        *self = Iter(succ, PhantomData);
        return nxt;
    }
}

pub struct Node<T> {
    pub data: T,
    pred: Link<T>,
    succ: Link<T>
}

impl<T> Node<T> {
    fn new(value: &T, posi0: Link<T>, posi1: Link<T>) -> Self {
        Node {
            data: unsafe {ptr::read(value as *const T)},
            pred: posi0,
            succ: posi1
        }
    }

    fn insert_as_pred(&mut self, value: &T) {
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

    fn insert_as_succ(&mut self, value: &T) {
        match self.succ {
            Some(mut node) => unsafe {
                node.as_mut().pred = NonNull::new(
                    malloc_val(&(Node::new(value, NonNull::new(self as *mut Self), Some(node))))
                );
                self.succ = node.as_mut().pred;
            },
            _ => {
                self.succ = NonNull::new(
                    malloc_val(&(Node::new(value, NonNull::new(self as *mut Self), None)))
                );
            }
        }
    }

    pub fn pred(&self) -> Link<T> {
        self.pred
    }

    pub fn succ(&self) -> Link<T> {
        self.succ
    }
}

pub struct List<T> {
    head: Link<T>,
    trail: Link<T>,
    len: usize
}

impl<T> List<T> {
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

    pub fn first_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            if let Some(node) = self.head {
                Some(&mut *(node.as_ptr()))
            } else {
                None
            }
        }
    }

    pub fn first(&mut self) -> Option<&Node<T>> {
        if let Some(node) = self.first_mut() {
            Some(node)
        } else {
            None
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut Node<T>> {
        unsafe {
            if let Some(node) = self.trail {
                Some(&mut *(node.as_ptr()))
            } else {
                None
            }
        }
    }

    pub fn last(&mut self) -> Option<&Node<T>> {
        if let Some(node) = self.last_mut() {
            Some(node)
        } else {
            None
        }
    }

    pub fn iter(&mut self) -> Iter<T> {
        Iter(self.head, PhantomData)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        if index >= self.len {
            return None;
        }
        
        let mut ptr: Option<&mut Node<T>>;

        if index <= self.len/2 {
            ptr = self.first_mut();
            for _ in 0..index {
                unsafe {
                    ptr = Some(&mut *(ptr.unwrap().succ().unwrap().as_ptr()));
                }
            }
        } else {
            let len = self.len;
            ptr = self.last_mut();
            for _ in 0..(len-index) {
                unsafe {
                    ptr = Some(&mut *(ptr.unwrap().succ().unwrap().as_ptr()));
                }
            }
        }

        ptr
    }

    pub fn get(&mut self, index: usize) -> Option<&Node<T>> {
        if let Some(node) = self.get_mut(index) {
            Some(node)
        } else {
            None
        }
    }

    pub fn insert(&mut self, index: usize, value: &T) {
        let mut head = self.head;
        let mut trail = self.trail;
        let len = self.len;
        let last = self.trail;
        if let Some(node) = self.get_mut(index) {
            node.insert_as_pred(value);
            if index == 0 {
                head = node.pred;
            }
        } else if index == 0 {
            let node = NonNull::new(malloc_val(&Node::new(value, None, None)));
            head = node;
            trail = node;
        } else if index == len {
            if let Some(mut end) = last {
                unsafe {
                    end.as_mut().insert_as_succ(value);
                    trail = end.as_ref().succ;
                }   
            }
        } else {
            panic!("bound error!");
        }

        self.head = head;
        self.trail = trail;
        self.len += 1;
    }

    pub fn remove(&mut self, lo: usize, hi: usize) {
        if lo >= hi || hi > self.len {
            panic!("bound error!");
        }

        let mut head = self.head;
        let mut trail = self.trail;

        if let Some(mut it) = self.get_mut(lo) {
            let begin = it.pred;
            let mut end = it.succ;

            unsafe {
                for _ in 0..(hi - lo) {
                    let mut tmp = it as *mut Node<T>;
                    end = it.succ;
                    free(tmp, 1).unwrap();
                    if let Some(node) = end {
                        it = &mut *(node.as_ptr())
                    }
                }
            }
            unsafe {
                if let Some(mut end) = end {
                    if let Some(mut node) = begin {
                        end.as_mut().pred = Some(node);
                        node.as_mut().succ = Some(end);
                    } else {
                        end.as_mut().pred = None;
                        head = Some(end);
                    }
                } else {
                    if let Some(mut node) = begin {
                        node.as_mut().pred = None;
                        trail = Some(node);
                    } else {
                        head = None;
                        trail = None;
                    }
                }
            }
        }

        self.head = head;
        self.trail = trail;
        self.len -= hi - lo;
    }
}

impl<T: PartialEq> List<T> {
    pub fn find_mut(&mut self, value: &T, lo: usize, hi: usize) -> Option<&mut Node<T>> {
        let mut it = self.get_mut(lo);
        let mut cnt = 0usize;

        unsafe {
            while let Some(node) = it {
                if cnt == hi {
                    break;
                }

                cnt += 1;

                if *value == node.data {
                    return Some(node);
                }

                if let Some(new) = node.succ {
                    it = Some(&mut *(new.as_ptr()));
                } else {
                    break;
                }
            }
        }

        None
    }

    pub fn find(&mut self, value: &T, lo: usize, hi: usize) -> Option<&Node<T>> {
        if let Some(node) = self.find_mut(value, lo, hi) {
            Some(node)
        } else {
            None
        }
    }

    pub fn deduplicate(&mut self) {
        if self.len < 2 {
            return;
        }

        let mut it = self.head;

        unsafe {
            while let Some(node) = it {
                let mut next = node.as_ref().succ();
                while let Some(mut other) = next {
                    next = other.as_ref().succ();
                    if node.as_ref().data == other.as_ref().data {
                        let pred = other.as_ref().pred;
                        pred.unwrap().as_mut().succ = next;
                        if let Some(mut next) = next {
                            next.as_mut().pred = pred;
                        } else {
                            self.trail = pred;
                        }
                        free(other.as_mut(), 1).unwrap();
                        self.len -= 1;
                    }
                }
                it = node.as_ref().succ;
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let len = self.len;

        if len != 0 {
            self.remove(0, len);
        }
    }
}

#[macro_export]
macro_rules! list {
    ($($e: expr), *) => (
        {
            let mut tmp = List::new();
            $(
                let len = tmp.len();
                tmp.insert(len, &$e);
            )*
            tmp
        }
    )
}
