use std::alloc::{alloc, dealloc, Layout, LayoutErr};
use std::mem::{align_of, size_of};
use std::ptr;

pub fn malloc<T>(capacity: usize) -> Result<*mut T, LayoutErr> {
    let layout = Layout::from_size_align(capacity * size_of::<T>(), align_of::<T>())?;
    unsafe { Ok(alloc(layout) as *mut T) }
}

pub fn malloc_val<T>(value: &T) -> *mut T {
    let layout = Layout::for_value(value);
    let ptr = unsafe { alloc(layout) as *mut T };
    unsafe { *ptr = ptr::read(value as *const T) };
    ptr
}

pub fn free<T>(ptr: *mut T, capacity: usize) -> Result<(), LayoutErr> {
    let layout = Layout::from_size_align(capacity * size_of::<T>(), align_of::<T>())?;
    unsafe { dealloc(ptr as *mut u8, layout) };
    Ok(())
}