use super::constants::*;
use crate::growth::growth_trait::Growth;
use crate::{Fragment, SplitVec};

/// Stategy which allows creates a fragment with double the capacity
/// of the prior fragment every time the split vector needs to expand.
///
/// Assuming it is the common case compared to empty vector scenarios,
/// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
///
/// # Examples
///
/// ```
/// use orx_split_vec::prelude::*;
///
/// // SplitVec<usize, DoublingGrowth>
/// let mut vec = SplitVec::with_doubling_growth();
///
/// assert_eq!(1, vec.fragments().len());
/// assert_eq!(Some(4), vec.fragments().first().map(|f| f.capacity()));
/// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
///
/// // fill the first 5 fragments
/// let expected_fragment_capacities = vec![4, 8, 16, 32];
/// let num_items: usize = expected_fragment_capacities.iter().sum();
/// for i in 0..num_items {
///     vec.push(i);
/// }
///
/// assert_eq!(
///     expected_fragment_capacities,
///     vec.fragments()
///     .iter()
///     .map(|f| f.capacity())
///     .collect::<Vec<_>>()
/// );
/// assert_eq!(
///     expected_fragment_capacities,
///     vec.fragments().iter().map(|f| f.len()).collect::<Vec<_>>()
/// );
///
/// // create the 6-th fragment doubling the capacity
/// vec.push(42);
/// assert_eq!(
///     vec.fragments().len(),
///     expected_fragment_capacities.len() + 1
/// );
///
/// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some(32 * 2));
/// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Doubling;

impl Growth for Doubling {
    fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
        fragments.last().map(|f| f.capacity() * 2).unwrap_or(4)
    }

    #[inline(always)]
    fn get_fragment_and_inner_indices<T>(
        &self,
        vec_len: usize,
        _fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        if element_index < vec_len {
            let element_index_offset = element_index + FIRST_FRAGMENT_CAPACITY;
            let leading_zeros = usize::leading_zeros(element_index_offset) as usize;
            let f = OFFSET_FRAGMENT_IDX - leading_zeros;
            Some((f, element_index - CUMULATIVE_CAPACITIES[f]))
        } else {
            None
        }
    }
}

impl<T> SplitVec<T, Doubling> {
    /// Stategy which allows to create a fragment with double the capacity
    /// of the prior fragment every time the split vector needs to expand.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
    ///
    /// # Panics
    /// Panics if `first_fragment_capacity` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// // SplitVec<usize, DoublingGrowth>
    /// let mut vec = SplitVec::with_doubling_growth();
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(4), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // fill the first 5 fragments
    /// let expected_fragment_capacities = vec![4, 8, 16, 32];
    /// let num_items: usize = expected_fragment_capacities.iter().sum();
    /// for i in 0..num_items {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments()
    ///     .iter()
    ///     .map(|f| f.capacity())
    ///     .collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments().iter().map(|f| f.len()).collect::<Vec<_>>()
    /// );
    ///
    /// // create the 6-th fragment doubling the capacity
    /// vec.push(42);
    /// assert_eq!(
    ///     vec.fragments().len(),
    ///     expected_fragment_capacities.len() + 1
    /// );
    ///
    /// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some(32 * 2));
    /// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
    /// ```
    pub fn with_doubling_growth() -> Self {
        Self {
            fragments: vec![Fragment::new(FIRST_FRAGMENT_CAPACITY)],
            growth: Doubling,
            len: 0,
        }
    }
}
