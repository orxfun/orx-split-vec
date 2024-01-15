use crate::{Doubling, Recursive, SplitVec};

impl<T> From<SplitVec<T, Doubling>> for SplitVec<T, Recursive> {
    /// Converts a `SplitVec<T, Doubling>` into a `SplitVec<T, Recursive>` with no cost.
    ///
    /// * The benefit of `Doubling` growth strategy is its constant random access time.
    /// * On the other hand, the benefit of `Recursive` growth strategy is the constant time `expand` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec_doubling: SplitVec<_, Doubling> = vec.into();
    /// assert_eq!(split_vec_doubling, &['a', 'b', 'c']);
    ///
    /// let split_vec_recursive: SplitVec<_, Recursive> = split_vec_doubling.into();
    /// assert_eq!(split_vec_recursive, &['a', 'b', 'c']);
    /// ```
    fn from(value: SplitVec<T, Doubling>) -> Self {
        Self {
            len: value.len,
            fragments: value.fragments,
            growth: Recursive,
        }
    }
}

impl<T: Clone> From<Vec<T>> for SplitVec<T, Recursive> {
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
    /// let split_vec: SplitVec<_, Recursive> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert!(vec_capacity <= split_vec.capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        let split_doubling: SplitVec<_, Doubling> = value.into();
        split_doubling.into()
    }
}
