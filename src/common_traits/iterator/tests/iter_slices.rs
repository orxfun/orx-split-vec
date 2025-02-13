use crate::{
    common_traits::iterator::iter_slices::IterSlices,
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
    [0..0, 0..1, 4..4, 4..5, 1..4, 2..6, 2..15, 4..11, 4..12, 4..13, 4..59, 7..28, 7..60, 4..28, 4..60, 0..28, 0..60]
)]
fn slices_iter(vec: SplitVec<usize, impl Growth>, n: usize, range: Range<usize>) {
    let vec = init_vec(vec, n);

    let a = min(range_start(&range), n);
    let b = min(range_end(&range, n), n);
    let expected: Vec<_> = (a..b).collect();

    let slices = IterSlices::new(&vec, a..b);
    let values: Vec<_> = slices.flat_map(|x| x.iter()).copied().collect();

    assert_eq!(values, expected);
}
