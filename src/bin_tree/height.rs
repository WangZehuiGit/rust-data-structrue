use super::Node;
use super::Ptr;
use std::ptr::NonNull;
use std::cmp::max;
use super::private::NodeHeight;

pub trait GetHeight<T>: Node<T> {
    fn height(&self) -> usize;
}

pub trait UpdateHeight<T>: NodeHeight<T> {
    fn stature(ptr: Ptr<Self>) -> usize {
        if let Some(node) = ptr {
            return unsafe {node.as_ref().height()};
        }

        0
    }

    fn update_height(mut node: NonNull<Self>) -> usize {
        unsafe {

            let (lc, rc) = (
                node.as_ref().lc(),
                node.as_ref().rc()
            );

            node.as_mut().set_height(&max(Self::stature(lc), Self::stature(rc)));

            node.as_ref().height()
        }
    }

    fn update_height_above(mut node: NonNull<Self>) {
        Self::update_height(node);

        unsafe{
            while let Some(parent) = node.as_ref().parent() {
                Self::update_height(parent);
                node = parent;
            } 
        } 
    } 
}
