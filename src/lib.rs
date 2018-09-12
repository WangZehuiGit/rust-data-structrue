extern crate libc;

use libc:: {
	c_void,
	size_t,
	malloc,
	free,
	realloc
};

#[inline]
fn unsafe_malloc(size: size_t) -> *mut c_void {
	unsafe {malloc(size)}
}

#[inline]
fn unsafe_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
	unsafe {realloc(ptr, size)}
}

#[inline]
fn unsafe_free(ptr: *mut c_void) {
	unsafe {free(ptr)};
}

pub mod vector;