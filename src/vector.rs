use libc::c_void;
use support::*;
use std::mem::size_of;
use std::ops::Index;
use std::ops::IndexMut;
use std::cmp::PartialEq;
use std::marker::PhantomData;

type	Rank = usize;
const	DEFAULT_CAPACITY: usize = 8;

pub struct Vector<T>
	where T: Clone + PartialEq {
	size: Rank,
	capacity: usize,
	ptr: *mut c_void,
	marker: PhantomData<T>
}

pub fn new<T>() -> Vector<T>
	where T: Clone + PartialEq {
	Vector {
		ptr: unsafe_malloc(size_of::<T>() * DEFAULT_CAPACITY),
		size: 0 as Rank,
		capacity: size_of::<T>() * DEFAULT_CAPACITY,
		marker: PhantomData
	}
}

pub fn from_slice<T>(slice: &[T]) -> Vector<T>
	where T: Clone + PartialEq {
	let new_ptr = unsafe_malloc(slice.len() * size_of::<T>());
	unsafe_memcpy(new_ptr, slice.as_ptr() as *mut c_void, slice.len() * size_of::<T>());
	Vector {
		ptr: new_ptr,
		size: slice.len(),
		capacity: size_of::<T>() * slice.len(),
		marker: PhantomData
	}
}

impl<T> Vector<T>
	where T: Clone + PartialEq {
	pub fn capacity(&self) -> usize {
		self.capacity
	}
	pub fn len(&self) -> Rank {
		self.size
	}
	pub fn empty(&self) -> bool {
		self.size == 0
	}
	pub fn find(&self, e: &T) -> Option<Rank> {
		for i in 0..self.size {
			if self[i] == *e {
				return Option::Some(i);
			}
		}
		Option::None
	}
	fn expand(&mut self) {
		if (self.size as usize) < self.capacity {
			return;
		}

		unsafe_realloc(self.ptr, self.capacity + DEFAULT_CAPACITY);
		self.capacity += DEFAULT_CAPACITY;
	}
	fn shrink(&mut self) {
		if self.capacity < 2*DEFAULT_CAPACITY || self.size*4 > self.capacity {
			return;
		}
		
		unsafe_realloc(self.ptr, self.capacity / 2);
		self.capacity /= 2;
	}
}

impl<T> Clone for Vector<T>
	where T: Clone + PartialEq {
	fn clone(&self) -> Self {
		let new_ptr = unsafe_malloc(self.capacity);

		unsafe_memcpy(new_ptr, self.ptr, self.size * size_of::<T>());
		Vector {
			ptr: new_ptr,
			size: self.size,
			capacity: self.capacity,
			marker: PhantomData
		}
	}
}

impl<T> Drop for Vector<T>
	where T: Clone + PartialEq {
	fn drop(&mut self) {
		unsafe_free(self.ptr as *mut c_void);
	}
}

impl<T> Index<Rank> for Vector<T>
	where T: Clone + PartialEq {
	type Output = T;

	fn index(&self, i: Rank) -> &T {
		unsafe {&(*((self.ptr as usize + i * size_of::<T>()) as *const T))}
	}
}

impl<T> IndexMut<Rank> for Vector<T>
	where T: Clone + PartialEq {
	fn index_mut(&mut self, i: Rank) -> &mut T {
		unsafe {&mut (*((self.ptr as usize + i * size_of::<T>()) as *mut T))}
	}
}
