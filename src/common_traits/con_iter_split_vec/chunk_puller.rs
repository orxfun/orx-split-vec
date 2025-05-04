use orx_concurrent_iter::ChunkPuller;

use super::con_iter::ConIterSplitVec;
use crate::Growth;

pub struct ChunkPullerSplitVec<'i, 'a, G, T>
where
    T: Send + Sync,
    G: Growth,
{
    con_iter: &'i ConIterSplitVec<'a, T, G>,
    chunk_size: usize,
}

impl<'i, 'a, G, T> ChunkPullerSplitVec<'i, 'a, G, T>
where
    T: Send + Sync,
    G: Growth,
{
    pub(super) fn new(con_iter: &'i ConIterSplitVec<'a, T, G>, chunk_size: usize) -> Self {
        Self {
            con_iter,
            chunk_size,
        }
    }
}

impl<'i, 'a, G, T> ChunkPuller for ChunkPullerSplitVec<'i, 'a, G, T>
where
    T: Send + Sync,
    G: Growth,
{
    type ChunkItem = &'a T;

    type Chunk<'c>
        = core::iter::Empty<&'a T>
    where
        Self: 'c;

    fn chunk_size(&self) -> usize {
        todo!()
    }

    fn pull(&mut self) -> Option<Self::Chunk<'_>> {
        todo!()
    }

    fn pull_with_idx(&mut self) -> Option<(usize, Self::Chunk<'_>)> {
        todo!()
    }
}
