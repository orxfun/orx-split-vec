use crate::{Doubling, Growth};
use orx_concurrent_iter::implementations::jagged::{JaggedIndex, JaggedIndexer, RawVec};

impl JaggedIndexer for Doubling {
    fn jagged_index<T>(&self, arrays: &[RawVec<T>], flat_index: usize) -> Option<JaggedIndex> {
        todo!()
    }

    unsafe fn jagged_index_unchecked<T>(
        &self,
        arrays: &[RawVec<T>],
        flat_index: usize,
    ) -> JaggedIndex {
        todo!()
    }
}
