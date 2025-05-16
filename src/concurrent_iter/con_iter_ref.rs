use crate::{ParGrowth, SplitVec};
use alloc::vec::Vec;
use orx_concurrent_iter::{
    IntoConcurrentIter,
    implementations::jagged_arrays::{ConIterJaggedOwned, RawSlice},
};

impl<'a, T, G> IntoConcurrentIter for &'a SplitVec<T, G>
where
    T: Send + Sync,
    G: ParGrowth,
{
    // type Item = &'a T;

    // type IntoIter = ConIterJaggedRef<'a, T, G>;

    type Item = T;

    type IntoIter = ConIterJaggedOwned<T, G>;

    fn into_con_iter(self) -> Self::IntoIter {
        let slices: Vec<_> = self
            .fragments
            .iter()
            .map(|v| RawSlice::from(v.as_slice()))
            .collect();

        // let jagged = RawJagged::new_as_reference(slices, self.growth.clone(), Some(self.len));

        // let mut con_iter = ConIterSplitVecRef { vec: self, jagged };

        // con_iter.jagged = RawJagged::new_as_reference(slices, self.growth.clone(), Some(self.len));

        todo!()
    }
}
