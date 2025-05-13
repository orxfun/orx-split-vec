use crate::{Growth, SplitVec};
use core::marker::PhantomData;
use orx_concurrent_iter::implementations::VecIntoSeqIter;
use orx_self_or::SoR;

pub struct SlicesIntoSeqIter<S, T, G>
where
    T: Send + Sync,
    G: Growth,
    S: SoR<SplitVec<T, G>>,
{
    split_vec: S,
    current: VecIntoSeqIter<T>,
    phantom: PhantomData<G>,
}
