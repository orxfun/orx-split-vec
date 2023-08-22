use super::growth_trait::SplitVecGrowth;
use crate::{Fragment, SplitVec};

/// Stategy which allows the split vector to grow linearly.
///
/// In other words, each new fragment will have equal capacity,
/// which is equal to the capacity of the first fragment.
///
/// # Examples
///
/// ```
/// use orx_split_vec::SplitVec;
///
/// // SplitVec<usize, LinearGrowth>
/// let mut vec = SplitVec::with_linear_growth(16);
///
/// assert_eq!(1, vec.fragments().len());
/// assert_eq!(Some(16), vec.fragments().first().map(|f| f.capacity()));
/// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
///
/// // push 160 elements
/// for i in 0..10 * 16 {
///     vec.push(i);
/// }
///
/// assert_eq!(10, vec.fragments().len());
/// for fragment in vec.fragments() {
///     assert_eq!(16, fragment.len());
///     assert_eq!(16, fragment.capacity());
/// }
///
/// // push the 161-st element
/// vec.push(42);
/// assert_eq!(11, vec.fragments().len());
/// assert_eq!(Some(16), vec.fragments().last().map(|f| f.capacity()));
/// assert_eq!(Some(1), vec.fragments().last().map(|f| f.len()));
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LinearGrowth;

const DEFAULT_FRAGMENT_CAPACITY: usize = 32;

impl<T> SplitVecGrowth<T> for LinearGrowth {
    fn new_fragment_capacity(&self, fragments: &[Fragment<T>]) -> usize {
        fragments
            .last()
            .map(|f| f.capacity())
            .unwrap_or(DEFAULT_FRAGMENT_CAPACITY)
    }
    fn get_fragment_and_inner_indices(
        &self,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        let cap = fragments[fragments.len() - 1].capacity();

        let mut div = 0;
        let mut rem = element_index;

        while rem >= cap {
            div += 1;
            rem -= cap;
        }

        if div < fragments.len() && rem < fragments[div].len() {
            Some((div, rem))
        } else {
            None
        }
    }
}

impl<T> SplitVec<T, LinearGrowth> {
    /// Creates a split vector with linear growth and given `constant_fragment_capacity`.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
    ///
    /// # Panics
    /// Panics if `constant_fragment_capacity` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// // SplitVec<usize, LinearGrowth>
    /// let mut vec = SplitVec::with_linear_growth(16);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(16), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // push 160 elements
    /// for i in 0..10 * 16 {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(10, vec.fragments().len());
    /// for fragment in vec.fragments() {
    ///     assert_eq!(16, fragment.len());
    ///     assert_eq!(16, fragment.capacity());
    /// }
    ///
    /// // push the 161-st element
    /// vec.push(42);
    /// assert_eq!(11, vec.fragments().len());
    /// assert_eq!(Some(16), vec.fragments().last().map(|f| f.capacity()));
    /// assert_eq!(Some(1), vec.fragments().last().map(|f| f.len()));
    /// ```
    pub fn with_linear_growth(constant_fragment_capacity: usize) -> Self {
        assert!(constant_fragment_capacity > 0);
        Self {
            fragments: vec![Fragment::new(constant_fragment_capacity)],
            growth: LinearGrowth,
        }
    }
}
