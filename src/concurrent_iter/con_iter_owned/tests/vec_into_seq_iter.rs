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
    [
        (0, 0, 0),
        (3, 0, 0),
        (4, 0, 2),
        (5, 0, 4),
        (12, 0, 12),
        (13, 0, 7),
        (14, 0, 9),
        (14, 0, 14),
        (3, 1, 0),
        (3, 1, 1),
        (3, 1, 2),
        (3, 3, 0),
        (15, 10, 0),
        (15, 10, 3),
        (15, 10, 5),
        (15, 15, 0),
    ]
)]
fn split_vec_into_seq_iter<G>(
    mut vec: SplitVec<String, G>,
    (n, n_pre_take, n_post_take): (usize, usize, usize),
) where
    G: Growth,
{
    vec = new_vec(vec, n, |x| (x + 10).to_string());
    let expected_pre: Vec<_> = vec.iter().cloned().take(n_pre_take).collect();
    let expected_post: Vec<_> = vec
        .iter()
        .cloned()
        .skip(n_pre_take)
        .take(n_post_take)
        .collect();

    let (len, mut fragments, growth) = (vec.len, vec.fragments, vec.growth);

    let mut pre_take = Vec::new();
    for p in 0..n_pre_take {
        let (f, i) = growth
            .get_fragment_and_inner_indices(len, &fragments, p)
            .unwrap();
        let p = unsafe { fragments[f].as_mut_ptr().add(i) };
        let taken = unsafe { take(p) };
        pre_take.push(taken);
    }
    assert_eq!(pre_take, expected_pre);

    let iters = fragments_to_iters(fragments.into_iter().map(Into::into), len, n_pre_take);
    let iter = SplitVecIntoSeqIter::new(iters);
    let post_take: Vec<_> = iter.take(n_post_take).collect();
    assert_eq!(post_take, expected_post);
}
