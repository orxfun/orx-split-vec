use super::chunk_puller::ChunkPullerSplitVecRef;
use crate::{
    Growth, SplitVec,
    common_traits::iterator::{FlattenedIterOfSlices, SliceBorrowAsRef},
};
use core::{
    iter::Skip,
    sync::atomic::{AtomicUsize, Ordering},
};
use orx_concurrent_iter::ConcurrentIter;
use orx_iterable::Iterable;

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

    pub(super) fn progress_and_get_iter(
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

impl<'a, T, G> ConcurrentIter for ConIterSplitVecRef<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
    type Item = &'a T;

    type SequentialIter = Skip<super::super::iterator::Iter<'a, T>>;

    type ChunkPuller<'i>
        = ChunkPullerSplitVecRef<'i, 'a, G, T>
    where
        Self: 'i;

    fn into_seq_iter(self) -> Self::SequentialIter {
        let current = self.counter.load(Ordering::Acquire);
        self.vec.iter().skip(current)
    }

    fn skip_to_end(&self) {
        let _ = self.counter.fetch_max(self.vec.len, Ordering::Acquire);
    }

    fn next(&self) -> Option<Self::Item> {
        self.progress_and_get_begin_idx(1).map(|idx| &self.vec[idx])
    }

    fn next_with_idx(&self) -> Option<(usize, Self::Item)> {
        self.progress_and_get_begin_idx(1)
            .map(|idx| (idx, &self.vec[idx]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num_taken = self.counter.load(Ordering::Acquire);
        let remaining = self.vec.len.saturating_sub(num_taken);
        (remaining, Some(remaining))
    }

    fn chunk_puller(&self, chunk_size: usize) -> Self::ChunkPuller<'_> {
        Self::ChunkPuller::new(self, chunk_size)
    }
}
