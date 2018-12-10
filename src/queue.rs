pub trait Queue<T> {
    fn size(&self) -> usize;
    fn empty(&self) -> bool;
    fn enqueue(&mut self, value: &T);
    fn dequeue(&mut self) -> T;
    fn front(&mut self) -> &mut T;
}
