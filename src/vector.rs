use libc::c_void;
use support::*;
use std::mem::size_of;

type	Rank = i32;
const	DEFAULT_CAPACITY: usize = 8;

pub struct Vector<T>
	where T: Clone {
	size: Rank,
	capacity: usize,
	ptr: *mut T
}


pub fn new<T>() -> Vector<T>
	where T: Clone {
	Vector::<T> {
		ptr: unsafe_malloc(size_of::<T>() * DEFAULT_CAPACITY) as *mut T,
		size: 0 as Rank,
		capacity: size_of::<T>() * DEFAULT_CAPACITY
	}
}

impl<T> Vector<T>
	where T: Clone {
	pub fn capacity(&self) -> usize {
		self.capacity
	}
	pub fn len(&self) -> Rank {
		self.size
	}
	pub fn empty(&self) -> bool {
		self.size == 0
	}
}

impl<T> Clone for Vector<T>
	where T: Clone {
	fn clone(&self) -> Self {
		let new_ptr = unsafe_malloc(self.capacity);

		for i in 0..self.size {
			unsafe {*((new_ptr as i32 + i) as *mut T) = (*((self.ptr as i32 + i) as *mut T)).clone()}
		}
		Vector::<T> {
			ptr: self.ptr,
			size: self.size,
			capacity: self.capacity
		}
	}
}

impl<T> Drop for Vector<T>
	where T: Clone {
	fn drop(&mut self) {
		unsafe_free(self.ptr as *mut c_void);
	}
}