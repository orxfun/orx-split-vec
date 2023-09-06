use crate::{SplitVec, SplitVecGrowth};
use orx_pinned_vec::PinnedVec;

impl<'a, T: Clone + 'a, G> Extend<&'a T> for SplitVec<T, G>
where
    G: SplitVecGrowth,
{
    /// Clones and appends all elements in the iterator to the vec.
    ///
    /// Iterates over the `iter`, clones each element, and then appends
    /// it to this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend(&[4, 5, 6, 7]);
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    ///
    /// let mut sec_vec = SplitVec::with_linear_growth(4);
    /// sec_vec.extend(vec.iter());
    /// assert_eq!(sec_vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x.clone());
        }
    }
}

impl<T, G> Extend<T> for SplitVec<T, G>
where
    G: SplitVecGrowth,
{
    /// Extends a collection with the contents of an iterator.
    ///
    /// Iterates over the `iter`, moves and appends each element
    /// to this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend(vec![4, 5, 6, 7].into_iter());
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn extend() {
        fn test<G: SplitVecGrowth>(mut vec: SplitVec<usize, G>) {
            vec.extend(0..42);
            vec.extend(&(42..63).collect::<Vec<_>>());
            vec.extend((53..90).map(|i| i + 10));

            assert_eq!(100, vec.len());
            for i in 0..100 {
                assert_eq!(i, vec[i]);
            }
        }
        test_all_growth_types!(test);
    }
}
