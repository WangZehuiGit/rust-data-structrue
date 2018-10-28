use libc:: {
	c_void,
	size_t,
	malloc,
	free,
	realloc,
	memcpy
};

#[inline]
pub fn unsafe_malloc(size: size_t) -> *mut c_void {
	unsafe {malloc(size)}
}

#[inline]
pub fn unsafe_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
	unsafe {realloc(ptr, size)}
}

#[inline]
pub fn unsafe_free(ptr: *mut c_void) {
	unsafe {free(ptr)};
}

#[inline]
pub fn unsafe_memcpy(dest: *mut c_void, src: *mut c_void, n: size_t) {
	unsafe {memcpy(dest, src, n)};
}