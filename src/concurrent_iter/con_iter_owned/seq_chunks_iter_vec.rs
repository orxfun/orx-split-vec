use super::vec_into_seq_iter::SplitVecIntoSeqIter;
use crate::{Growth, SplitVec};
use core::marker::PhantomData;

pub struct SeqChunksIterSplitVec<'i, T>
where
    T: Send + Sync,
{
    iter: SplitVecIntoSeqIter<T>,
    phantom: PhantomData<&'i ()>,
}

impl<T> SeqChunksIterSplitVec<'_, T>
where
    T: Send + Sync,
{
    pub(super) fn new<G: Growth>(vec: &SplitVec<T, G>, begin: usize, end: usize) -> Self {
        todo!()
    }
}
