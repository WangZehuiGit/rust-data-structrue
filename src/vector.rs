type	Rank = i32;
const	DEFAULT_CAPACITY: u32 = 8;

struct Vector<T> {
	size: Rank,
	capacity: usize,
	elem: *mut T
}