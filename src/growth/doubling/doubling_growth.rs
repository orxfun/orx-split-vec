use super::constants::*;
use crate::growth::growth_trait::{Growth, GrowthWithConstantTimeAccess};
use crate::{Fragment, SplitVec};

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

    /// ***O(1)*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the split vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    #[inline(always)]
    unsafe fn get_ptr_mut<T>(&self, fragments: &mut [Fragment<T>], index: usize) -> Option<*mut T> {
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
    unsafe fn get_ptr_mut_and_indices<T>(
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

        Err(format!(
            "Maximum cumulative capacity that can be reached by the Doubling strategy is {}.",
            CUMULATIVE_CAPACITIES[CUMULATIVE_CAPACITIES.len() - 1]
        ))
    }
}

impl GrowthWithConstantTimeAccess for Doubling {
    fn get_fragment_and_inner_indices_unchecked(&self, element_index: usize) -> (usize, usize) {
        let element_index_offset = element_index + FIRST_FRAGMENT_CAPACITY;
        let leading_zeros = usize::leading_zeros(element_index_offset) as usize;
        let f = OFFSET_FRAGMENT_IDX - leading_zeros;
        (f, element_index - CUMULATIVE_CAPACITIES[f])
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
        let fragments = Fragment::new(FIRST_FRAGMENT_CAPACITY).into_fragments();
        Self::from_raw_parts(0, fragments, Doubling)
    }

    /// Creates a new split vector with `Doubling` growth and initial `fragments_capacity`.
    ///
    /// This method differs from [`SplitVec::with_doubling_growth`] only by the pre-allocation of fragments collection.
    /// Note that this (only) important for concurrent programs:
    /// * SplitVec already keeps all elements pinned to their locations;
    /// * Creating a buffer for storing the meta information is important for keeping the meta information pinned as well.
    /// This is relevant and important for concurrent programs.
    ///
    /// # Panics
    ///
    /// Panics if `fragments_capacity == 0`.
    pub fn with_doubling_growth_and_fragments_capacity(fragments_capacity: usize) -> Self {
        assert!(fragments_capacity > 0);
        let fragments =
            Fragment::new(FIRST_FRAGMENT_CAPACITY).into_fragments_with_capacity(fragments_capacity);
        Self::from_raw_parts(0, fragments, Doubling)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orx_pinned_vec::{PinnedVec, PinnedVecGrowthError};
    use test_case::test_matrix;

    #[test]
    fn get_fragment_and_inner_indices() {
        let growth = Doubling;

        let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
        let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

        assert_eq!((0, 0), growth.get_fragment_and_inner_indices_unchecked(0));
        assert_eq!((0, 1), growth.get_fragment_and_inner_indices_unchecked(1));
        assert_eq!((1, 0), growth.get_fragment_and_inner_indices_unchecked(4));
        assert_eq!((1, 5), growth.get_fragment_and_inner_indices_unchecked(9));
        assert_eq!((2, 0), growth.get_fragment_and_inner_indices_unchecked(12));

        assert_eq!(Some((0, 0)), get(0));
        assert_eq!(Some((0, 1)), get(1));
        assert_eq!(Some((1, 0)), get(4));
        assert_eq!(Some((1, 5)), get(9));
        assert_eq!(Some((2, 0)), get(12));

        assert_eq!(None, get_none(0));
        assert_eq!(None, get_none(1));
        assert_eq!(None, get_none(4));
        assert_eq!(None, get_none(9));
        assert_eq!(None, get_none(12));
    }

    #[test]
    fn get_fragment_and_inner_indices_exhaustive() {
        let growth = Doubling;

        let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
        let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

        let mut f = 0;
        let mut prev_cumulative_capacity = 0;
        let mut curr_capacity = 4;
        let mut cumulative_capacity = 4;

        for index in 0..1111111 {
            if index == cumulative_capacity {
                prev_cumulative_capacity = cumulative_capacity;
                curr_capacity *= 2;
                cumulative_capacity += curr_capacity;
                f += 1;
            }

            let (f, i) = (f, index - prev_cumulative_capacity);
            assert_eq!(
                (f, i),
                growth.get_fragment_and_inner_indices_unchecked(index)
            );
            assert_eq!(Some((f, i)), get(index));
            assert_eq!(None, get_none(index));
        }
    }

    #[test]
    fn maximum_concurrent_capacity() {
        fn max_cap<T>(vec: &SplitVec<T, Doubling>) -> usize {
            vec.growth()
                .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
        }

        let mut vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
        assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);

        let until = max_cap(&vec);
        for _ in 0..until {
            vec.push('x');
            assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32);
        }

        // fragments allocate beyond max_cap
        vec.push('x');
        assert_eq!(max_cap(&vec), 4 + 8 + 16 + 32 + 64 + 128 + 256 + 512);
    }

    #[test_matrix([true, false])]
    fn with_doubling_growth(zero_memory: bool) {
        let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth();

        assert_eq!(4, vec.fragments.capacity());

        for _ in 0..100_000 {
            vec.push('x');
        }

        assert!(vec.fragments.capacity() > 4);

        let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth();
        let result = unsafe { vec.grow_to(100_000, zero_memory) };
        assert!(result.expect("must-be-ok") >= 100_000);
    }

    #[test]
    fn with_doubling_growth_and_fragments_capacity_normal_growth() {
        let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(1);

        assert_eq!(1, vec.fragments.capacity());

        for _ in 0..100_000 {
            vec.push('x');
        }

        assert!(vec.fragments.capacity() > 4);
    }

    #[test_matrix([true, false])]
    fn with_doubling_growth_and_fragments_capacity_concurrent_grow_never(zero_memory: bool) {
        let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(1);

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1, zero_memory) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test_matrix([true, false])]
    fn with_doubling_growth_and_fragments_capacity_concurrent_grow_once(zero_memory: bool) {
        let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(2);

        assert!(vec.can_concurrently_add_fragment());

        let next_capacity = vec.capacity() + vec.growth().new_fragment_capacity(vec.fragments());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1, zero_memory) };
        assert_eq!(result, Ok(next_capacity));

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1, zero_memory) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test_matrix([true, false])]
    fn with_doubling_growth_and_fragments_capacity_concurrent_grow_twice(zero_memory: bool) {
        // when possible
        let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(3);

        assert!(vec.can_concurrently_add_fragment());

        let fragment_2_capacity = vec.growth().new_fragment_capacity(vec.fragments());
        let fragment_3_capacity = fragment_2_capacity * 2;
        let new_capacity = vec.capacity() + fragment_2_capacity + fragment_3_capacity;

        let result = unsafe { vec.concurrently_grow_to(new_capacity - 1, zero_memory) };
        assert_eq!(result, Ok(new_capacity));

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1, zero_memory) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );

        // when not possible
        let mut vec: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(2);

        assert!(vec.can_concurrently_add_fragment()); // although we can add one fragment

        let result = unsafe { vec.concurrently_grow_to(new_capacity - 1, zero_memory) }; // we cannot add two
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test]
    #[should_panic]
    fn with_doubling_growth_and_fragments_capacity_zero() {
        let _: SplitVec<char, _> = SplitVec::with_doubling_growth_and_fragments_capacity(0);
    }

    #[test]
    fn required_fragments_len() {
        let vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
        let num_fragments = |max_cap| {
            vec.growth()
                .required_fragments_len(vec.fragments(), max_cap)
        };

        // 4 - 12 - 28 - 60 - 124
        assert_eq!(num_fragments(0), Ok(0));
        assert_eq!(num_fragments(1), Ok(1));
        assert_eq!(num_fragments(4), Ok(1));
        assert_eq!(num_fragments(5), Ok(2));
        assert_eq!(num_fragments(12), Ok(2));
        assert_eq!(num_fragments(13), Ok(3));
        assert_eq!(num_fragments(36), Ok(4));
        assert_eq!(num_fragments(67), Ok(5));
        assert_eq!(num_fragments(136), Ok(6));
    }

    #[test]
    fn required_fragments_len_at_max() {
        let vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
        let num_fragments = |max_cap| {
            vec.growth()
                .required_fragments_len(vec.fragments(), max_cap)
        };

        let maximum_possible_capacity = *CUMULATIVE_CAPACITIES.last().expect("is not empty");
        assert_eq!(num_fragments(maximum_possible_capacity), Ok(32));
    }

    #[test]
    fn required_fragments_len_more_than_max() {
        let vec: SplitVec<char, Doubling> = SplitVec::with_doubling_growth();
        let num_fragments = |max_cap| {
            vec.growth()
                .required_fragments_len(vec.fragments(), max_cap)
        };

        let more_than_max_possible_capacity =
            *CUMULATIVE_CAPACITIES.last().expect("is not empty") + 1;
        assert!(num_fragments(more_than_max_possible_capacity).is_err());
    }
}
