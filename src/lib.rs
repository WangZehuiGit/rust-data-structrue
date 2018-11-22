use std::alloc:: {
	alloc,
	dealloc,
	Layout,
	LayoutErr
};
use std::mem::{align_of, size_of};
use std::ptr;

pub mod vector;
pub mod list;

pub trait PriorityQueue<V: std::cmp::PartialOrd> {
    fn insert(&mut self, value: &V);
    fn size(&self) -> usize;
    fn del_max(&mut self) -> V;
    fn get_max(&self) -> &V;
}

fn malloc<T>(capacity: usize) -> Result<*mut T, LayoutErr> {
	let result = Layout::from_size_align(capacity * size_of::<T>(), align_of::<T>());
	match result {
		Err(layout_err) => return Err(layout_err),
		Ok(layout) => unsafe {Ok(alloc(layout) as *mut T)}
	}
}

fn malloc_val<T>(value: &T) -> *mut T {
	let layout = Layout::for_value(value);
	let ptr = unsafe {alloc(layout) as *mut T};
	unsafe {*ptr = ptr::read(value as *const T)};
	ptr
}

fn free<T>(ptr: *mut T, capacity: usize) -> Result<(), LayoutErr> {
	let layout = Layout::from_size_align(capacity * size_of::<T>(), align_of::<T>())?;
	unsafe {dealloc(ptr as *mut u8, layout);}
	Ok(())
}
