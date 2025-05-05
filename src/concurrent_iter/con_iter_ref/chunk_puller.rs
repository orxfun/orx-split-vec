use super::con_iter::ConIterSplitVecRef;
use crate::{
    Growth,
    common_traits::iterator::{FlattenedIterOfSlices, SliceBorrowAsRef},
};
use orx_concurrent_iter::ChunkPuller;

pub struct ChunkPullerSplitVecRef<'i, 'a, G, T>
where
    T: Send + Sync,
    G: Growth,
{
    con_iter: &'i ConIterSplitVecRef<'a, T, G>,
    chunk_size: usize,
}

impl<'i, 'a, G, T> ChunkPullerSplitVecRef<'i, 'a, G, T>
where
    T: Send + Sync,
    G: Growth,
{
    pub(super) fn new(con_iter: &'i ConIterSplitVecRef<'a, T, G>, chunk_size: usize) -> Self {
        Self {
            con_iter,
            chunk_size,
        }
    }
}

impl<'a, G, T> ChunkPuller for ChunkPullerSplitVecRef<'_, 'a, G, T>
where
    T: Send + Sync,
    G: Growth,
{
    type ChunkItem = &'a T;

    type Chunk<'c>
        = FlattenedIterOfSlices<'a, T, SliceBorrowAsRef>
    where
        Self: 'c;

    fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    fn pull(&mut self) -> Option<Self::Chunk<'_>> {
        self.con_iter
            .progress_and_get_iter(self.chunk_size)
            .map(|(_, iter)| iter)
    }

    fn pull_with_idx(&mut self) -> Option<(usize, Self::Chunk<'_>)> {
        self.con_iter.progress_and_get_iter(self.chunk_size)
    }
}
