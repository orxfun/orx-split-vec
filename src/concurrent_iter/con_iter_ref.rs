use crate::{Growth, SplitVec};
use alloc::vec::Vec;
use orx_concurrent_iter::{
    IntoConcurrentIter,
    implementations::jagged::{ConIterJaggedRef, JaggedIndexer, RawJagged, RawVec},
};

pub struct ConIterSplitVecRef<'a, T, G>
where
    T: Send + Sync,
    G: Growth + JaggedIndexer,
{
    vec: &'a SplitVec<T, G>,
    jagged: RawJagged<T, G>,
}

impl<'a, T, G> IntoConcurrentIter for &'a SplitVec<T, G>
where
    T: Send + Sync,
    G: Growth + JaggedIndexer,
{
    type Item = &'a T;

    type IntoIter = ConIterJaggedRef<'a, T, G>;

    fn into_con_iter(self) -> Self::IntoIter {
        let slices: Vec<_> = self
            .fragments
            .iter()
            .map(|v| RawVec::from(v.as_slice()))
            .collect();

        let jagged = RawJagged::new_as_reference(slices, self.growth.clone(), Some(self.len));

        let mut con_iter = ConIterSplitVecRef { vec: self, jagged };

        // con_iter.jagged = RawJagged::new_as_reference(slices, self.growth.clone(), Some(self.len));

        todo!()
    }
}
