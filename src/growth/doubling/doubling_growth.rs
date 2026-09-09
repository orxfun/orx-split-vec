use super::constants::*;
use crate::growth::growth_trait::{Growth, GrowthWithConstantTimeAccess};
use crate::{Fragment, SplitVec};
use alloc::string::String;
use orx_pseudo_default::PseudoDefault;

/// Strategy which allows creates a fragment with double the capacity
/// of the prior fragment every time the split vector needs to expand.
///
/// Assuming it is the common case compared to empty vector scenarios,
/// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
///
/// # Examples
///
/// ```
/// use orx_split_vec::*;
///
/// // SplitVec<usize, Doubling>
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

impl PseudoDefault for Doubling {
    fn pseudo_default() -> Self {
        Default::default()
    }
}

impl Growth for Doubling {
    #[inline(always)]
    fn new_fragment_capacity_from(
        &self,
        fragment_capacities: impl ExactSizeIterator<Item = usize>,
    ) -> usize {
        fragment_capacities.last().map(|x| x * 2).unwrap_or(4)
    }

    #[inline(always)]
    fn get_fragment_and_inner_indices<T>(
        &self,
        vec_len: usize,
        _fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        match element_index < vec_len {
            true => Some(self.get_fragment_and_inner_indices_unchecked(element_index)),
            false => None,
        }
    }

    /// ***O(1)*** Returns a pointer to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the split vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    #[inline(always)]
    fn get_ptr<T>(&self, fragments: &[Fragment<T>], index: usize) -> Option<*const T> {
        <Self as GrowthWithConstantTimeAccess>::get_ptr(self, fragments, index)
    }

    /// ***O(1)*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the split vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    #[inline(always)]
    fn get_ptr_mut<T>(&self, fragments: &mut [Fragment<T>], index: usize) -> Option<*mut T> {
        <Self as GrowthWithConstantTimeAccess>::get_ptr_mut(self, fragments, index)
    }

    /// ***O(1)*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`
    /// together with the index of the fragment that the element belongs to
    /// and index of the element withing the respective fragment.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the split vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    #[inline(always)]
    fn get_ptr_mut_and_indices<T>(
        &self,
        fragments: &mut [Fragment<T>],
        index: usize,
    ) -> Option<(*mut T, usize, usize)> {
        <Self as GrowthWithConstantTimeAccess>::get_ptr_mut_and_indices(self, fragments, index)
    }

    fn maximum_concurrent_capacity<T>(
        &self,
        fragments: &[Fragment<T>],
        fragments_capacity: usize,
    ) -> usize {
        assert!(fragments_capacity >= fragments.len());

        CUMULATIVE_CAPACITIES[fragments_capacity]
    }

    /// Returns the number of fragments with this growth strategy in order to be able to reach a capacity of `maximum_capacity` of elements.
    ///
    /// This method is relevant and useful for concurrent programs, which helps in avoiding the fragments to allocate.
    ///
    /// # Panics
    ///
    /// Panics if `maximum_capacity` is greater than sum { 2^f | for f in 2..34 }.
    fn required_fragments_len<T>(
        &self,
        _: &[Fragment<T>],
        maximum_capacity: usize,
    ) -> Result<usize, String> {
        for (f, capacity) in CUMULATIVE_CAPACITIES.iter().enumerate() {
            if maximum_capacity <= *capacity {
                return Ok(f);
            }
        }

        Err(alloc::format!(
            "Maximum cumulative capacity that can be reached by the Doubling strategy is {}.",
            CUMULATIVE_CAPACITIES[CUMULATIVE_CAPACITIES.len() - 1]
        ))
    }

    fn maximum_concurrent_capacity_bound<T>(&self, _: &[Fragment<T>], _: usize) -> usize {
        *CUMULATIVE_CAPACITIES
            .last()
            .expect("cumulative capacities is not empty")
    }
}

impl GrowthWithConstantTimeAccess for Doubling {
    #[inline(always)]
    fn get_fragment_and_inner_indices_unchecked(&self, element_index: usize) -> (usize, usize) {
        let element_index_offset = element_index + FIRST_FRAGMENT_CAPACITY;
        let leading_zeros = usize::leading_zeros(element_index_offset) as usize;
        let f = OFFSET_FRAGMENT_IDX - leading_zeros;
        (f, element_index - CUMULATIVE_CAPACITIES[f])
    }

    fn fragment_capacity_of(&self, fragment_index: usize) -> usize {
        CAPACITIES[fragment_index]
    }
}

impl<T> SplitVec<T, Doubling> {
    /// Strategy which allows to create a fragment with double the capacity
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
    /// use orx_split_vec::*;
    ///
    /// // SplitVec<usize, Doubling>
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
        let fragments = Fragment::new_empty(FIRST_FRAGMENT_CAPACITY).into_fragments();
        Self::from_raw_parts(0, fragments, Doubling)
    }

    /// Creates a new split vector with `Doubling` growth and initial `fragments_capacity`.
    ///
    /// This method differs from [`SplitVec::with_doubling_growth`] only by the pre-allocation of fragments collection.
    /// Note that this (only) important for concurrent programs:
    /// * SplitVec already keeps all elements pinned to their locations;
    /// * Creating a buffer for storing the meta information is important for keeping the meta information pinned as well.
    ///   This is relevant and important for concurrent programs.
    ///
    /// # Panics
    ///
    /// Panics if `fragments_capacity == 0`.
    ///
    /// # Panics
    ///
    /// Panics if `fragments_capacity` is zero or `fragments_capacity > MAX_FRAGMENTS_CAPACITY` where `MAX_FRAGMENTS_CAPACITY` is:
    ///
    /// * 29 in 32-bit targets,
    /// * 32 in 64-bit.
    pub fn with_doubling_growth_and_fragments_capacity(fragments_capacity: usize) -> Self {
        assert!(
            fragments_capacity > 0 && fragments_capacity <= CAPACITIES_LEN,
            "fragments_capacity must be within 1..{CAPACITIES_LEN}; however, {fragments_capacity} is provided."
        );
        assert!(fragments_capacity > 0);
        let fragments = Fragment::new_empty(FIRST_FRAGMENT_CAPACITY)
            .into_fragments_with_capacity(fragments_capacity);
        Self::from_raw_parts(0, fragments, Doubling)
    }

    /// Creates a new split vector with `Doubling` growth and maximum concurrent capacity which depends
    /// on the pointer size of the target architecture.
    ///
    /// This method differs from [`SplitVec::with_doubling_growth`] only by the pre-allocation of fragments collection,
    /// which never contains more elements than 33.
    pub fn with_doubling_growth_and_max_concurrent_capacity() -> Self {
        let fragments = Fragment::new_empty(FIRST_FRAGMENT_CAPACITY)
            .into_fragments_with_capacity(CAPACITIES_LEN);
        Self::from_raw_parts(0, fragments, Doubling)
    }
}
