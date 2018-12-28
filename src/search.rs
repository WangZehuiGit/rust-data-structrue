use vector::Vector;
use list::List;
use sort::Sort;

pub trait Ordered<T: Ord> {
    fn insert(&mut self, value: &T);
}

impl<T: Ord + Copy> Ordered<T> for Vector<T> {
    fn insert(&mut self, value: &T) {
        let len = self.len();
        self.insert(len, value);
        self.insertion_sort(|a, b| a.cmp(b));
    }
}

impl<T: Ord> Ordered<T> for List<T> {
    fn insert(&mut self, value: &T) {
        let len = self.len();
        self.insert(len, value);
        self.sort(|a, b| a.cmp(b));
    }
}
