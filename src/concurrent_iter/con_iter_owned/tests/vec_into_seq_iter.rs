use core::mem::ManuallyDrop;
use std::dbg;

use crate::concurrent_iter::con_iter_owned::vec_into_seq_iter::SplitVecIntoSeqIter;
use crate::*;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use orx_concurrent_iter::implementations::VecIntoSeqIter;
use test_case::test_matrix;

#[cfg(miri)]
const N: usize = 125;
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

// [SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)],
#[test_matrix(
    [SplitVec::with_doubling_growth()],
    [0, 1, N]
)]
fn vec_into_seq_iter_into_all_use_all<G: Growth>(mut vec: SplitVec<String, G>, n: usize) {
    vec = new_vec(vec, n, |x| (x + 10).to_string());
    let expected = vec.clone().to_vec();

    let (_len, fragments, _growth) = (vec.len, vec.fragments, vec.growth);
    let iters = fragments.into_iter().filter_map(|f| {
        match f.len() {
            0 => None,
            _ => {
                let first = f.as_ptr();
                let last = unsafe { f.as_ptr().add(f.len() - 1) };
                let current = first;
                let drop_capacity = None; // Some(f.capacity());
                let _ = ManuallyDrop::new(f);
                Some(unsafe { VecIntoSeqIter::new(false, first, last, current, drop_capacity) })
            }
        }
    });

    let iter = SplitVecIntoSeqIter::new(iters);
    let collected: Vec<_> = iter.collect();

    assert_eq!(collected, expected);
}
