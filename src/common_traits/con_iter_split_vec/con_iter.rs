use crate::{
    Fragment, Growth, SplitVec,
    common_traits::iterator::{FlattenedIterOfSlices, IterOfSlices, SliceBorrowAsRef},
};
use core::{
    iter::Skip,
    sync::atomic::{AtomicUsize, Ordering},
};
use orx_concurrent_iter::ConcurrentIter;
use orx_iterable::Iterable;
use orx_pinned_vec::PinnedVec;

use super::chunk_puller::ChunkPullerSplitVec;

pub struct ConIterSplitVec<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
    vec: &'a SplitVec<T, G>,
    counter: AtomicUsize,
}

impl<'a, T, G> ConIterSplitVec<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
    pub(crate) fn new(vec: &'a SplitVec<T, G>) -> Self {
        Self {
            vec,
            counter: 0.into(),
        }
    }

    fn progress_and_get_begin_idx(&self, number_to_fetch: usize) -> Option<usize> {
        let begin_idx = self.counter.fetch_add(number_to_fetch, Ordering::Relaxed);
        match begin_idx < self.vec.len {
            true => Some(begin_idx),
            false => None,
        }
    }

    pub(super) fn progress_and_get_slice(
        &self,
        chunk_size: usize,
    ) -> Option<(usize, FlattenedIterOfSlices<'a, T, SliceBorrowAsRef>)> {
        self.progress_and_get_begin_idx(chunk_size)
            .map(|begin_idx| {
                let end_idx = (begin_idx + chunk_size).min(self.vec.len).max(begin_idx);
                let range = begin_idx..end_idx;
                let iter = FlattenedIterOfSlices::<_, SliceBorrowAsRef>::new(self.vec, range);
                (begin_idx, iter)
            })
    }
}

impl<'a, T, G> ConcurrentIter for ConIterSplitVec<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
    type Item = &'a T;

    type SequentialIter = Skip<super::super::iterator::Iter<'a, T>>;

    type ChunkPuller<'i>
        = ChunkPullerSplitVec<'i, 'a, G, T>
    where
        Self: 'i;

    fn into_seq_iter(self) -> Self::SequentialIter {
        let current = self.counter.load(Ordering::Acquire);
        self.vec.iter().skip(current)
    }

    fn skip_to_end(&self) {
        todo!()
    }

    fn next(&self) -> Option<Self::Item> {
        todo!()
    }

    fn next_with_idx(&self) -> Option<(usize, Self::Item)> {
        todo!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }

    fn chunk_puller(&self, chunk_size: usize) -> Self::ChunkPuller<'_> {
        todo!()
    }
}
