use crate::concurrent_iter::con_iter_owned::vec_into_seq_iter::SplitVecIntoSeqIter;
use crate::*;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem::ManuallyDrop;
use orx_concurrent_iter::implementations::VecIntoSeqIter;
use test_case::test_matrix;

#[cfg(miri)]
const N: usize = 41;
#[cfg(not(miri))]
const N: usize = 4735;

fn new_vec<G: Growth>(
    mut vec: SplitVec<String, G>,
    n: usize,
    elem: impl Fn(usize) -> String,
) -> SplitVec<String, G> {
    for i in 0..n {
        vec.push(elem(i));
    }
    vec
}

fn fragments_to_iters<T: Send + Sync>(
    fragments: Vec<Fragment<T>>,
) -> impl Iterator<Item = VecIntoSeqIter<T>> {
    fragments.into_iter().filter_map(|f| match f.len() {
        0 => None,
        _ => {
            let first = f.as_ptr();
            let last = unsafe { f.as_ptr().add(f.len() - 1) };
            let current = first;
            let drop_capacity = Some(f.capacity());
            let _ = ManuallyDrop::new(f);
            Some(unsafe { VecIntoSeqIter::new(false, first, last, current, drop_capacity) })
        }
    })
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2)],
    [0, 1, 4, 5, 12, N]
)]
fn vec_into_seq_iter_into_all_use_all<G: Growth>(mut vec: SplitVec<String, G>, n: usize) {
    vec = new_vec(vec, n, |x| (x + 10).to_string());
    let expected = vec.clone().to_vec();

    let (_len, fragments, _growth) = (vec.len, vec.fragments, vec.growth);
    let iters = fragments_to_iters(fragments);

    let iter = SplitVecIntoSeqIter::new(iters);
    let collected: Vec<_> = iter.collect();

    assert_eq!(collected, expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2)],
    [3, 4, 5, 15]
)]
fn vec_into_seq_iter_into_all_use_some_from_first_fragment<G: Growth>(
    mut vec: SplitVec<String, G>,
    n: usize,
) {
    vec = new_vec(vec, n, |x| (x + 10).to_string());
    let expected: Vec<_> = vec.iter().cloned().take(3).collect();

    let (_len, fragments, _growth) = (vec.len, vec.fragments, vec.growth);
    let iters = fragments_to_iters(fragments);

    let mut iter = SplitVecIntoSeqIter::new(iters);
    let mut collected = Vec::new();
    for _ in 0..3 {
        collected.push(iter.next().unwrap());
    }

    assert_eq!(collected, expected);
}
