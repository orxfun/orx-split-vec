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
