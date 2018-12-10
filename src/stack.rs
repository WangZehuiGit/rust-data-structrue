pub trait Stack<T> {
    fn size(&self) -> usize;
    fn empty(&self) -> bool;
    fn push(&mut self, value: &T);
    fn pop(&mut self) -> T;
    fn top(&mut self) -> &mut T;
}
