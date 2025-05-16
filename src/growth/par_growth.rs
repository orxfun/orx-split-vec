use crate::{Doubling, Growth, GrowthWithConstantTimeAccess, Linear, Recursive};
use orx_concurrent_iter::implementations::jagged_arrays::{
    AsRawSlice, JaggedIndex, JaggedIndexer, Slices,
};

/// A [`Growth`] that supports parallelization.
///
/// All types implementing both [`Growth`] and [`JaggedIndexer`] implement [`ParGrowth`].
///
/// [`Doubling`], [`Linear`] and [`Recursive`] growth strategies all support parallel growth.
///
/// [`Doubling`]: crate::Doubling
/// [`Linear`]: crate::Linear
/// [`Recursive`]: crate::Recursive
pub trait ParGrowth: Growth + JaggedIndexer {}

impl<G: Growth + JaggedIndexer> ParGrowth for G {}

impl JaggedIndexer for Doubling {
    unsafe fn jagged_index_unchecked<'a, T: 'a>(
        &self,
        _: &impl Slices<'a, T>,
        flat_index: usize,
    ) -> JaggedIndex {
        self.get_fragment_and_inner_indices_unchecked(flat_index)
            .into()
    }

    unsafe fn jagged_index_unchecked_from_slice<'a, T: 'a>(
        &self,
        _: &[impl AsRawSlice<T>],
        flat_index: usize,
    ) -> JaggedIndex {
        self.get_fragment_and_inner_indices_unchecked(flat_index)
            .into()
    }
}

impl JaggedIndexer for Linear {
    unsafe fn jagged_index_unchecked<'a, T: 'a>(
        &self,
        _: &impl Slices<'a, T>,
        flat_index: usize,
    ) -> JaggedIndex {
        self.get_fragment_and_inner_indices_unchecked(flat_index)
            .into()
    }

    unsafe fn jagged_index_unchecked_from_slice<'a, T: 'a>(
        &self,
        _: &[impl AsRawSlice<T>],
        flat_index: usize,
    ) -> JaggedIndex {
        self.get_fragment_and_inner_indices_unchecked(flat_index)
            .into()
    }
}

impl JaggedIndexer for Recursive {
    unsafe fn jagged_index_unchecked<'a, T: 'a>(
        &self,
        arrays: &impl Slices<'a, T>,
        flat_index: usize,
    ) -> JaggedIndex {
        let mut idx = flat_index;
        let [mut f, mut i] = [0, 0];
        let mut current_f = 0;
        while idx > 0 {
            let current_len = unsafe { arrays.slice_at_unchecked(current_f) }.len();
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

    unsafe fn jagged_index_unchecked_from_slice<'a, T: 'a>(
        &self,
        arrays: &[impl AsRawSlice<T>],
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
