use std::alloc:: {
	alloc,
	dealloc,
	Layout
};
use std::ptr;
use std::mem::size_of;
use std::ops::Index;
use std::ops::IndexMut;
use std::cmp::PartialEq;

type	Rank = usize;
const	DEFAULT_CAPACITY: usize = 8;

pub struct Vector<T>
	where T: Clone + PartialEq {
	size: Rank,
	capacity: usize,
	ptr: *mut T
}	

pub fn new<T>() -> Vector<T>
	where T: Clone + PartialEq {
	
	Vector {
		ptr: unsafe {alloc(Layout::from_size_align(DEFAULT_CAPACITY, size_of::<T>()).unwrap())} as *mut T,
		size: 0 as Rank,
		capacity: DEFAULT_CAPACITY,
	}
}

pub fn from_slice<T>(slice: &[T]) -> Vector<T>
	where T: Clone + PartialEq {
	let new_ptr = unsafe {alloc(Layout::from_size_align(slice.len(), size_of::<T>()).unwrap())} as *mut T;
	unsafe {ptr::copy(slice.as_ptr(), new_ptr, slice.len());}
	Vector {
		ptr: new_ptr,
		size: slice.len(),
		capacity: slice.len()
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

		unsafe {
			dealloc(self.ptr as *mut u8, Layout::from_size_align(self.capacity, size_of::<T>()).unwrap());
			self.ptr = alloc(Layout::from_size_align(self.capacity * 2, size_of::<T>()).unwrap()) as *mut T;
		}
		self.capacity *= 2;
	}
	fn shrink(&mut self) {
		if self.capacity < 2*DEFAULT_CAPACITY || self.size*4 > self.capacity {
			return;
		}
		
		unsafe {
			dealloc(self.ptr as *mut u8, Layout::from_size_align(self.capacity, size_of::<T>()).unwrap());
			self.ptr = alloc(Layout::from_size_align(self.capacity / 2, size_of::<T>()).unwrap()) as *mut T;
		}
		self.capacity /= 2;
	}
}

impl<T> Clone for Vector<T>
	where T: Clone + PartialEq {
	fn clone(&self) -> Self {
		let new_ptr = unsafe {alloc(Layout::from_size_align(self.capacity, size_of::<T>()).unwrap())} as *mut T;

		unsafe {ptr::copy(self.ptr, new_ptr, self.size);}
		Vector {
			ptr: new_ptr,
			size: self.size,
			capacity: self.capacity
		}
	}
}

impl<T> Drop for Vector<T>
	where T: Clone + PartialEq {
	fn drop(&mut self) {
		unsafe {dealloc(self.ptr as *mut u8, Layout::from_size_align(self.capacity, size_of::<T>()).unwrap());}
	}
}

impl<T> Index<Rank> for Vector<T>
	where T: Clone + PartialEq {
	type Output = T;

	fn index(&self, i: Rank) -> &T {
		unsafe {&(*(self.ptr.add(i)))}
	}
}

impl<T> IndexMut<Rank> for Vector<T>
	where T: Clone + PartialEq {
	fn index_mut(&mut self, i: Rank) -> &mut T {
		unsafe {&mut (*(self.ptr.add(i)))}
	}
}
