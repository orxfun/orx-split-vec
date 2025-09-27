use crate::{Doubling, SplitVec};
use orx_pinned_vec::{IntoConcurrentPinnedVec, PinnedVec};
use std::string::ToString;

#[test]
fn into_iter_non_taken() {
    let iter = || {
        let vec: SplitVec<_, Doubling> = (0..20).map(|x| x.to_string()).collect();
        let range = 0..vec.len();
        let convec = vec.into_concurrent();
        let (growth, data, capacity) = convec.destruct();

        ConcurrentFixedVecIntoIter::new(data, range)
    };

    let consume_all = iter().count();
    assert_eq!(consume_all, 20);

    let mut consume_half = iter();
    for _ in 0..10 {
        _ = consume_half.next();
    }

    let _consume_none = iter();
}
