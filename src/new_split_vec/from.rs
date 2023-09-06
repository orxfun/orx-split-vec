use crate::{Doubling, Exponential, Linear, SplitVec};

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
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            fragments: vec![value.into()],
            growth: Linear,
        }
    }
}
impl<T> From<Vec<T>> for SplitVec<T, Doubling> {
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
    /// let split_vec: SplitVec<_, Doubling> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            fragments: vec![value.into()],
            growth: Doubling,
        }
    }
}
impl<T> From<Vec<T>> for SplitVec<T, Exponential> {
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
    /// let split_vec: SplitVec<_, Exponential> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            fragments: vec![value.into()],
            growth: Exponential::default(),
        }
    }
}

// from SplitVec
