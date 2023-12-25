use super::constants::FIXED_CAPACITIES;
use crate::{Linear, SplitVec};

// into SplitVec
impl<T> From<Vec<T>> for SplitVec<T, Linear> {
    /// Converts a `Vec` into a `SplitVec` by
    /// moving the vector into the split vector as the first fragment,
    /// without copying the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec: SplitVec<_, Linear> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert!(vec_capacity <= split_vec.capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        let f = FIXED_CAPACITIES
            .iter()
            .enumerate()
            .find(|(_, fixed_cap)| **fixed_cap > len)
            .map(|(f, _)| f)
            .expect("overflow");
        Self {
            fragments: vec![value.into()],
            growth: Linear::new(f),
            len,
        }
    }
}
