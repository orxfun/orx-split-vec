use crate::{
    common_traits::iterator::{iter_of_slices::IterOfSlices, SliceBorrowAsMut},
    range_helpers::{range_end, range_start},
    Growth, SplitVec,
};
use alloc::vec::Vec;
use core::cmp::min;
use core::ops::Range;
use orx_pinned_vec::*;
use test_case::test_matrix;

fn init_vec<G: Growth>(mut vec: SplitVec<usize, G>, n: usize) -> SplitVec<usize, G> {
    vec.clear();
    vec.extend(0..n);
    vec
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2), SplitVec::with_recursive_growth()],
    [0, 1, 3, 4, 5, 8, 27, 185, 446],
    [0..0, 0..1, 4..4, 4..5, 1..4, 2..6, 2..15, 4..11, 4..12, 4..13, 4..59, 5..9, 5..12, 7..28, 7..60, 4..28, 4..60, 0..28, 0..60]
)]
fn slices_iter_mut(vec: SplitVec<usize, impl Growth>, n: usize, range: Range<usize>) {
    let mut vec = init_vec(vec, n);

    let a = min(range_start(&range), n);
    let b = min(range_end(&range, n), n);
    let expected: Vec<_> = (a..b).collect();

    let slices = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, a..b);
    for x in slices {
        for x in x {
            *x += 25;
        }
    }
    let slices = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, a..b);
    for x in slices {
        for x in x {
            *x -= 25;
        }
    }

    let slices = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, a..b);
    let values: Vec<_> = slices.flat_map(|x| x.iter()).copied().collect();

    assert_eq!(values, expected);
}

#[test]
fn slices_iter_mut_empty() {
    let mut iter = IterOfSlices::<u32, SliceBorrowAsMut>::default();
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn slices_iter_mut_fused() {
    let mut vec = init_vec(SplitVec::new(), 100);
    let mut slices = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, 3..87);
    let mut expected = 3;

    while let Some(slice) = slices.next() {
        for x in slice {
            assert_eq!(*x, expected);
            expected += 1;
        }
    }

    assert!(slices.next().is_none());
    assert!(slices.next().is_none());
}

#[test]
fn slices_iter_mut_exact_sized() {
    let mut vec = init_vec(SplitVec::new(), 20);

    let mut empty = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, 20..87);
    assert_eq!(empty.len(), 0);
    _ = empty.next();
    _ = empty.next();
    assert_eq!(empty.len(), 0);

    let mut single = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, 5..11);
    assert_eq!(single.len(), 1);
    _ = single.next();
    assert_eq!(single.len(), 0);
    _ = single.next();
    assert_eq!(single.len(), 0);

    let mut two = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, 5..13);
    assert_eq!(two.len(), 2);
    _ = two.next();
    assert_eq!(two.len(), 1);
    _ = two.next();
    assert_eq!(two.len(), 0);
    _ = two.next();
    assert_eq!(two.len(), 0);

    let mut three = IterOfSlices::<_, SliceBorrowAsMut>::new(&mut vec, 3..13);
    assert_eq!(three.len(), 3);
    _ = three.next();
    assert_eq!(three.len(), 2);
    _ = three.next();
    assert_eq!(three.len(), 1);
    _ = three.next();
    assert_eq!(three.len(), 0);
    _ = three.next();
    assert_eq!(three.len(), 0);
}
