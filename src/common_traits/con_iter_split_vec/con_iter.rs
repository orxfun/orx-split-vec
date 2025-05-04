use crate::{
    Fragment, Growth, SplitVec,
    common_traits::iterator::{FlattenedIterOfSlices, IterOfSlices, SliceBorrowAsRef},
};
use core::sync::atomic::{AtomicUsize, Ordering};
use orx_pinned_vec::PinnedVec;

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
