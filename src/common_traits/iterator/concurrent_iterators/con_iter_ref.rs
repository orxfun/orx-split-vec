use crate::{Growth, SplitVec};
use core::{
    iter::Skip,
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};
use orx_concurrent_iter::{
    BufferedChunk, BufferedChunkX, ConcurrentIter, ConcurrentIterX, ConcurrentIterable,
    IntoConcurrentIter, Next, NextChunk,
};
use orx_iterable::{Collection, Iterable};
use orx_pinned_vec::PinnedVec;

// concurrent iterable

impl<T, G> ConcurrentIterable for SplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    type Item<'i>
        = &'i T
    where
        Self: 'i;

    type ConIter<'i>
        = ConIterRef<'i, T, G>
    where
        Self: 'i;

    fn con_iter(&self) -> Self::ConIter<'_> {
        ConIterRef::new(self)
    }
}

impl<'i, T, G> IntoConcurrentIter for &'i SplitVec<T, G>
where
    T: Send + Sync,
    G: Growth,
{
    type Item = &'i T;

    type ConIter = ConIterRef<'i, T, G>;

    fn into_con_iter(self) -> Self::ConIter {
        ConIterRef::new(self)
    }
}

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

    fn progress_and_get_begin_idx(&self, number_to_fetch: usize) -> Option<usize> {
        let begin_idx = self.counter.fetch_add(number_to_fetch, Ordering::Relaxed);
        match begin_idx < self.vec.len() {
            true => Some(begin_idx),
            _ => None,
        }
    }

    fn progress_and_get_chunk(
        &self,
        chunk_size: usize,
    ) -> Option<(usize, impl ExactSizeIterator<Item = &'a T>)> {
        self.progress_and_get_begin_idx(chunk_size)
            .map(|begin_idx| {
                let end_idx = (begin_idx + chunk_size).min(self.vec.len()).max(begin_idx);
                (begin_idx, self.vec.iter_over(begin_idx..end_idx))
            })
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
        iter.progress_and_get_chunk(self.chunk_size).map(|x| x.1)
    }
}

impl<'a, T, G> BufferedChunk<&'a T> for ConBufferedIterRef<T, G>
where
    T: Send + Sync,
    G: Growth + 'a,
{
    fn pull(
        &mut self,
        iter: &Self::ConIter,
    ) -> Option<NextChunk<&'a T, impl ExactSizeIterator<Item = &'a T>>> {
        iter.progress_and_get_chunk(self.chunk_size)
            .map(|(begin_idx, values)| NextChunk { begin_idx, values })
    }
}

// concurrent iter

impl<'a, T: Send + Sync, G: Growth> ConcurrentIterX for ConIterRef<'a, T, G> {
    type Item = &'a T;

    type SeqIter = Skip<<<SplitVec<T, G> as Collection>::Iterable<'a> as Iterable>::Iter>;

    type BufferedIterX = ConBufferedIterRef<T, G>;

    fn into_seq_iter(self) -> Self::SeqIter {
        let current = self.counter.load(Ordering::Acquire);
        self.vec.iter().skip(current)
    }

    fn next_chunk_x(&self, chunk_size: usize) -> Option<impl ExactSizeIterator<Item = Self::Item>> {
        self.progress_and_get_chunk(chunk_size).map(|x| x.1)
    }

    #[inline(always)]
    fn next(&self) -> Option<Self::Item> {
        let idx = self.counter.fetch_add(1, Ordering::Acquire);
        self.get(idx)
    }

    fn skip_to_end(&self) {
        let _ = self.counter.fetch_max(self.vec.len(), Ordering::Acquire);
    }

    fn try_get_len(&self) -> Option<usize> {
        let current = self.counter.load(Ordering::Acquire);
        let initial_len = self.vec.len();
        Some(match current.cmp(&initial_len) {
            core::cmp::Ordering::Less => initial_len - current,
            _ => 0,
        })
    }

    fn try_get_initial_len(&self) -> Option<usize> {
        Some(self.vec.len())
    }
}

impl<'a, T: Send + Sync, G: Growth> ConcurrentIter for ConIterRef<'a, T, G> {
    type BufferedIter = Self::BufferedIterX;

    fn next_id_and_value(&self) -> Option<Next<Self::Item>> {
        let idx = self.counter.fetch_add(1, Ordering::Acquire);
        self.get(idx).map(|value| Next { idx, value })
    }

    fn next_chunk(
        &self,
        chunk_size: usize,
    ) -> Option<NextChunk<Self::Item, impl ExactSizeIterator<Item = Self::Item>>> {
        self.progress_and_get_chunk(chunk_size)
            .map(|(begin_idx, values)| NextChunk { begin_idx, values })
    }
}
