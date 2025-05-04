use crate::*;
use alloc::vec::Vec;
use orx_concurrent_bag::ConcurrentBag;
use orx_concurrent_iter::{ConcurrentIter, IntoConcurrentIter};
use test_case::test_matrix;

fn new_vec<G: Growth>(mut vec: SplitVec<usize, G>, n: usize) -> SplitVec<usize, G> {
    for i in 0..n {
        vec.push(i);
    }
    vec
}

#[test_matrix([SplitVec::with_doubling_growth_and_fragments_capacity(16), SplitVec::with_linear_growth_and_fragments_capacity(10, 33)])]
fn split_vec_ref_into_concurrent_iter<G: Growth>(mut vec: SplitVec<usize, G>) {
    let (nt, n) = (2, 177);
    vec = new_vec(vec, n);
    let rf = &vec;
    let iter = rf.into_con_iter();

    let bag = ConcurrentBag::new();
    let num_spawned = ConcurrentBag::new();
    std::thread::scope(|s| {
        for _ in 0..nt {
            s.spawn(|| {
                num_spawned.push(true);
                while num_spawned.len() < nt {} // allow all threads to be spawned

                while let Some(x) = iter.next() {
                    bag.push(x);
                }
            });
        }
    });

    let mut expected: Vec<_> = (0..n).map(|i| &vec[i]).collect();
    expected.sort();
    let mut collected = bag.into_inner().to_vec();
    collected.sort();

    assert_eq!(expected, collected);
}
