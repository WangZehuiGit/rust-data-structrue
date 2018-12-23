use std::{cmp, ops};
use std::iter::FromIterator;

fn swap<R>(mut a: R, mut b: R)
where
    R: ops::DerefMut,
    R::Target: Copy
{
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn merge<I, F>(it: I, mid: usize, compare: F)
where
    I: Iterator + Clone,
    I::Item: ops::DerefMut,
    <I::Item as ops::Deref>::Target: Copy,
    F: ops::Fn(
        &<I::Item as ops::Deref>::Target,
        &<I::Item as ops::Deref>::Target
    ) -> cmp::Ordering
{
    let mut i = 0usize;
    let mut j = mid + 1;
    let tmp_data = Vec::from_iter(it.clone().map(|x| *x));

    for mut n in it {
        if i > mid {
            *n = tmp_data[j];
            j += 1;
        } else if j >= tmp_data.len() {
            *n = tmp_data[i];
            i += 1;
        } else if compare(&tmp_data[i], &tmp_data[j]) == cmp::Ordering::Greater {
            *n = tmp_data[j];
            j += 1;
        } else {
            *n = tmp_data[i];
            i += 1;
        }
    }
}

fn partition<I, F>(mut it: I, compare: F) -> usize
where
    I: Iterator + Clone,
    I::Item: ops::DerefMut,
    <I::Item as ops::Deref>::Target: Copy,
    F: ops::Fn(
        &<I::Item as ops::Deref>::Target,
        &<I::Item as ops::Deref>::Target
    ) -> cmp::Ordering
{
    let mut point = it.next().unwrap();
    let mut it0 = it.clone();
    let mut next = it0.next().unwrap();
    let mut count = it.clone().count();
    let point_value = *point;

    while compare(&point_value, &next) == cmp::Ordering::Greater {
        swap(&mut *next, &mut *point);
        count -= 1;
        point = it.next().unwrap();
        if let Some(n) = it0.next() {
            next = n;
        } else {
            break;
        }
    }

    for mut n in it0.clone() {
        if compare(&point_value, &n) == cmp::Ordering::Greater {
            swap(&mut *next, &mut *n);
            swap(&mut *point, &mut *next);
            count -= 1;
            point = it.next().unwrap();
            next = it0.next().unwrap();
        }
    }

    count
}

fn quick_sort_rec<I, F>(it: I, compare: F, lo: usize, hi: usize)
where
    I: Iterator + Clone,
    I::Item: ops::DerefMut,
    <I::Item as ops::Deref>::Target: Copy,
    F: ops::Fn(
        &<I::Item as ops::Deref>::Target,
        &<I::Item as ops::Deref>::Target
    ) -> cmp::Ordering
    + Copy
{
    if hi - lo <= 1 {
        return;
    }

    let mid = hi - partition(it.clone().skip(lo).take(hi - lo), compare);

    quick_sort_rec(it.clone(), compare, lo, mid - 1);
    quick_sort_rec(it.clone(), compare, mid, hi);
}

pub trait Sort<I>
where
    I: Iterator + Copy,
    I::Item: ops::DerefMut,
    <I::Item as ops::Deref>::Target: Copy
{
    fn iter(&mut self) -> I;
    fn len(&self) -> usize;

    fn bubble_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(
            &<I::Item as ops::Deref>::Target,
            &<I::Item as ops::Deref>::Target
        ) -> cmp::Ordering
    {
        let it = self.iter();

        for len in (2..=self.len()).rev() {
            let mut it0 = it.take(len - 1);
            let mut it1 = it.take(len).skip(1);
            
            for (n0, n1) in it0.zip(it1) {
                if compare(&n0, &n1) == cmp::Ordering::Greater {
                    swap(n0, n1);
                }
            }
        }
    }

    fn selection_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(
            &<I::Item as ops::Deref>::Target,
            &<I::Item as ops::Deref>::Target
        ) -> cmp::Ordering
    {
        let it = self.iter();

        for (i, mut n) in it.take(self.len() - 1).enumerate() {
            let mut it0 = it.skip(i + 1);
            for mut n0 in it0 {
                if compare(&n, &n0) == cmp::Ordering::Greater {
                    swap(&mut *n, &mut *n0);
                }
            }
        }
    }

    fn insertion_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(
            &<I::Item as ops::Deref>::Target,
            &<I::Item as ops::Deref>::Target
        ) -> cmp::Ordering
    {
        let it = self.iter();

        for (i, mut n) in it.enumerate() {
            let mut it0 = it.take(i);

            while let Some(mut n0) = it0.next() {
                if compare(&n0, &n) == cmp::Ordering::Greater {
                    swap(&mut *n0, &mut *n);
                    break;
                }
            }

            for mut n1 in it0 {
                swap(&mut *n1, &mut *n);
            }
        }
    }

    fn shell_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(
            &<I::Item as ops::Deref>::Target,
            &<I::Item as ops::Deref>::Target
        ) -> cmp::Ordering
    {
        let length = self.len();
        let mut h = 1usize;
        let it = self.iter();

        while h < length/3 {h = 3*h + 1;}

        while h >= 1 {
            let it0 = it.step_by(h);

            for (i, mut n) in it0.clone().enumerate() {
                let mut it1 = it0.clone().take(i);

                while let Some(mut n0) = it1.next() {
                    if compare(&n0, &n) == cmp::Ordering::Greater {
                        swap(&mut *n0, &mut *n);
                        break;
                    }
                }

                for mut n1 in it1 {
                    swap(&mut *n1, &mut *n);
                }
            }

            h /= 3;
        }
    }

    fn merge_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(
            &<I::Item as ops::Deref>::Target,
            &<I::Item as ops::Deref>::Target
        ) -> cmp::Ordering
    {
        let len = self.len();
        match len {
            0 | 1 => return,
            _ => ()
        }

        let it = self.iter();

        for s in (1..).map(|x| 1 << x).take_while(|x| *x < len * 2) {
            for i in (0..len).step_by(s) {
                merge(it.skip(i).take(s), s/2 - 1, &compare);
            }
        }
    }

    fn quick_sort<F>(&mut self, compare: F)
    where
        F: ops::Fn(
            &<I::Item as ops::Deref>::Target,
            &<I::Item as ops::Deref>::Target
        ) -> cmp::Ordering
    {
        quick_sort_rec(self.iter(), &compare, 0, self.len());
    }
}
