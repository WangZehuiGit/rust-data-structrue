use std::{cmp, ops};

pub fn bubble_sort<T, F>(data: &mut [T], mut compare: F)
    where T: cmp::Ord, F: ops::FnMut(&T, &T) -> cmp::Ordering {
    for i in 1..data.len() {
        for j in 0..(data.len()-i) {
            if compare(&data[j], &data[j+1]) == cmp::Ordering::Greater {
                data.swap(j+1, j);
            }
        }
    }
}

pub fn selection_sort<T, F>(data: &mut [T], mut compare: F)
    where T: cmp::Ord, F: ops::FnMut(&T, &T) -> cmp::Ordering {
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if compare(&data[i], &data[j]) == cmp::Ordering::Greater {
                data.swap(i, j);
            }
        }
    }
}

pub fn insertion_sort<T, F>(data: &mut [T], mut compare: F)
    where T: cmp::Ord, F: ops::FnMut(&T, &T) -> cmp::Ordering {
    for i in 1..data.len() {
        for j in (1..=i).rev() {
            if compare(&data[j-1], &data[j]) == cmp::Ordering::Greater {
                data.swap(j-1, j);
            } else {
                break;
            }
        }
    }
}

pub fn shell_sort<T, F>(data: &mut [T], mut compare: F)
    where T: cmp::Ord, F: ops::FnMut(&T, &T) -> cmp::Ordering {
    let length = data.len();
    let mut h = 1usize;

    while h < length/3 {h = 3*h + 1;}
    while h >= 1 {
        for i in h..length {
            for j in ((1..=i/h).map(|x| x*h + i%h)).rev() {
                if compare(&data[j-h], &data[j]) == cmp::Ordering::Greater {
                    data.swap(j-h, j);
                } else {
                    break;
                }
            }
        }

        h /= 3;
    }
}

pub fn merge_sort<T, F>(data: &mut [T], compare: F)
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

pub fn quick_sort<T, F>(data: &mut [T], compare: F)
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
