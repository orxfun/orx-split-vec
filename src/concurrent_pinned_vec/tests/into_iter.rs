use crate::{
    Doubling, GrowthWithConstantTimeAccess, Linear, SplitVec,
    concurrent_pinned_vec::into_iter::ConcurrentSplitVecIntoIter,
};
use orx_pinned_vec::{ConcurrentPinnedVec, IntoConcurrentPinnedVec, PinnedVec};
use std::string::{String, ToString};
use test_case::test_matrix;

fn vec_doubling(n: usize) -> SplitVec<String, Doubling> {
    (0..n).map(|x| x.to_string()).collect()
}

fn vec_linear(n: usize) -> SplitVec<String, Linear> {
    let mut vec = SplitVec::with_linear_growth(2);
    vec.extend((0..n).map(|x| x.to_string()));
    vec
}

#[test_matrix([vec_doubling, vec_linear])]
fn into_iter_empty<G, F>(vec: F)
where
    G: GrowthWithConstantTimeAccess,
    F: Fn(usize) -> SplitVec<String, G>,
{
    let iter = || {
        let vec = vec(0);
        let range = 0..vec.len();
        let convec = vec.into_concurrent();
        let (growth, data, capacity) = convec.destruct();
        ConcurrentSplitVecIntoIter::new(capacity, data, growth, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 0);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}

#[test_matrix([vec_doubling, vec_linear])]
fn into_iter_non_taken<G, F>(vec: F)
where
    G: GrowthWithConstantTimeAccess,
    F: Fn(usize) -> SplitVec<String, G>,
{
    let iter = || {
        let vec = vec(20);
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

#[test_matrix([vec_doubling, vec_linear])]
fn into_iter_taken_from_beg<G, F>(vec: F)
where
    G: GrowthWithConstantTimeAccess,
    F: Fn(usize) -> SplitVec<String, G>,
{
    let iter = || {
        let vec = vec(20);
        let range = 5..vec.len();
        let convec = vec.into_concurrent();

        for i in 0..range.start {
            let p = unsafe { convec.get_ptr_mut(i) };
            let _value = unsafe { p.read() };
        }

        let (growth, data, capacity) = convec.destruct();
        ConcurrentSplitVecIntoIter::new(capacity, data, growth, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 15);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}

#[test_matrix([vec_doubling, vec_linear])]
fn into_iter_taken_from_end<G, F>(vec: F)
where
    G: GrowthWithConstantTimeAccess,
    F: Fn(usize) -> SplitVec<String, G>,
{
    let iter = || {
        let vec = vec(20);
        let vec_len = vec.len();
        let range = 0..15;
        let convec = vec.into_concurrent();

        for i in range.end..vec_len {
            let p = unsafe { convec.get_ptr_mut(i) };
            let _value = unsafe { p.read() };
        }

        let (growth, data, capacity) = convec.destruct();
        ConcurrentSplitVecIntoIter::new(capacity, data, growth, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 15);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}

#[test_matrix([vec_doubling, vec_linear])]
fn into_iter_taken_from_both_ends<G, F>(vec: F)
where
    G: GrowthWithConstantTimeAccess,
    F: Fn(usize) -> SplitVec<String, G>,
{
    let iter = || {
        let vec = vec(20);
        let vec_len = vec.len();
        let range = 4..15;
        let convec = vec.into_concurrent();

        for i in (0..range.start).chain(range.end..vec_len) {
            let p = unsafe { convec.get_ptr_mut(i) };
            let _value = unsafe { p.read() };
        }

        let (growth, data, capacity) = convec.destruct();
        ConcurrentSplitVecIntoIter::new(capacity, data, growth, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 11);

    let mut consume_half = iter();
    for _ in 0..7 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}
