use crate::{Doubling, GrowthWithConstantTimeAccess, Linear};
use orx_concurrent_iter::implementations::jagged::{JaggedIndex, JaggedIndexer, RawVec};

impl JaggedIndexer for Doubling {
    #[inline(always)]
    fn jagged_index<T>(
        &self,
        total_len: usize,
        _: &[RawVec<T>],
        flat_index: usize,
    ) -> Option<JaggedIndex> {
        (flat_index <= total_len).then(|| {
            let (f, i) = self.get_fragment_and_inner_indices_unchecked(flat_index);
            JaggedIndex::new(f, i)
        })
    }

    #[inline(always)]
    unsafe fn jagged_index_unchecked<T>(&self, _: &[RawVec<T>], flat_index: usize) -> JaggedIndex {
        let (f, i) = self.get_fragment_and_inner_indices_unchecked(flat_index);
        JaggedIndex::new(f, i)
    }
}

impl JaggedIndexer for Linear {
    #[inline(always)]
    fn jagged_index<T>(
        &self,
        total_len: usize,
        _: &[RawVec<T>],
        flat_index: usize,
    ) -> Option<JaggedIndex> {
        (flat_index <= total_len).then(|| {
            let (f, i) = self.get_fragment_and_inner_indices_unchecked(flat_index);
            JaggedIndex::new(f, i)
        })
    }

    #[inline(always)]
    unsafe fn jagged_index_unchecked<T>(&self, _: &[RawVec<T>], flat_index: usize) -> JaggedIndex {
        let (f, i) = self.get_fragment_and_inner_indices_unchecked(flat_index);
        JaggedIndex::new(f, i)
    }
}
