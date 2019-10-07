use std::cmp::Ordering;
use std::ops::IndexMut;

pub trait Search<K: Copy, V> {
    fn search(&mut self, key: K) -> &mut V;
}

pub trait Ordered<T: Ord> {
    fn push(&mut self, value: &T);
}

pub fn binary_search<'a, K, E, L, F>(
    data: &'a mut L,
    key: K,
    cmp: F,
    lo: usize,
    hi: usize,
) -> Option<&'a mut L::Output>
where
    K: Copy,
    E: Ord,
    L: 'a + IndexMut<usize> + Ordered<E>,
    L::Output: Sized,
    F: Fn(K, &L::Output) -> Ordering,
{
    if lo >= hi {
        return None;
    }

    let mid = (lo + hi) / 2;
    match cmp(key, data.index(mid)) {
        Ordering::Equal => Some(data.index_mut(mid)),
        Ordering::Less => binary_search(data, key, cmp, lo, mid),
        Ordering::Greater => binary_search(data, key, cmp, mid + 1, hi),
    }
}
