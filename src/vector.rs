type	Rank = i32;
const	DEFAULT_CAPACITY: usize = 8;

struct vector<T> {
	size: Rank,
	capacity: usize,
	elem: *mut T
}