use crate::{Growth, SplitVec};
use orx_fixed_vec::FixedVec;
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
    /// let mut split_vec = SplitVec::with_linear_growth(4);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec: Vec<_> = split_vec.into();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(4);
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
    /// let mut split_vec = SplitVec::with_linear_growth(4);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec = split_vec.to_vec();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(4);
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

// orx_fixed_vec::FixedVec
impl<T, G> SplitVec<T, G>
where
    G: Growth,
    T: NotSelfRefVecItem + Clone,
{
    /// Collects the split vector into a fixed vector
    /// with a fixed capacity being exactly equal to the length of this split vector.
    ///
    /// # Safety
    ///
    /// Since `T: NotSelfRefVecItem`, it is safe to clone the data of the elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// // SplitVec with dynamic capacity and configurable growth strategy.
    /// let mut split = SplitVec::with_linear_growth(32);
    /// for i in 0..35 {
    ///     split.push(i);
    /// }
    /// assert_eq!(35, split.len());
    /// assert_eq!(2, split.fragments().len());
    /// assert_eq!(32, split.fragments()[0].len());
    /// assert_eq!(3, split.fragments()[1].len());
    ///
    /// // FixedVec with std::vec::Vec complexity & performance.
    /// let fixed = split.collect_fixed_vec();
    /// assert_eq!(35, fixed.len());
    /// assert_eq!(fixed, split);
    /// ```
    pub fn collect_fixed_vec(&self) -> FixedVec<T> {
        unsafe { self.unsafe_collect_fixed_vec() }
    }
}
impl<T, G> SplitVec<T, G>
where
    G: Growth,
    T: Clone,
{
    /// Collects the split vector into a fixed vector
    /// with a fixed capacity being exactly equal to the length of this split vector.
    ///
    /// # Safety
    ///
    /// Since `T` is not a `NotSelfRefVecItem`, it is assumed as a `SelfRefVecItem`
    /// to be conservative. A naive clone of a vector of `SelfRefVecItem` elements
    /// is unsafe due to the following scenario:
    ///
    /// * say the vector contains two elements `['a', 'b']` where `'a'` holds a reference to `'b'`.
    /// * when we clone this vector, element `'a'` of the second vector will be pointing to
    /// element `'b'` of the first vector, which is already incorrect.
    /// * furthermore, if the first vector is dropped, the abovementioned reference will be
    /// an dangling reference leading to UB.
    ///
    /// Therefore, cloning elements of a vector where elements are not `NotSelfRefVecItem`
    /// is `unsafe`.
    pub unsafe fn unsafe_collect_fixed_vec(&self) -> FixedVec<T> {
        let mut fixed = FixedVec::new(self.len());
        for fragment in &self.fragments {
            fixed.extend_from_slice(&fragment.data);
        }
        fixed
    }
}
