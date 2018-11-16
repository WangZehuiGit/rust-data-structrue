use std::alloc:: {
	alloc,
	dealloc,
	Layout
};
use std::ptr;
use std::mem::{align_of, size_of};
use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::cmp::PartialEq;
use std::fmt;
use std::iter::Iterator;

type	Rank = usize;
const	DEFAULT_CAPACITY: usize = 8;

pub struct Vector<T>
	where T: Clone + PartialEq {
	len: Rank,
	capacity: usize,
	ptr: *mut T
}	

pub fn new<T>() -> Vector<T>
	where T: Clone + PartialEq {
	let (size, align) = (
		DEFAULT_CAPACITY * size_of::<T>(),
		align_of::<T>()
	);

	Vector {
		ptr: unsafe {alloc(Layout::from_size_align(size, align).unwrap())} as *mut T,
		len: 0 as Rank,
		capacity: DEFAULT_CAPACITY,
	}
}

pub fn from_slice<T>(slice: &[T]) -> Vector<T>
	where T: Clone + PartialEq {
	let (size, align) = (
		slice.len() * size_of::<T>(),
		align_of::<T>()
	);
	let new_ptr = unsafe {alloc(Layout::from_size_align(size, align).unwrap())} as *mut T;
	unsafe {ptr::copy(slice.as_ptr(), new_ptr, slice.len());}
	Vector {
		ptr: new_ptr,
		len: slice.len(),
		capacity: slice.len()
	}
}

impl<T> Vector<T>
	where T: Clone + PartialEq {
	pub fn capacity(&self) -> usize {
		self.capacity
	}
	pub fn len(&self) -> Rank {
		self.len
	}
	pub fn empty(&self) -> bool {
		self.len == 0
	}
	pub fn find(&self, e: &T) -> Option<Rank> {
		for i in 0..self.len {
			if self[i] == *e {
				return Option::Some(i);
			}
		}
		Option::None
	}
	pub fn insert(&mut self, rank: Rank, value: &T) {
		self.expand();
		self.len += 1;
		for i in (rank..(self.len-1)).rev() {
			self[i+1] = self[i].clone();
		}
		self[rank] = value.clone();
	}
	pub fn remove(&mut self, mut lo: Rank, mut hi: Rank) {
		let size = hi - lo;
		while hi < self.len {
			self[lo] = self[hi].clone();
			lo += 1;
			hi += 1;
		}
		self.shrink();
		self.len -= size;
	}
	fn expand(&mut self) {
		if (self.len as usize) < self.capacity {
			return;
		}
		let (size, align) = (
			self.capacity * size_of::<T>(),
			align_of::<T>()
		);
		unsafe {
			let new_ptr = alloc(Layout::from_size_align(size * 2, align).unwrap()) as *mut T;
			ptr::copy(self.ptr, new_ptr, self.len);
			dealloc(self.ptr as *mut u8, Layout::from_size_align(size, align).unwrap());
			self.ptr = new_ptr;
		}
		self.capacity *= 2;
	}
	fn shrink(&mut self) {
		if self.capacity < 2*DEFAULT_CAPACITY || self.len*4 > self.capacity {
			return;
		}
		let (size, align) = (
			self.capacity * size_of::<T>(),
			align_of::<T>()
		);
		unsafe {
			let new_ptr = alloc(Layout::from_size_align(size / 2, align).unwrap()) as *mut T;
			ptr::copy(self.ptr, new_ptr, self.len);
			dealloc(self.ptr as *mut u8, Layout::from_size_align(size, align).unwrap());
			self.ptr = new_ptr;
		}
		self.capacity /= 2;
	}
}

impl<T> Clone for Vector<T>
	where T: Clone + PartialEq {
	fn clone(&self) -> Self {
		let (size, align) = (
			self.capacity * size_of::<T>(),
			align_of::<T>()
		);
		let new_ptr = unsafe {alloc(Layout::from_size_align(size, align).unwrap())} as *mut T;

		unsafe {ptr::copy(self.ptr, new_ptr, self.len);}
		Vector {
			ptr: new_ptr,
			len: self.len,
			capacity: self.capacity
		}
	}
}

impl<T> Drop for Vector<T>
	where T: Clone + PartialEq {
	fn drop(&mut self) {
		let (size, align) = (
			self.capacity * size_of::<T>(),
			align_of::<T>()
		);
		unsafe {dealloc(self.ptr as *mut u8, Layout::from_size_align(size, align).unwrap());}
	}
}

impl<T> Index<Rank> for Vector<T>
	where T: Clone + PartialEq {
	type Output = T;

	fn index(&self, i: Rank) -> &T {
		if i >= self.len {
			panic!("array bound!");
		}
		unsafe {&(*(self.ptr.add(i)))}
	}
}

impl<T> IndexMut<Rank> for Vector<T>
	where T: Clone + PartialEq {
	fn index_mut(&mut self, i: Rank) -> &mut T {
		if i >= self.len {
			panic!("array bound!");
		}
		unsafe {&mut (*(self.ptr.add(i)))}
	}
}

impl<T> Deref for Vector<T>
	where T: Clone + PartialEq {
	type Target = [T];

	fn deref(&self) -> &[T] {
		unsafe {std::slice::from_raw_parts(self.ptr, self.len)}
	}
}

impl<T> DerefMut for Vector<T>
	where T: Clone + PartialEq {
	fn deref_mut(&mut self) -> &mut [T] {
		unsafe {std::slice::from_raw_parts_mut(self.ptr, self.len)}
	}
}

impl<T> PartialEq for Vector<T>
	where T: Clone + PartialEq {
	fn eq(&self, other: &Vector<T>) -> bool {
		if self.len != other.len {
			return false;
		}
		for i in 0..self.len {
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
		let mut it = 0..self.len;

		s.push_str(&format!("[{:?}", self[it.next().unwrap()]));
		
		for i in it {
			s.push_str(&format!(", {:?}", self[i]));
		}
		s.push(']');

		write!(f, "{}", s)
	}
}
