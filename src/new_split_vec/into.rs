use crate::{Growth, SplitVec};
use orx_pinned_vec::{NotSelfRefVecItem, PinnedVec};

// std::vec::vec
impl<T, G> From<SplitVec<T, G>> for Vec<T>
where
    G: Growth,
    T: NotSelfRefVecItem,
{
    /// Converts the `SplitVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec: Vec<_> = split_vec.into();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// for i in 0..10 {
    ///     split_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], split_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], split_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], split_vec.fragments()[2].as_slice());
    ///
    /// let vec: Vec<_> = split_vec.into();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    fn from(mut value: SplitVec<T, G>) -> Self {
        let mut vec = vec![];
        vec.reserve(value.len());
        for f in &mut value.fragments {
            vec.append(&mut f.data);
        }
        vec
    }
}

impl<T, G> SplitVec<T, G>
where
    G: Growth,
    T: NotSelfRefVecItem,
{
    /// Converts the `SplitVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec = split_vec.to_vec();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(2);
    /// for i in 0..10 {
    ///     split_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], split_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], split_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], split_vec.fragments()[2].as_slice());
    ///
    /// let vec = split_vec.to_vec();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        self.into()
    }
}
