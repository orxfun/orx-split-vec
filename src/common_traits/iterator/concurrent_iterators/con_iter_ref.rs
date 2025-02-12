use crate::{Growth, SplitVec};
use core::{
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};
use orx_concurrent_iter::{BufferedChunkX, ConcurrentIterX};
use orx_pinned_vec::PinnedVec;

// iter

pub struct ConIterRef<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
    counter: AtomicUsize,
    vec: &'a SplitVec<T, G>,
}

impl<'a, T, G> ConIterRef<'a, T, G>
where
    T: Send + Sync,
    G: Growth,
{
    pub fn new(vec: &'a SplitVec<T, G>) -> Self {
        Self {
            counter: 0.into(),
            vec,
        }
    }

    #[inline(always)]
    fn get(&self, item_idx: usize) -> Option<&'a T> {
        self.vec.get(item_idx)
    }

    #[inline(always)]
    pub(crate) fn progress_and_get_begin_idx(&self, number_to_fetch: usize) -> Option<usize> {
        let begin_idx = self.counter.fetch_add(number_to_fetch, Ordering::Relaxed);
        match begin_idx < self.vec.len() {
            true => Some(begin_idx),
            _ => None,
        }
    }
}

impl<T: Send + Sync, G: Growth> Clone for ConIterRef<'_, T, G> {
    fn clone(&self) -> Self {
        let counter = self.counter.load(Ordering::SeqCst).into();
        Self {
            counter,
            vec: self.vec,
        }
    }
}

unsafe impl<T: Send + Sync, G: Growth> Sync for ConIterRef<'_, T, G> {}

unsafe impl<T: Send + Sync, G: Growth> Send for ConIterRef<'_, T, G> {}

// buffered iter

pub struct ConBufferedIterRef<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    chunk_size: usize,
    phantom: PhantomData<(T, G)>,
}

impl<'a, T, G> BufferedChunkX<&'a T> for ConBufferedIterRef<T, G>
where
    T: Send + Sync,
    G: Growth + 'a,
{
    type ConIter = ConIterRef<'a, T, G>;

    fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    fn pull_x(&mut self, iter: &Self::ConIter) -> Option<impl ExactSizeIterator<Item = &'a T>> {
        iter.progress_and_get_begin_idx(self.chunk_size)
            .map(|begin_idx| {
                let vec = iter.vec;
                let end_idx = (begin_idx + self.chunk_size).min(vec.len()).max(begin_idx);
                let a = vec.slices(begin_idx..end_idx);
                let b = a.into_iter();
                let c = b.flat_map(|x| x.iter());
                // TODO: return an iterator of slices
                core::iter::empty()
            })
    }
}

impl<'a, T: Send + Sync, G: Growth> ConcurrentIterX for ConIterRef<'a, T, G> {
    type Item = &'a T;

    type SeqIter = core::iter::Empty<&'a T>;

    type BufferedIterX = ConBufferedIterRef<T, G>;

    fn into_seq_iter(self) -> Self::SeqIter {
        let current = self.counter.load(Ordering::Acquire);
        todo!()
    }

    fn next_chunk_x(&self, chunk_size: usize) -> Option<impl ExactSizeIterator<Item = Self::Item>> {
        Some(core::iter::empty())
    }

    fn next(&self) -> Option<Self::Item> {
        todo!()
    }

    fn skip_to_end(&self) {
        todo!()
    }

    fn try_get_len(&self) -> Option<usize> {
        todo!()
    }

    fn try_get_initial_len(&self) -> Option<usize> {
        todo!()
    }
}
