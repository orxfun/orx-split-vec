use crate::{SplitVec, SplitVecGrowth};
use std::ops::{Index, IndexMut};

impl<T, G> Index<usize> for SplitVec<T, G>
where
    G: SplitVecGrowth,
{
    type Output = T;
    /// Returns a reference to the `index`-th item of the vector.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
    ///
    /// vec.extend_from_slice(&[0, 1, 2, 3]);
    ///
    /// assert_eq!(&1, &vec[1]);
    /// assert_eq!(&3, &vec[3]);
    /// // let x = &vec[4]; // panics!
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        let (f, i) = self
            .get_fragment_and_inner_indices(index)
            .expect("index is out of bounds");
        &self.fragments[f][i]
    }
}

impl<T, G> IndexMut<usize> for SplitVec<T, G>
where
    G: SplitVecGrowth,
{
    /// Returns a mutable reference to the `index`-th item of the vector.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
    ///
    /// vec.extend_from_slice(&[0, 1, 2, 3]);
    ///
    /// let item2 = &mut vec[2];
    /// *item2 = 42;
    /// assert_eq!(vec, &[0, 1, 42, 3]);
    ///
    /// // let x = &mut vec[4]; // panics!
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let (f, i) = self
            .get_fragment_and_inner_indices(index)
            .expect("index is out of bounds");
        &mut self.fragments[f][i]
    }
}

impl<T, G> Index<(usize, usize)> for SplitVec<T, G>
where
    G: SplitVecGrowth,
{
    type Output = T;
    /// One can treat the split vector as a jagged array
    /// and access an item with (fragment_index, inner_fragment_index)
    /// if these numbers are known.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * `fragment_and_inner_index.0` is not a valid fragment index; i.e., not within `0..self.fragments().len()`, or
    /// * `fragment_and_inner_index.1` is not a valid index for the corresponding fragment; i.e., not within `0..self.fragments()[fragment_and_inner_index.0].len()`.
    ///
    /// # Examples
    ///
    /// Assume that we create a split vector with a constant growth of N elements.
    /// This means that each fraction will have a capacity and max-length of N.
    ///
    /// Then, the fragment and inner index of the element with index `i` can be computed as
    /// `(i / N, i % N)`.
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
    ///
    /// for i in 0..10 {
    ///     vec.push(i);
    /// }
    ///
    /// // layout of the data will be as follows:
    /// // fragment-0: [0, 1, 2, 3]
    /// // fragment-1: [4, 5, 6, 7]
    /// // fragment-2: [8, 9]
    ///
    /// assert_eq!(1, vec[(0, 1)]);
    /// assert_eq!(7, vec[(1, 3)]);
    /// assert_eq!(8, vec[(2, 0)]);
    ///
    /// // since we know the layout, we can define the index transformer for direct access
    /// fn fragment_and_inner_idx(index: usize) -> (usize, usize) {
    ///     (index / 4, index % 4)
    /// }
    ///
    /// for index in 0..vec.len() {
    ///     let split_access = &vec[index];
    ///     let direct_access = &vec[fragment_and_inner_idx(index)];
    ///     assert_eq!(split_access, direct_access);
    /// }
    ///
    /// ```
    fn index(&self, fragment_and_inner_index: (usize, usize)) -> &Self::Output {
        &self.fragments[fragment_and_inner_index.0][fragment_and_inner_index.1]
    }
}
impl<T, G> IndexMut<(usize, usize)> for SplitVec<T, G>
where
    G: SplitVecGrowth,
{
    /// One can treat the split vector as a jagged array
    /// and access an item with (fragment_index, inner_fragment_index)
    /// if these numbers are known.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * `fragment_and_inner_index.0` is not a valid fragment index; i.e., not within `0..self.fragments().len()`, or
    /// * `fragment_and_inner_index.1` is not a valid index for the corresponding fragment; i.e., not within `0..self.fragments()[fragment_and_inner_index.0].len()`.
    ///
    /// # Examples
    ///
    /// Assume that we create a split vector with a constant growth of N elements.
    /// This means that each fraction will have a capacity and max-length of N.
    ///
    /// Then, the fragment and inner index of the element with index `i` can be computed as
    /// `(i / N, i % N)`.
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
    ///
    /// for i in 0..10 {
    ///     vec.push(i);
    /// }
    ///
    /// // layout of the data will be as follows:
    /// // fragment-0: [0, 1, 2, 3]
    /// // fragment-1: [4, 5, 6, 7]
    /// // fragment-2: [8, 9]
    ///
    /// vec[(0, 1)] += 100; // 1 -> 101
    /// vec[(1, 3)] += 100; // 7 -> 107
    /// vec[(2, 0)] += 100; // 8 -> 108
    /// assert_eq!(vec, &[0, 101, 2, 3, 4, 5, 6, 107, 108, 9]);
    ///
    /// // since we know the layout, we can define the index transformer for direct access
    /// fn fragment_and_inner_idx(index: usize) -> (usize, usize) {
    ///     (index / 4, index % 4)
    /// }
    ///
    /// for index in 0..vec.len() {
    ///     let direct_access = &mut vec[fragment_and_inner_idx(index)];
    ///     if *direct_access < 100 {
    ///         *direct_access += 100;
    ///     }
    /// }
    /// assert_eq!(vec, &[100, 101, 102, 103, 104, 105, 106, 107, 108, 109]);
    ///
    /// ```
    fn index_mut(&mut self, fragment_and_inner_index: (usize, usize)) -> &mut Self::Output {
        &mut self.fragments[fragment_and_inner_index.0][fragment_and_inner_index.1]
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn index() {
        fn test<G: SplitVecGrowth>(mut vec: SplitVec<usize, G>) {
            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            vec.extend_from_slice(&(42..63).collect::<Vec<_>>());
            vec.extend_from_slice(&(63..100).collect::<Vec<_>>());

            assert_eq!(100, vec.len());
            for i in 0..100 {
                assert_eq!(i, vec[i]);
                vec[i] += 100;
            }
            for i in 0..100 {
                assert_eq!(100 + i, vec[i]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn double_indices() {
        fn test<G: SplitVecGrowth>(mut vec: SplitVec<usize, G>) {
            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            vec.extend_from_slice(&(42..63).collect::<Vec<_>>());
            vec.extend_from_slice(&(63..100).collect::<Vec<_>>());

            assert_eq!(100, vec.len());
            for i in 0..100 {
                let (f, j) = vec.get_fragment_and_inner_indices(i).expect("is-some");
                assert_eq!(i, vec[(f, j)]);
                vec[(f, j)] += 100;
            }
            for i in 0..100 {
                let (f, j) = vec.get_fragment_and_inner_indices(i).expect("is-some");
                assert_eq!(100 + i, vec[(f, j)]);
            }
        }
        test_all_growth_types!(test);
    }
}
