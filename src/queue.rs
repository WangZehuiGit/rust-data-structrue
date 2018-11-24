use super::list::*;
use std::ptr;

pub trait Queue<T> {
    fn size(&self) -> usize;
    fn empty(&self) -> bool;
    fn enqueue(&mut self, value: &T);
    fn dequeue(&mut self) -> T;
    fn front(&mut self) -> &mut T;
}

pub trait PriorityQueue<V: std::cmp::PartialOrd> : Queue<V> {
    fn insert(&mut self, value: &V);
    fn get_max(&self) -> &V;
    fn del_max(&mut self) -> V;
}

pub struct QueueList<T> {
    list: List<T>
}

impl<T> Queue<T> for QueueList<T> {
    fn size(&self) -> usize {
        self.list.len()
    }

    fn empty(&self) -> bool {
        self.list.empty()
    }

    fn enqueue(&mut self, value: &T) {
        let size = self.size();
        self.list.insert(size, value);
    }

    fn dequeue(&mut self) -> T {
        let size = self.size();
        if size == 0 {
            panic!("this queue is empty");
        }
        unsafe {
            let out = ptr::read(&self.list.first().unwrap().data);
            self.list.remove(size - 1, size);

            out
        }
    }

    fn front(&mut self) -> &mut T {
        if self.size() == 0 {
            panic!("this queue is empty");
        }
        &mut self.list.first().unwrap().data
    }
}

impl<T> QueueList<T> {
    pub fn new() -> Self {
        QueueList {
            list: List::new()
        }
    }
}
