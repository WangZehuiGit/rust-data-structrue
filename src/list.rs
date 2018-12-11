use super::{malloc_val, free};
use super::queue::Queue;
use std::ptr::{self, NonNull};
use std::ops::{Drop, Index, IndexMut, Fn};
use std::cmp::PartialEq;
use std::default::Default;

type Link<T> = Option<NonNull<Node<T>>>;

pub struct Node<T> {
    pub data: T,
    pred: Link<T>,
    succ: Link<T>
}

impl<T> Node<T> {
    fn new(value: &T, posi0: Link<T>, posi1: Link<T>) -> Self {
        Node {
            data: unsafe {ptr::read(value)},
            pred: posi0,
            succ: posi1
        }
    }

    fn insert_as_pred(&mut self, value: &T) {
        match self.pred {
            Some(mut node) => unsafe {
                node.as_mut().succ = NonNull::new(
                    malloc_val(&(Node::new(value, Some(node), NonNull::new(self))))
                );
                self.pred = node.as_mut().succ;
            },
            _ => {
                self.pred = NonNull::new(
                    malloc_val(&(Node::new(value, None, NonNull::new(self))))
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
    head: *mut Node<T>,
    trail: *mut Node<T>,
    len: usize
}

impl<T: Default> List<T> {
    pub fn new() -> Self {
        let list = List::<T> {
            head: malloc_val(&Node::new(&Default::default(), None, None)),
            trail: malloc_val(&Node::new(&Default::default(), None, None)),
            len: 0
        };
        unsafe {
            (*list.head).succ = NonNull::new(list.trail);
            (*list.trail).pred = NonNull::new(list.head);
        }

        list
    }
} 

impl<T> List<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn empty(&self) -> bool {
        if self.len == 0 {
            return true;
        }
        false
    }

    fn get(&self, index: usize) -> Link<T> {
        if index > self.len {
            return None;
        }
        
        let mut ptr: Link<T>;

        if index < self.len/2 {
            ptr = unsafe {(*self.head).succ};
            for _ in 0..index {
                unsafe {
                    ptr = ptr.unwrap().as_ref().succ();
                }
            }
        } else {
            ptr = NonNull::new(self.trail);
            for _ in 0..(self.len-index) {
                unsafe {
                    ptr = ptr.unwrap().as_ref().pred();
                }
            }
        }

        ptr
    }

    pub fn map<F, R>(&mut self, func: F, lo: usize, hi: usize) -> Box<Vec<R>>
    where
        F: Fn(&T) -> R
    {
        let mut it = self.get(lo);
        let mut cnt = lo;
        let mut r = Box::new(Vec::<R>::new());

        unsafe {
            while let Some(mut node) = it {
                if cnt == hi {
                    break;
                }

                r.push(func(&node.as_mut().data));

                it = node.as_ref().succ;
                cnt += 1;
           }

           r
        }
    }

    pub fn insert(&mut self, index: usize, value: &T) {
        unsafe {
            if let Some(mut node) = self.get(index) {
                node.as_mut().insert_as_pred(value);
            } else {
                panic!("bound error!");
            }
        }
        self.len += 1;
    }

    pub fn remove(&mut self, lo: usize, hi: usize) {
        if lo >= hi || hi > self.len {
            panic!("bound error!");
        }

        if let Some(mut it) = self.get(lo) {
            unsafe {
                let begin = it.as_ref().pred;
                let end = self.get(hi);
                for _ in 0..(hi - lo) {
                    let next = it.as_ref().succ;
                    free(it.as_mut(), 1).unwrap();
                    if let Some(node) = next {
                        it = node;
                    }
                    end.unwrap().as_mut().pred = begin;
                    begin.unwrap().as_mut().succ = end;
                }
            }
        }

        self.len -= hi - lo;
    }
}

impl<T: PartialEq> List<T> {
    pub fn find(&mut self, value: &T, lo: usize, hi: usize) -> Option<usize> {
        let mut it = self.get(lo);
        let mut cnt = lo;

        unsafe {
            while let Some(node) = it {
                if cnt == hi {
                    break;
                }

                if *value == node.as_ref().data {
                    return Some(cnt);
                }

                it = node.as_ref().succ;
                cnt += 1;
           }
        }

        None
    }

    pub fn deduplicate(&mut self) {
        if self.len < 2 {
            return;
        }

        unsafe {
            let mut it = (*self.head).succ;
            let end = NonNull::new(self.trail);

            while let Some(node) = it {
                let mut next = node.as_ref().succ;
                while let Some(mut other) = next {
                    next = other.as_ref().succ;
                    if node.as_ref().data == other.as_ref().data {
                        let pred = other.as_ref().pred;

                        pred.unwrap().as_mut().succ = next;
                        next.unwrap().as_mut().pred = pred;
                        free(other.as_mut(), 1).unwrap();
                        self.len -= 1;
                    }
                    if next == end {
                        break;
                    }
                }
                it = node.as_ref().succ;
                if it == end {
                    break;
                }
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

        free(self.head, 1).unwrap();
        free(self.trail, 1).unwrap();
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        if i >= self.len() {
            panic!("bound error!");
        }

        let node = self.get(i).unwrap();

        unsafe {&(*node.as_ptr()).data}
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        if i >= self.len() {
            panic!("bound error!");
        }

        let node = self.get(i).unwrap();

        unsafe {&mut (*node.as_ptr()).data}
    }
}

impl<T> Queue<T> for List<T> {
    fn size(&self) -> usize {
        self.len()
    }

    fn empty(&self) -> bool {
        self.empty()
    }

    fn enqueue(&mut self, value: &T) {
        let size = self.size();
        self.insert(size, value);
    }

    fn dequeue(&mut self) -> T {
        let size = self.size();
        if size == 0 {
            panic!("this queue is empty");
        }
        unsafe {
            let out = ptr::read(&(self[0]));
            self.remove(size - 1, size);

            out
        }
    }

    fn front(&mut self) -> &mut T {
        if self.size() == 0 {
            panic!("this queue is empty");
        }
        &mut self[0]
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
