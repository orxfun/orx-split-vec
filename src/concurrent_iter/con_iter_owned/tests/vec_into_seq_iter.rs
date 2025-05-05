use crate::concurrent_iter::con_iter_owned::vec_into_seq_iter::SplitVecIntoSeqIter;
use crate::fragment::RawFragment;
use crate::*;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use orx_concurrent_iter::implementations::VecIntoSeqIter;
use test_case::test_matrix;

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

unsafe fn take<T>(ptr: *mut T) -> T {
    let mut value = MaybeUninit::<T>::uninit();
    unsafe { value.as_mut_ptr().swap(ptr) };
    unsafe { value.assume_init() }
}

fn fragments_to_iters<T: Send + Sync>(
    fragments: impl Iterator<Item = RawFragment<T>> + Clone,
    total_len: usize,
    num_taken: usize,
) -> impl Iterator<Item = VecIntoSeqIter<T>> {
    let completed = num_taken == total_len;
    let mut num_taken = num_taken;

    fragments.filter_map(move |f| match (completed, f.len) {
        (_, 0) | (true, _) => {
            f.manually_drop();
            None
        }
        (false, len) => {
            match num_taken >= len {
                true => {
                    num_taken -= len;
                    f.manually_drop();
                    None
                }
                false => {
                    let first = f.ptr;
                    let last = unsafe { f.ptr.add(len - 1) };
                    let current = unsafe { f.ptr.add(num_taken) }; // first + num_taken is in bounds
                    let drop_capacity = Some(f.capacity);
                    num_taken = 0;
                    Some(unsafe { VecIntoSeqIter::new(false, first, last, current, drop_capacity) })
                }
            }
        }
    })
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2)],
    [0, 1, 4, 5, 12, 41]
)]
fn vec_into_seq_iter_into_all_use_all<G: Growth>(mut vec: SplitVec<String, G>, n: usize) {
    vec = new_vec(vec, n, |x| (x + 10).to_string());
    let expected = vec.clone().to_vec();

    let (len, fragments, _growth) = (vec.len, vec.fragments, vec.growth);
    let iters = fragments_to_iters(fragments.into_iter().map(Into::into), len, 0);

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

    let (len, fragments, _growth) = (vec.len, vec.fragments, vec.growth);
    let iters = fragments_to_iters(fragments.into_iter().map(Into::into), len, 0);

    let mut iter = SplitVecIntoSeqIter::new(iters);
    let mut collected = Vec::new();
    for _ in 0..3 {
        collected.push(iter.next().unwrap());
    }

    assert_eq!(collected, expected);
}

#[test_matrix(
    [SplitVec::with_doubling_growth(), SplitVec::with_linear_growth(2)],
    [7, 8, 9, 15]
)]
fn vec_into_seq_iter_into_all_use_some_from_second_fragment<G: Growth>(
    mut vec: SplitVec<String, G>,
    n: usize,
) {
    vec = new_vec(vec, n, |x| (x + 10).to_string());
    let expected: Vec<_> = vec.iter().cloned().take(7).collect();

    let (len, fragments, _growth) = (vec.len, vec.fragments, vec.growth);
    let iters = fragments_to_iters(fragments.into_iter().map(Into::into), len, 0);

    let mut iter = SplitVecIntoSeqIter::new(iters);
    let mut collected = Vec::new();
    for _ in 0..7 {
        collected.push(iter.next().unwrap());
    }

    assert_eq!(collected, expected);
}
