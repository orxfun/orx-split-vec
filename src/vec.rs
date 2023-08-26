use crate::{growth::growth_trait::SplitVecGrowthWithFlexibleIndexAccess, Fragment, SplitVec};
use orx_pinned_vec::PinnedVec;

impl<T, GFlex> SplitVec<T, GFlex>
where
    GFlex: SplitVecGrowthWithFlexibleIndexAccess<T>,
{
    /// Directly appends the `fragment` to the end of the split vector.
    ///
    /// This operation does not require any copies or allocation;
    /// the fragment is moved into the split vector and added as a new fragment,
    /// without copying the underlying data.
    ///
    /// This method is not available for `SplitVec<_, LinearGrowth>` and
    /// `SplitVec<_, DoublingGrowth>` since those variants exploit the closed
    /// form formula to speed up element accesses by index.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_exponential_growth(8, 1.5);
    ///
    /// // append to empty split vector
    /// assert!(vec.is_empty());
    /// let mut other = Vec::with_capacity(4);
    /// other.extend_from_slice(&[0, 1, 2]);
    ///
    /// vec.append(other);
    /// assert_eq!(vec, &[0, 1, 2]);
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(4, vec.fragments()[0].capacity()); // SplitVec will make use of the appended vector's additional capacity
    ///
    /// vec.push(3);
    /// assert_eq!(vec, &[0, 1, 2, 3]);
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(vec.fragments()[0].as_slice(), &[0, 1, 2, 3]);
    ///
    /// // next push will use SplitVec's growth
    /// vec.extend_from_slice(&[4, 5, 6]);
    /// assert_eq!(vec, &[0, 1, 2, 3, 4, 5, 6]);
    /// assert_eq!(2, vec.fragments().len());
    /// assert_eq!(vec.fragments()[0].as_slice(), &[0, 1, 2, 3]);
    /// assert_eq!(vec.fragments()[1].as_slice(), &[4, 5, 6]);
    ///
    /// // we can append another fragment directly
    /// vec.append(vec![7, 8]);
    /// assert_eq!(vec, &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
    /// assert_eq!(3, vec.fragments().len());
    /// assert_eq!(vec.fragments()[0].as_slice(), &[0, 1, 2, 3]);
    /// assert_eq!(vec.fragments()[1].as_slice(), &[4, 5, 6]);
    /// assert_eq!(vec.fragments()[2].as_slice(), &[7, 8]);
    /// ```
    pub fn append<F>(&mut self, fragment: F)
    where
        F: Into<Fragment<T>>,
    {
        if self.is_empty() {
            self.fragments[0] = fragment.into();
        } else {
            self.fragments.push(fragment.into());
        }
    }
}
