use crate::growth::growth_trait::Growth;
use crate::growth::linear::constants::FIXED_CAPACITIES;
use crate::{Fragment, SplitVec};

/// Stategy which allows the split vector to grow linearly.
///
/// In other words, each new fragment will have equal capacity,
/// which is equal to the capacity of the first fragment.
///
/// # Examples
///
/// ```
/// use orx_split_vec::prelude::*;
///
/// // SplitVec<usize, LinearGrowth>
/// let mut vec = SplitVec::with_linear_growth(4);
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
pub struct Linear {
    constant_fragment_capacity_exponent: usize,
    constant_fragment_capacity: usize,
}
impl Linear {
    pub(crate) fn new(constant_fragment_capacity_exponent: usize) -> Self {
        let constant_fragment_capacity = FIXED_CAPACITIES[constant_fragment_capacity_exponent];
        Self {
            constant_fragment_capacity_exponent,
            constant_fragment_capacity,
        }
    }
}

impl Growth for Linear {
    fn new_fragment_capacity<T>(&self, _fragments: &[Fragment<T>]) -> usize {
        self.constant_fragment_capacity
    }

    #[inline(always)]
    fn get_fragment_and_inner_indices<T>(
        &self,
        vec_len: usize,
        _fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        if element_index < vec_len {
            let f = element_index >> self.constant_fragment_capacity_exponent;
            let i = element_index % self.constant_fragment_capacity;
            Some((f, i))
        } else {
            None
        }
    }
}

impl<T> SplitVec<T, Linear> {
    /// Creates a split vector with linear growth where each fragment will have a capacity of `2 ^ constant_fragment_capacity_exponent`.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
    ///
    /// # Panics
    ///
    /// Panics if `constant_fragment_capacity_exponent` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// // SplitVec<usize, LinearGrowth>
    /// let mut vec = SplitVec::with_linear_growth(4);
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
    pub fn with_linear_growth(constant_fragment_capacity_exponent: usize) -> Self {
        assert!(constant_fragment_capacity_exponent > 0);
        let constant_fragment_capacity = FIXED_CAPACITIES[constant_fragment_capacity_exponent];
        Self {
            fragments: vec![Fragment::new(constant_fragment_capacity)],
            growth: Linear::new(constant_fragment_capacity_exponent),
            len: 0,
        }
    }
}
