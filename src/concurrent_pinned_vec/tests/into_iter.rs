use crate::{
    Doubling, GrowthWithConstantTimeAccess, Linear, SplitVec,
    concurrent_pinned_vec::into_iter::ConcurrentSplitVecIntoIter,
};
use orx_pinned_vec::{IntoConcurrentPinnedVec, PinnedVec};
use std::string::{String, ToString};
use test_case::test_matrix;

fn vec_doubling() -> SplitVec<String, Doubling> {
    (0..20).map(|x| x.to_string()).collect()
}

fn vec_linear() -> SplitVec<String, Linear> {
    let mut vec = SplitVec::with_linear_growth(2);
    vec.extend((0..20).map(|x| x.to_string()));
    vec
}

#[test_matrix([vec_doubling, vec_linear])]
fn into_iter_non_taken<G, F>(vec: F)
where
    G: GrowthWithConstantTimeAccess,
    F: Fn() -> SplitVec<String, G>,
{
    let iter = || {
        let vec = vec();
        let range = 0..vec.len();
        let convec = vec.into_concurrent();
        let (growth, data, capacity) = convec.destruct();
        ConcurrentSplitVecIntoIter::new(capacity, data, growth, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 20);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}
