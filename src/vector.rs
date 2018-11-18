use super::{malloc, free};
use std::ptr;
use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::cmp::PartialEq;
use std::fmt;

type	Rank = usize;
const	DEFAULT_CAPACITY: usize = 8;

pub struct Vector<T>
	where T: Clone + PartialEq
{
	len: Rank,
	capacity: usize,
	ptr: *mut T
}	

impl<T> Vector<T>
where
	T: Clone + PartialEq 
{
	pub fn new() -> Vector<T> {
		Vector {
			ptr: malloc(DEFAULT_CAPACITY).unwrap(),
			len: 0 as Rank,
			capacity: DEFAULT_CAPACITY,
		}
	}

	pub fn from_slice(slice: &[T]) -> Vector<T> {
		let new_ptr = malloc(slice.len()).unwrap();
		unsafe {ptr::copy(slice.as_ptr(), new_ptr, slice.len());}
		Vector {
			ptr: new_ptr,
			len: slice.len(),
			capacity: slice.len()
		}
	}

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
		unsafe {
			let new_ptr = malloc(self.capacity).unwrap();
			ptr::copy(self.ptr, new_ptr, self.len);
			free(self.ptr, self.capacity).unwrap();
			self.ptr = new_ptr;
		}
		self.capacity *= 2;
	}

	fn shrink(&mut self) {
		if self.capacity < 2*DEFAULT_CAPACITY || self.len*4 > self.capacity {
			return;
		}
		unsafe {
			let new_ptr = malloc(self.capacity).unwrap();
			ptr::copy(self.ptr, new_ptr, self.len);
			free(self.ptr, self.capacity).unwrap();
			self.ptr = new_ptr;
		}
		self.capacity /= 2;
	}
}

impl<T> Clone for Vector<T>
where
	T: Clone + PartialEq
{
	fn clone(&self) -> Self {
		let new_ptr = malloc(self.capacity).unwrap();

		unsafe {ptr::copy(self.ptr, new_ptr, self.len);}
		Vector {
			ptr: new_ptr,
			len: self.len,
			capacity: self.capacity
		}
	}
}

impl<T> Drop for Vector<T>
where
	T: Clone + PartialEq 
{
	fn drop(&mut self) {
		free(self.ptr, self.capacity).unwrap();
	}
}

impl<T> Index<Rank> for Vector<T>
where
	T: Clone + PartialEq 
{
	type Output = T;

	fn index(&self, i: Rank) -> &T {
		if i >= self.len {
			panic!("array bound!");
		}
		unsafe {&(*(self.ptr.add(i)))}
	}
}

impl<T> IndexMut<Rank> for Vector<T>
where
	T: Clone + PartialEq 
{
	fn index_mut(&mut self, i: Rank) -> &mut T {
		if i >= self.len {
			panic!("array bound!");
		}
		unsafe {&mut (*(self.ptr.add(i)))}
	}
}

impl<T> Deref for Vector<T>
where
	T: Clone + PartialEq 
{
	type Target = [T];

	fn deref(&self) -> &[T] {
		unsafe {std::slice::from_raw_parts(self.ptr, self.len)}
	}
}

impl<T> DerefMut for Vector<T>
where
	T: Clone + PartialEq 
{
	fn deref_mut(&mut self) -> &mut [T] {
		unsafe {std::slice::from_raw_parts_mut(self.ptr, self.len)}
	}
}

impl<T> PartialEq for Vector<T>
where
	T: Clone + PartialEq 
{
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
where
	T: Clone + PartialEq + fmt::Debug 
{
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

pub struct PriorityQueueVector<T>
where
	T: Clone + PartialEq 
{
	vec: Vector<T>
}

impl<T> PriorityQueueVector<T>
where
	T: Clone + PartialEq 
{
	pub fn new() -> PriorityQueueVector<T> {
		PriorityQueueVector {
			vec: Vector::<T>::new()
		}
	}
}

impl<T> super::PriorityQueue<T> for PriorityQueueVector<T>
where
	T: Clone + PartialOrd 
{
	fn size(&self) -> usize {
		self.vec.len()
	}

	fn insert(&mut self, v: &T) {
		let size = self.vec.len();
		self.vec.insert(size, v);
	}

	fn get_max(&self) -> &T {
		let mut max_index = 0usize;
		for i in 1..self.vec.len() {
			if self.vec[i] > self.vec[max_index] {
				max_index = i;
			}
		}
		&(self.vec[max_index])
	}

	fn del_max(&mut self) -> T {
		let mut max_index = 0usize;
		for i in 1..self.vec.len() {
			if self.vec[i] > self.vec[max_index] {
				max_index = i;
			}
		}
		let max = self.vec[max_index].clone();
		self.vec.remove(max_index, max_index + 1);
		max
	}
}

#[macro_export]
macro_rules! pq_vec {
	($($e: expr), *) => (
		{
			let mut temp = PriorityQueueVector::new();
			$(
				temp.insert(&$e);
			)*
			temp
		}
	)
}
