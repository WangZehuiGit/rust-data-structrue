use support::*;
use std::mem::size_of;

type	Rank = i32;
const	DEFAULT_CAPACITY: usize = 8;

pub struct Vector<T> {
	size: Rank,
	capacity: usize,
	ptr: *mut T
}


pub fn new<T>() -> Vector<T> {
	Vector::<T> {
		ptr: unsafe_malloc(size_of::<T>() * DEFAULT_CAPACITY) as *mut T,
		size: 0 as Rank,
		capacity: size_of::<T>() * DEFAULT_CAPACITY
	}
}

impl<T> Vector<T> {
	pub fn capacity(&self) -> usize {
		self.capacity
	}
}