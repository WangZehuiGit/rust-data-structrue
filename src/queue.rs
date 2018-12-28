pub trait Queue<T> {
    fn enqueue(&mut self, value: &T);
    fn dequeue(&mut self) -> T;
    fn front(&mut self) -> &mut T;
}

pub trait PriorityQueue<T: PartialOrd> {
    fn insert(&mut self, value: &T);
    fn del_max(&mut self) -> T;
    fn max(&self) -> &T;
}
