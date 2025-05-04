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
