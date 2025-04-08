use crate::{
    Growth, SplitVec,
    common_traits::iterator::{
        SliceBorrowAsMut, SliceBorrowAsRef, flattened_iter_of_slices::FlattenedIterOfSlices,
    },
    range_helpers::{range_end, range_start},
};
use alloc::vec::Vec;
use core::cmp::min;
use core::ops::Range;
use orx_pinned_vec::*;
use test_case::test_matrix;

#[cfg(not(miri))]
const N: [usize; 9] = [0, 1, 3, 4, 5, 8, 27, 185, 446];
#[cfg(miri)]
const N: [usize; 9] = [0, 1, 3, 4, 5, 8, 17, 28, 35];

fn init_vec<G: Growth>(mut vec: SplitVec<usize, G>, n: usize) -> SplitVec<usize, G> {
    vec.clear();
    vec.extend(0..n);
    vec
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0..0, 0..1, 4..4, 4..5, 1..4, 2..6, 2..15, 4..11, 4..12, 4..13, 4..59, 5..9, 5..12, 7..28, 7..60, 4..28, 4..60, 0..28, 0..60]
)]
fn flattened_slices_iter(vec: SplitVec<usize, impl Growth>, range: Range<usize>) {
    for n in N {
        let vec = init_vec(vec.clone(), n);

        let a = min(range_start(&range), n);
        let b = min(range_end(&range, n), n);
        let expected: Vec<_> = (a..b).collect();

        let iter = FlattenedIterOfSlices::<_, SliceBorrowAsRef>::new(&vec, a..b);
        let values: Vec<_> = iter.copied().collect();

        assert_eq!(values, expected);
    }
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0..0, 0..1, 4..4, 4..5, 1..4, 2..6, 2..15, 4..11, 4..12, 4..13, 4..59, 5..9, 5..12, 7..28, 7..60, 4..28, 4..60, 0..28, 0..60]
)]
fn flattened_slices_iter_mut(vec: SplitVec<usize, impl Growth>, range: Range<usize>) {
    for n in N {
        let mut vec = init_vec(vec.clone(), n);

        let a = min(range_start(&range), n);
        let b = min(range_end(&range, n), n);
        let expected: Vec<_> = (a..b).collect();

        let iter = FlattenedIterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, a..b);
        for x in iter {
            *x += 20;
        }
        let iter = FlattenedIterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, a..b);
        for x in iter {
            *x -= 20;
        }

        let iter = FlattenedIterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, a..b);
        let values: Vec<_> = iter.map(|x| *x).collect();

        assert_eq!(values, expected);
    }
}

#[test]
fn flattened_slices_iter_empty() {
    let mut iter = FlattenedIterOfSlices::<u32, SliceBorrowAsRef>::default();
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn flattened_slices_iter_fused() {
    let vec = init_vec(SplitVec::new(), 100);
    let mut iter = FlattenedIterOfSlices::<_, SliceBorrowAsRef>::new(&vec, 3..87);
    let mut expected = 3;

    while let Some(x) = iter.next() {
        assert_eq!(*x, expected);
        expected += 1;
    }

    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
}

#[test]
fn flattened_slices_iter_exact_sized() {
    let vec = init_vec(SplitVec::new(), 20);

    let mut empty = FlattenedIterOfSlices::<_, SliceBorrowAsRef>::new(&vec, 20..87);
    assert_eq!(empty.len(), 0);
    _ = empty.next();
    _ = empty.next();
    assert_eq!(empty.len(), 0);

    let mut single = FlattenedIterOfSlices::<_, SliceBorrowAsRef>::new(&vec, 5..11);
    assert_eq!(single.len(), 6);
    for i in (0..6).rev() {
        _ = single.next();
        assert_eq!(single.len(), i);
    }
    assert_eq!(single.len(), 0);
    _ = single.next();
    assert_eq!(single.len(), 0);

    let mut two = FlattenedIterOfSlices::<_, SliceBorrowAsRef>::new(&vec, 5..13);
    assert_eq!(two.len(), 8);
    for i in (0..8).rev() {
        _ = two.next();
        assert_eq!(two.len(), i);
    }
    assert_eq!(two.len(), 0);
    _ = two.next();
    assert_eq!(two.len(), 0);

    let mut three = FlattenedIterOfSlices::<_, SliceBorrowAsRef>::new(&vec, 3..13);
    assert_eq!(three.len(), 10);
    for i in (0..10).rev() {
        _ = three.next();
        assert_eq!(three.len(), i);
    }
    assert_eq!(three.len(), 0);
    _ = three.next();
    assert_eq!(three.len(), 0);
}
