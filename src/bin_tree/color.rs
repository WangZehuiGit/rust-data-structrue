mod private {
    use super::*;
    pub trait SetColor<T>: GetColor<T> {
        fn set_color(&mut self, color: Color);
    }
}

use super::Node;

pub enum Color {
    Red,
    Black
}

pub trait GetColor<T>: Node<T> {
    fn color(&self) -> Color;
}
