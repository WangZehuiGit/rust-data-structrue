use super::Node;
use super::Ptr;
use super::InsertErr;
use std::ptr::NonNull;
use std::cmp::max;
use std::marker::PhantomData;
use super::private::HeightNode;
use super::private;

pub trait GetHeight<T>: Node<T> {
    fn height(&self) -> usize;
}

pub trait UpdateHeight<T>: HeightNode<T> {
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

            node.as_mut().set_height(&(1 + max(Self::stature(lc), Self::stature(rc))));

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

pub struct HeightBinNode<T, N: private::Node<T>> {
    node: N,
    height: usize,
    marker: PhantomData<T>
}

impl<T, N: private::Node<T>> HeightBinNode<T, N> {
    fn into(node: Ptr<Self>) -> Ptr<N> {
        if let Some(mut node) = node {
            return unsafe {
                NonNull::new(&mut node.as_mut().node)
            }
        } else {
            None
        }
    }

    fn from(node: Ptr<N>) -> Ptr<Self> {
        if let Some(node) = node {
            return NonNull::new(node.as_ptr() as *mut HeightBinNode<T, N>);
        } else {
            None
        }
    }
}

impl<T, N: private::Node<T>> Node<T> for HeightBinNode<T, N> {
    fn get(&mut self) -> &mut T {
        self.node.get()
    }

    fn parent(&self) -> Ptr<Self> {
        Self::from(self.node.parent())
    }

    fn lc(&self) -> Ptr<Self> {
        Self::from(self.node.lc())
    }

    fn rc(&self) -> Ptr<Self> {
        Self::from(self.node.rc())
    }
}

impl<T, N: private::Node<T>> private::Node<T> for HeightBinNode<T, N>
where
    Self: UpdateHeight<T>
{
    fn new(value: &T, parent: Ptr<Self>) -> Self {
        Self {
            node: N::new(value, Self::into(parent)),
            height: 1,
            marker: PhantomData
        }
    }

    fn set_parent(&mut self, value: &Ptr<Self>) {
        let parent = self.node.parent();

        self.node.set_parent(&Self::into(*value));
        
        if parent != None {
            Self::update_height_above(Self::from(parent).unwrap());
        }
    }
    
    fn set_lc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr> {
        let result = self.node.set_lc(&Self::into(*value));
        Self::update_height_above(NonNull::new(self).unwrap());

        result
    }

    fn set_rc(&mut self, value: &Ptr<Self>) -> Result<(), InsertErr> {
        let result = self.node.set_rc(&Self::into(*value));
        Self::update_height_above(NonNull::new(self).unwrap());

        result
    }
}

impl<T, N: private::Node<T>> GetHeight<T> for HeightBinNode<T, N> {
    fn height(&self) -> usize {
        self.height
    }
}

impl<T, N: private::Node<T>> HeightNode<T> for HeightBinNode<T, N> {
    fn set_height(&mut self, value: &usize) {
        self.height = *value;
    }
}

impl<T, N: private::Node<T>> UpdateHeight<T> for HeightBinNode<T, N> {}
