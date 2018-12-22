use std::{cmp, ops};

pub trait Sort<I>: Clone
where
    I: Iterator + Copy,
    I::Item: ops::DerefMut,
    <I::Item as ops::Deref>::Target: Copy
{
    fn iter(&mut self) -> I;
    fn len(&self) -> usize;

    fn swap(mut a: I::Item, mut b: I::Item) {
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }

    fn bubble_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(I::Item, I::Item) -> cmp::Ordering
    {
        let it = self.iter();

        for len in (2..=self.len()).rev() {
            let mut it0 = it.take(len - 1);
            let mut it1 = it.take(len).skip(1);
            
            for (n0, n1) in it0.zip(it1) {
                if compare(n0, n1) == cmp::Ordering::Greater {
                    Self::swap(n0, n1);
                }
            }
        }
    }

    fn selection_sort<T, F>(&mut self, compare: F)
    where
        F: ops::Fn(I::Item, I::Item) -> cmp::Ordering
    {
        let it = self.iter();

        for (i, n) in it.take(self.len() - 1).enumerate() {
            let mut it0 = it.skip(i + 1);
            for n0 in it0 {
                if compare(n, n0) == cmp::Ordering::Greater {
                    Self::swap(n, n0);
                }
            }
        }
    }

    fn insertion_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(I::Item, I::Item) -> cmp::Ordering
    {
        let it = self.iter();

        for (i, n) in it.enumerate() {
            let mut it0 = it.take(i);

            for n0 in it0 {
                if compare(n0, n) == cmp::Ordering::Greater {
                    Self::swap(n0, n);

                    for n1 in it0 {
                        Self::swap(n1, n);
                    }
                }
            }
        }
    }

    fn shell_sort<T, F>(&mut self, compare: F)
    where
        F: ops::Fn(I::Item, I::Item) -> cmp::Ordering
    {
        let length = self.len();
        let mut h = 1usize;
        let it = self.iter();

        while h < length/3 {h = 3*h + 1;}

        while h >= 1 {
            let it = it.step_by(h);

            for (i, n) in it.enumerate() {
                let mut it0 = it.take(i);

                for n0 in it0 {
                    if compare(n0, n) == cmp::Ordering::Greater {
                        Self::swap(n0, n);

                        for n1 in it0 {
                            Self::swap(n1, n);
                        }
                    }
                }
            }

            h /= 3;
        }
    }

    fn merge<T, F>(data: &mut [T], mid: usize, mut compare: F)
        where T: cmp::Ord + Clone, F: ops::FnMut(&T, &T) -> cmp::Ordering {
        let mut i = 0usize;
        let mut j = mid + 1;
        let tmp_data = Vec::from(&data[..]);

        for k in 0..data.len() {
            if i > mid {
                data[k] = tmp_data[j].clone();
                j += 1;
            } else if j >= data.len() {
                data[k] = tmp_data[i].clone();
                i += 1;
            } else if compare(&tmp_data[i], &tmp_data[j]) == cmp::Ordering::Greater {
                data[k] = tmp_data[j].clone();
                j += 1;
            } else {
                data[k] = tmp_data[i].clone();
                i += 1;
            }
        }
    }

    fn merge_sort<T, F>(data: &mut [T], compare: F)
        where T: cmp::Ord + Clone, F: ops::FnMut(&T, &T) -> cmp::Ordering + Clone {
        let len = data.len();
        match len {
            0 | 1 => return,
            _ => ()
        }

        let mut cnt = 1usize;
        while (1 << cnt) < len {
            cnt += 1;
        }
        for s in (1..=cnt).map(|x| 1 << x) {
            for i in (1..=(len/s + 1)).map(|x| x * s) {
                let lo = i - s;
                let hi = if i <= len {i} else {len};
                merge(&mut data[lo..hi], s/2 - 1, compare.clone());
            }
        }
    }

    fn quick_sort<T, F>(data: &mut [T], compare: F)
        where T: cmp::Ord + Clone, F: ops::FnMut(&T, &T) -> cmp::Ordering + Clone {
        if data.len() <= 1 {
            return;
        }

        let mid = partition(&mut data[..], compare.clone());

        quick_sort(&mut data[0..mid], compare.clone());
        quick_sort(&mut data[(mid+1)..], compare.clone());
    }

    fn partition<T, F>(data: &mut [T], mut compare: F) -> usize
        where T: cmp::Ord + Clone, F: ops::FnMut(&T, &T) -> cmp::Ordering {
        let mut i = 0usize;
        let mut j = data.len() - 1;
        let point = data[0].clone();

        loop {
            loop {
                if i == data.len() - 1 || compare(&point, &data[i]) != cmp::Ordering::Greater {
                    break;
                }
                i += 1;
            }
            loop {
                if j == 0 || compare(&data[j], &point) != cmp::Ordering::Greater {
                    break;
                }
                j -= 1;
            }
            if i == j {
                break;
            }
            data.swap(i, j);
        }

        i
    }
}
