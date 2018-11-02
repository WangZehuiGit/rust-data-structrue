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
use std::fmt;
use std::iter::Iterator;

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
	pub fn insert(&mut self, rank: Rank, value: &T) {
		self.expand();

		for i in (rank..self.size).rev() {
			self[i+1] = self[i].clone();
		}
		self[rank] = value.clone();
		self.size += 1;
	}
	pub fn remove(&mut self, mut lo: Rank, mut hi: Rank) {
		let size = hi - lo;
		while hi < self.size {
			self[lo] = self[hi].clone();
			lo += 1;
			hi += 1;
		}
		self.shrink();
		self.size = size;
	}
	fn expand(&mut self) {
		if (self.size as usize) < self.capacity {
			return;
		}

		unsafe {
			let new_ptr = alloc(Layout::from_size_align(self.capacity * 2, size_of::<T>()).unwrap()) as *mut T;
			ptr::copy(self.ptr, new_ptr, self.size);
			dealloc(self.ptr as *mut u8, Layout::from_size_align(self.capacity, size_of::<T>()).unwrap());
			self.ptr = new_ptr;
		}
		self.capacity *= 2;
	}
	fn shrink(&mut self) {
		if self.capacity < 2*DEFAULT_CAPACITY || self.size*4 > self.capacity {
			return;
		}
		
		unsafe {
			let new_ptr = alloc(Layout::from_size_align(self.capacity / 2, size_of::<T>()).unwrap()) as *mut T;
			ptr::copy(self.ptr, new_ptr, self.size);
			dealloc(self.ptr as *mut u8, Layout::from_size_align(self.capacity, size_of::<T>()).unwrap());
			self.ptr = new_ptr;
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
		if i >= self.size {
			panic!("array bound!");
		}
		unsafe {&(*(self.ptr.add(i)))}
	}
}

impl<T> IndexMut<Rank> for Vector<T>
	where T: Clone + PartialEq {
	fn index_mut(&mut self, i: Rank) -> &mut T {
		if i >= self.size {
			panic!("array bound!");
		}
		unsafe {&mut (*(self.ptr.add(i)))}
	}
}

impl<T> PartialEq for Vector<T>
	where T: Clone + PartialEq {
	fn eq(&self, other: &Vector<T>) -> bool {
		if self.size != other.size {
			return false;
		}
		for i in 0..self.size {
			if self[i] != other[i] {
				return false;
			}
		}
		true
	}
}

impl<T> fmt::Debug for Vector<T>
	where T: Clone + PartialEq + fmt::Debug {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();
		let mut it = 0..self.size;

		s.push_str(&format!("[{:?}", self[it.next().unwrap()]));
		
		for i in it {
			s.push_str(&format!(", {:?}", self[i]));
		}
		s.push(']');

		write!(f, "{}", s)
	}
}
