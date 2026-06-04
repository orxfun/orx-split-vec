use core::{cmp::min, mem::MaybeUninit, ptr::copy_nonoverlapping};

use super::constants::CUMULATIVE_CAPACITIES;
use crate::{Doubling, Fragment, SplitVec, growth::doubling::constants::CAPACITIES};
use alloc::vec::Vec;

impl<T> From<Vec<T>> for SplitVec<T, Doubling> {
    /// Converts a `Vec` into a `SplitVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec: SplitVec<_, Doubling> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert!(vec_capacity <= split_vec.capacity());
    /// ```
    fn from(mut value: Vec<T>) -> Self {
        let len = value.len();
        // Number of fragments to create
        let f = CUMULATIVE_CAPACITIES
            .iter()
            .enumerate()
            .find(|(_, cum_cap)| **cum_cap >= len)
            .map(|(f, _)| f)
            .expect("overflow");

        let mut fragments = Vec::with_capacity(f + 1);
        let fragments_init = fragments.spare_capacity_mut();
        let mut remaining_len = len;
        let mut curr_f = f;
        while remaining_len > 0 {
            curr_f -= 1;
            let capacity = CAPACITIES[curr_f];
            // for example, if the current fragment has a capacity of 8 but there are only 5 elements to copy,
            // we want the copy length to only be 1
            let copy_len = min(remaining_len - CUMULATIVE_CAPACITIES[curr_f], capacity);
            remaining_len -= copy_len;

            // This is adapted from Vec::split_off, with the difference that it
            // reserves the full capacity first to avoid extra allocations
            let mut fragment_data = Vec::with_capacity(capacity);
            unsafe {
                value.set_len(remaining_len);
                fragment_data.set_len(copy_len);
                copy_nonoverlapping(
                    value.as_ptr().add(remaining_len),
                    fragment_data.as_mut_ptr(),
                    copy_len,
                );
            }
            let fragment = Fragment::new(fragment_data, capacity);
            fragments_init[curr_f] = MaybeUninit::new(fragment);
        }
        debug_assert_eq!(curr_f, 0);
        unsafe { fragments.set_len(f) };

        Self::from_raw_parts(len, fragments, Doubling)
    }
}
