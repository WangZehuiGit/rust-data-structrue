use super::Node;

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Black
}

pub trait GetColor<T>: Node<T> {
    fn color(&self) -> Color;
}
