use crate::{Doubling, GrowthWithConstantTimeAccess, Linear, Recursive};
use orx_concurrent_iter::implementations::jagged_arrays::{AsSlice, JaggedIndex, JaggedIndexer};

impl JaggedIndexer for Doubling {
    fn jagged_index<T>(
        &self,
        total_len: usize,
        _: &[impl AsSlice<T>],
        flat_index: usize,
    ) -> Option<JaggedIndex> {
        (flat_index <= total_len).then(|| {
            self.get_fragment_and_inner_indices_unchecked(flat_index)
                .into()
        })
    }

    unsafe fn jagged_index_unchecked<T>(
        &self,
        _: &[impl AsSlice<T>],
        flat_index: usize,
    ) -> JaggedIndex {
        self.get_fragment_and_inner_indices_unchecked(flat_index)
            .into()
    }
}

impl JaggedIndexer for Linear {
    fn jagged_index<T>(
        &self,
        total_len: usize,
        _arrays: &[impl AsSlice<T>],
        flat_index: usize,
    ) -> Option<JaggedIndex> {
        (flat_index <= total_len).then(|| {
            self.get_fragment_and_inner_indices_unchecked(flat_index)
                .into()
        })
    }

    unsafe fn jagged_index_unchecked<T>(
        &self,
        _: &[impl AsSlice<T>],
        flat_index: usize,
    ) -> JaggedIndex {
        self.get_fragment_and_inner_indices_unchecked(flat_index)
            .into()
    }
}

impl JaggedIndexer for Recursive {
    fn jagged_index<T>(
        &self,
        total_len: usize,
        arrays: &[impl AsSlice<T>],
        flat_index: usize,
    ) -> Option<JaggedIndex> {
        (flat_index <= total_len).then(|| {
            // SAFETY: flat_index is in bounds or equal to length
            unsafe { self.jagged_index_unchecked(arrays, flat_index) }.into()
        })
    }

    unsafe fn jagged_index_unchecked<T>(
        &self,
        arrays: &[impl AsSlice<T>],
        flat_index: usize,
    ) -> JaggedIndex {
        let mut idx = flat_index;
        let [mut f, mut i] = [0, 0];
        let mut current_f = 0;
        while idx > 0 {
            let current_len = arrays[current_f].length();
            match current_len > idx {
                true => {
                    i = idx;
                    idx = 0;
                }
                false => {
                    f += 1;
                    idx -= current_len;
                }
            }
            current_f += 1;
        }
        JaggedIndex::new(f, i)
    }
}
