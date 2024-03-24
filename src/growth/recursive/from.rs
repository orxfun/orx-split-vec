use crate::{Doubling, Linear, Recursive, SplitVec};

impl<T> From<SplitVec<T, Doubling>> for SplitVec<T, Recursive> {
    /// Converts a `SplitVec<T, Doubling>` into a `SplitVec<T, Recursive>` with no cost.
    ///
    /// * The benefit of `Doubling` growth strategy is its constant random access time.
    /// * On the other hand, the benefit of `Recursive` growth strategy is the constant time `expand` operation.
    ///
    /// Note that this is a one-way conversion:
    /// * it is possible to convert any split vec `SplitVec<T, Doubling>` into `SplitVec<T, Recursive>`;
    /// * however, not the other way around, since constant random access time requirements of `Doubling` are not satisfied.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut split_vec_doubling = SplitVec::with_doubling_growth();
    /// split_vec_doubling.extend_from_slice(&['a', 'b', 'c']);
    /// assert_eq!(split_vec_doubling, &['a', 'b', 'c']);
    ///
    /// let split_vec_recursive: SplitVec<_, Recursive> = split_vec_doubling.into();
    /// assert_eq!(split_vec_recursive, &['a', 'b', 'c']);
    /// ```
    fn from(value: SplitVec<T, Doubling>) -> Self {
        Self::from_raw_parts(value.len, value.fragments, Recursive)
    }
}

impl<T> From<SplitVec<T, Linear>> for SplitVec<T, Recursive> {
    /// Converts a `SplitVec<T, Doubling>` into a `SplitVec<T, Recursive>` with no cost.
    ///
    /// * The benefit of `Doubling` growth strategy is its constant random access time.
    /// * On the other hand, the benefit of `Recursive` growth strategy is the constant time `expand` operation.
    ///
    /// Note that this is a one-way conversion:
    /// * it is possible to convert any split vec `SplitVec<T, Doubling>` into `SplitVec<T, Recursive>`;
    /// * however, not the other way around, since constant random access time requirements of `Doubling` are not satisfied.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut split_vec_linear = SplitVec::with_linear_growth(4);
    /// split_vec_linear.extend_from_slice(&['a', 'b', 'c']);
    /// assert_eq!(split_vec_linear, &['a', 'b', 'c']);
    ///
    /// let split_vec_recursive: SplitVec<_, Recursive> = split_vec_linear.into();
    /// assert_eq!(split_vec_recursive, &['a', 'b', 'c']);
    /// ```
    fn from(value: SplitVec<T, Linear>) -> Self {
        Self::from_raw_parts(value.len, value.fragments, Recursive)
    }
}

impl<T: Clone> From<Vec<T>> for SplitVec<T, Recursive> {
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
    /// let split_vec: SplitVec<_, Recursive> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert!(vec_capacity <= split_vec.capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        SplitVec::from_raw_parts(value.len(), vec![value.into()], Recursive)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn validate<G: Growth>(split: SplitVec<usize, G>)
    where
        SplitVec<usize, G>: Into<SplitVec<usize, Recursive>>,
    {
        let recursive: SplitVec<_, Recursive> = split.clone().into();

        assert_eq!(split.len(), recursive.len());
        for i in 0..split.len() {
            assert_eq!(split.get(i), recursive.get(i));
        }
    }

    #[test]
    fn into_recursive() {
        let mut vec = vec![];
        let mut linear = SplitVec::with_linear_growth(4);
        let mut doubling = SplitVec::with_doubling_growth();

        for i in 0..879 {
            vec.push(i);
            linear.push(i);
            doubling.push(i);
        }

        let recursive: SplitVec<_, Recursive> = vec.into();

        validate(recursive);
        validate(linear);
        validate(doubling);
    }
}
