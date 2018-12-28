pub use super::queue::PriorityQueue;
use super::vector::Vector;
use std::ptr;

pub struct Heap<T: Ord> {
    vec: Vector<T>,
}

impl<T> Heap<T>
where
    T: Ord + Default,
{
    pub fn new() -> Self {
        let mut h = Heap {
            vec: Vector::new(),
        };

        h.vec.insert(0, &Default::default());
        
        h
    }
}

impl<T: Ord> Heap<T> {
    pub fn size(&self) -> usize {
        self.vec.len() - 1
    }

    fn swim(&mut self, mut index: usize) {
        while index != 1 {
            if self.vec[index] > self.vec[index / 2] {
                self.vec.swap(index, index / 2);
                index /= 2;
            } else {
                break;
            }
        }
    }

    fn sink(&mut self, mut index: usize) {
        while index <= (self.vec.len() - 1) / 2 {
            let mut max_child = index * 2;
            if max_child < self.vec.len() - 1 &&
                self.vec[max_child] < self.vec[max_child + 1] {
                max_child += 1;
            }

            if self.vec[index] < self.vec[max_child] {
                self.vec.swap(index, max_child);
                index = max_child;
            } else {
                break;
            }
        }
    }
}

impl<T: Ord> PriorityQueue<T> for Heap<T> {
    fn insert(&mut self, value: &T) {
        let len = self.vec.len();
        self.vec.insert(len, value);
        self.swim(len);
    }

    fn del_max(&mut self) -> T {
        let len = self.vec.len();
        let v = unsafe {ptr::read(&self.vec[1])};
        self.vec.swap(1, len - 1);
        self.vec.remove(len - 1, len);
        self.sink(1);
        
        v
    }

    fn max(&self) -> &T {
        &self.vec[1]
    }
}
