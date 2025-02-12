use crate::{test_all_growth_types, Growth, SplitVec};
use alloc::vec::Vec;
use orx_pinned_vec::*;
use test_case::test_matrix;

#[test]
fn iter() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        let n = 564;
        let std_vec: Vec<_> = (0..n).collect();
        vec.extend(std_vec);

        for (i, x) in vec.iter().enumerate() {
            assert_eq!(i, *x);
        }
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_empty_split_vec() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        let mut iter = vec.iter();
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_empty_first_fragment() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        vec.push(0);
        _ = vec.pop();
        assert!(vec.is_empty());

        let mut iter = vec.iter();
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
    test_all_growth_types!(test);
}

#[test]
fn iter_one_fragment() {
    fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
        vec.clear();
        vec.push(0);
        vec.push(1);

        assert_eq!(alloc::vec![0, 1], vec.iter().copied().collect::<Vec<_>>());
    }
    test_all_growth_types!(test);
}

fn init_vec<G: Growth>(mut vec: SplitVec<usize, G>, n: usize) -> SplitVec<usize, G> {
    vec.clear();
    vec.extend(0..n);
    vec
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn clone_whole(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);

    let iter1 = vec.iter();
    let iter2 = iter1.clone();

    for (i, (a, b)) in iter1.zip(iter2).enumerate() {
        assert_eq!(i, *a);
        assert_eq!(i, *b);
    }
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 3, 4, 5, 8, 27, 423]
)]
fn clone_used(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);
    let num_used = n / 2;

    let mut iter1 = vec.iter();
    for _ in 0..num_used {
        _ = iter1.next();
    }
    let iter2 = iter1.clone();

    for (i, (a, b)) in iter1.zip(iter2).enumerate() {
        assert_eq!(i + num_used, *a);
        assert_eq!(i + num_used, *b);
    }
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn all(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);

    assert!(vec.iter().all(|x| *x as isize >= -1));
    assert!(vec.is_empty() || !vec.iter().all(|x| *x < n - 1));
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn any(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);

    assert!(!vec.iter().any(|x| *x as isize <= -1));
    assert!(vec.is_empty() || vec.iter().any(|x| *x >= n / 2));
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn count(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);
    let num_used = n / 2;

    assert_eq!(vec.iter().count(), n);

    let mut iter = vec.iter();
    for _ in 0..num_used {
        _ = iter.next();
    }
    assert_eq!(iter.count(), n - num_used);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn fold(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);

    let sum = vec.iter().fold(0isize, |x, b| {
        if b % 2 == 0 {
            x + *b as isize
        } else {
            x - *b as isize
        }
    });

    let expected = (0..n).filter(|x| x % 2 == 0).sum::<usize>() as isize
        - (0..n).filter(|x| x % 2 == 1).sum::<usize>() as isize;

    assert_eq!(sum, expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn last(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);

    let expected = match n {
        0 => None,
        _ => Some(n - 1),
    };
    assert_eq!(vec.iter().last().copied(), expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn max(vec: SplitVec<usize, impl Growth>, n: usize) {
    let mut vec = init_vec(vec, n);

    let expected = match n {
        0 => None,
        _ => Some(n - 1),
    };
    assert_eq!(vec.iter().max().copied(), expected);

    if let Some(x) = vec.get_mut(n / 2) {
        *x = n + 1;
        assert_eq!(vec.iter().max().copied(), Some(n + 1));
    };

    let mut vec = init_vec(vec, n);
    if let Some(x) = vec.get_mut(4) {
        *x = n + 1;
        assert_eq!(vec.iter().max().copied(), Some(n + 1));
    };
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn min(vec: SplitVec<usize, impl Growth>, n: usize) {
    let mut vec = init_vec(vec, n);

    let expected = match n {
        0 => None,
        _ => Some(0),
    };
    assert_eq!(vec.iter().min().copied(), expected);

    for x in vec.iter_mut() {
        *x += n;
    }
    if let Some(x) = vec.get_mut(n / 2) {
        *x = 1;
        assert_eq!(vec.iter().min().copied(), Some(1));
    };

    if let Some(x) = vec.get_mut(4) {
        *x = 0;
        assert_eq!(vec.iter().min().copied(), Some(0));
    };
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423],
    [0, 3, 4, 8, 124, 99]
)]
fn nth(vec: SplitVec<usize, impl Growth>, n: usize, nth: usize) {
    let vec = init_vec(vec, n);

    let expected = match nth < n {
        true => Some(nth),
        false => None,
    };
    assert_eq!(vec.iter().nth(nth).copied(), expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423],
    [0, 3, 4, 8, 124, 99]
)]
fn nth_progressed(vec: SplitVec<usize, impl Growth>, n: usize, nth: usize) {
    let vec = init_vec(vec, n);

    let mut iter = vec.iter();
    let num_used = n / 2;
    for _ in 0..num_used {
        _ = iter.next();
    }

    let original_nth = num_used + nth;
    let expected = match original_nth < n {
        true => Some(original_nth),
        false => None,
    };

    assert_eq!(iter.nth(nth).copied(), expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423]
)]
fn reduce(vec: SplitVec<usize, impl Growth>, n: usize) {
    let vec = init_vec(vec, n);

    let sum = vec.iter().copied().reduce(|x, b| x + b);
    let expected = match n {
        0 => None,
        _ => Some((0..n).sum::<usize>()),
    };

    assert_eq!(sum, expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423],
    [0, 3, 4, 8, 124, 99]
)]
fn skip(vec: SplitVec<usize, impl Growth>, n: usize, skip: usize) {
    let vec = init_vec(vec, n);
    let expected: usize = match skip < n {
        true => (0..n).skip(skip).sum(),
        false => 0,
    };
    assert_eq!(vec.iter().skip(skip).sum::<usize>(), expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 423],
    [0, 3, 4, 8, 124, 99]
)]
fn skip_progressed(vec: SplitVec<usize, impl Growth>, n: usize, skip: usize) {
    let vec = init_vec(vec, n);
    let mut iter = vec.iter();

    let num_used = n / 2;
    for _ in 0..num_used {
        _ = iter.next();
    }

    let original_skip = num_used + skip;
    let expected: usize = match original_skip < n {
        true => (original_skip..n).sum(),
        false => 0,
    };

    assert_eq!(iter.skip(skip).sum::<usize>(), expected);
}
