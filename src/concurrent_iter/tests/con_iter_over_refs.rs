use crate::{concurrent_iter::ConIterSplitVecRef, *};
use alloc::string::{String, ToString};
use orx_concurrent_iter::{ChunkPuller, ConcurrentIter};
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

#[test]
fn enumeration() {
    let vec: SplitVec<_> = (0..6).collect();
    let iter = ConIterSplitVecRef::new(&vec);

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next_with_idx(), Some((1, &1)));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next_with_idx(), Some((3, &3)));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next_with_idx(), Some((5, &5)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_with_idx(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_with_idx(), None);
}

#[test_matrix([
    SplitVec::with_doubling_growth_and_fragments_capacity(16),
    SplitVec::with_linear_growth_and_fragments_capacity(10, 33)
])]
fn size_hint<G: Growth>(mut vec: SplitVec<String, G>) {
    let mut n = 25;
    vec = new_vec(vec, n, |x| (x + 10).to_string());
    let iter = ConIterSplitVecRef::new(&vec);

    for _ in 0..10 {
        assert_eq!(iter.size_hint(), (n, Some(n)));
        let _ = iter.next();
        n -= 1;
    }

    let mut chunks_iter = iter.chunk_puller(7);

    assert_eq!(iter.size_hint(), (n, Some(n)));
    // assert_eq!(iter.len(), n);
    let _ = chunks_iter.pull();
    n -= 7;

    assert_eq!(iter.size_hint(), (n, Some(n)));
    // assert_eq!(iter.len(), n);
    let _ = chunks_iter.pull();
    assert_eq!(iter.size_hint(), (1, Some(1)));

    let _ = chunks_iter.pull();
    // assert_eq!(iter.len(), 0);
    assert_eq!(iter.size_hint(), (0, Some(0)));

    let _ = chunks_iter.pull();
    // assert_eq!(iter.len(), 0);
    assert_eq!(iter.size_hint(), (0, Some(0)));

    let _ = iter.next();
    // assert_eq!(iter.len(), 0);
    assert_eq!(iter.size_hint(), (0, Some(0)));
}
