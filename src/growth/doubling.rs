use crate::{Fragment, SplitVec};

use super::growth_trait::SplitVecGrowth;

/// Stategy which allows creates a fragment with double the capacity
/// of the prior fragment every time the split vector needs to expand.
///
/// Assuming it is the common case compared to empty vector scenarios,
/// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
///
/// # Examples
///
/// ```
/// use orx_split_vec::SplitVec;
///
/// // SplitVec<usize, DoublingGrowth>
/// let mut vec = SplitVec::with_doubling_growth(2);
///
/// assert_eq!(1, vec.fragments().len());
/// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
/// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
///
/// // fill the first 5 fragments
/// let expected_fragment_capacities = vec![2, 4, 8, 16, 32];
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
pub struct DoublingGrowth;

impl<T> SplitVecGrowth<T> for DoublingGrowth {
    fn new_fragment_capacity(&self, fragments: &[Fragment<T>]) -> usize {
        fragments.last().map(|f| f.capacity() * 2).unwrap_or(4)
    }

    fn get_fragment_and_inner_indices(
        &self,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        let c = fragments.first().map(|f| f.capacity()).unwrap_or(4);

        if element_index < c && element_index < fragments[0].len() {
            Some((0, element_index))
        } else {
            let f = ((element_index + c) as f32 / c as f32).log2() as usize;
            let beg = (usize::pow(2, f as u32) - 1) * c;
            let i = element_index - beg;
            if f < fragments.len() && i < fragments[f].len() {
                Some((f, i))
            } else {
                None
            }
        }
    }
}

impl<T> SplitVec<T, DoublingGrowth> {
    /// Stategy which allows creates a fragment with double the capacity
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
    /// use orx_split_vec::SplitVec;
    ///
    /// // SplitVec<usize, DoublingGrowth>
    /// let mut vec = SplitVec::with_doubling_growth(2);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // fill the first 5 fragments
    /// let expected_fragment_capacities = vec![2, 4, 8, 16, 32];
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
    pub fn with_doubling_growth(first_fragment_capacity: usize) -> Self {
        assert!(first_fragment_capacity > 0);
        Self {
            fragments: vec![Fragment::new(first_fragment_capacity)],
            growth: DoublingGrowth,
        }
    }
}
