use orx_split_vec::*;
use std::cmp::Ordering;

#[test]
fn sort() {
    let mut vec: SplitVec<_> = [3, 4, 1, 2, 3, 5, 8, 10, 9].into_iter().collect();
    vec.sort();
    assert_eq!(&vec, &[1, 2, 3, 3, 4, 5, 8, 9, 10]);
}

#[test]
fn sort_by() {
    let compare = |a: &i32, b: &i32| match a.cmp(b) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => Ordering::Equal,
    };
    let mut vec: SplitVec<_> = [3, 4, 1, 2, 3, 5, 8, 10, 9].into_iter().collect();
    vec.sort_by(compare);
    assert_eq!(&vec, &[10, 9, 8, 5, 4, 3, 3, 2, 1]);
}

#[test]
fn sort_by_key() {
    let key = |a: &i32| match a % 2 {
        0 => *a,
        _ => -a,
    };

    let mut vec: SplitVec<_> = [3, 4, 1, 2, 3, 5, 8, 10, 9].into_iter().collect();
    vec.sort_by_key(key);
    assert_eq!(&vec, &[9, 5, 3, 3, 1, 2, 4, 8, 10]);
}
