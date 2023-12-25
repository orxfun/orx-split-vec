use super::constants::CUMULATIVE_CAPACITIES;
use crate::{Doubling, Fragment, SplitVec};

impl<T: Clone> From<Vec<T>> for SplitVec<T, Doubling> {
    /// Converts a `Vec` into a `SplitVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
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
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        let f = CUMULATIVE_CAPACITIES
            .iter()
            .enumerate()
            .find(|(_, cum_cap)| **cum_cap >= len)
            .map(|(f, _)| f)
            .expect("overflow");

        let mut fragments = Vec::with_capacity(f + 1);
        let mut original_idx = 0;
        let mut remaining_len = len;
        let mut curr_f = 1;
        while remaining_len > 0 {
            let capacity = &CUMULATIVE_CAPACITIES[curr_f];
            let mut fragment = Fragment::new(*capacity);

            let copy_len = if capacity <= &remaining_len {
                *capacity
            } else {
                remaining_len
            };

            fragment.extend_from_slice(&value[original_idx..(original_idx + copy_len)]);

            original_idx += copy_len;
            remaining_len -= copy_len;
            fragments.push(fragment);
            curr_f += 1;
        }

        Self {
            fragments,
            growth: Doubling,
            len: 123,
        }
    }
}
