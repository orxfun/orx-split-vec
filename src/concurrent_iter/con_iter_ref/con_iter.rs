use crate::{Growth, GrowthWithConstantTimeAccess, SplitVec};
use core::sync::atomic::AtomicUsize;

/// Concurrent iterator of reference of a [`SplitVec`] yielding references to the items
/// of the split vector.
///
/// It can be created by calling [`into_con_iter`] on a reference of a split vector.
///
/// Alternatively, it can be created by calling [`con_iter`] on the split vector.
///
/// [`into_con_iter`]: orx_concurrent_iter::IntoConcurrentIter::into_con_iter
/// [`con_iter`]: orx_concurrent_iter::ConcurrentIterable::con_iter
pub struct ConIterSplitVecRef<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
    vec: &'a SplitVec<T, G>,
    counter: AtomicUsize,
}

impl<'a, T, G> ConIterSplitVecRef<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
}

trait JaggedIndexer {
    fn jagged_index(&self, flat_index: usize) -> Option<(usize, usize)>;
}

impl<G: Growth, T> JaggedIndexer for SplitVec<T, G> {
    fn jagged_index(&self, flat_index: usize) -> Option<(usize, usize)> {
        self.get_fragment_and_inner_indices(flat_index)
    }
}

impl<G: GrowthWithConstantTimeAccess> JaggedIndexer for G {
    fn jagged_index(&self, flat_index: usize) -> Option<(usize, usize)> {
        Some(self.get_fragment_and_inner_indices_unchecked(flat_index))
    }
}
