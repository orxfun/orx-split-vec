use crate::{ParGrowth, SplitVec};
use alloc::vec::Vec;
use orx_concurrent_iter::{
    IntoConcurrentIter,
    implementations::jagged_arrays::{ConIterJaggedOwned, RawJagged, RawVec},
};

impl<T, G> IntoConcurrentIter for SplitVec<T, G>
where
    G: ParGrowth,
    T: Send + Sync,
{
    type Item = T;

    type IntoIter = ConIterJaggedOwned<T, G>;

    fn into_con_iter(self) -> Self::IntoIter {
        let arrays = self
            .fragments
            .into_iter()
            .map(|f| RawVec::from(Vec::from(f)))
            .collect();
        let jagged = RawJagged::new(arrays, self.growth, Some(self.len));
        jagged.into_con_iter()
    }
}
