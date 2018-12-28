pub trait Stack<T> {
    fn push(&mut self, value: &T);
    fn pop(&mut self) -> T;
    fn top(&mut self) -> &mut T;
}
