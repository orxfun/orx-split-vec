use crate::growth::growth_trait::{Growth, GrowthWithConstantTimeAccess};
use crate::growth::linear::constants::FIXED_CAPACITIES;
use crate::{Fragment, SplitVec};

/// Strategy which allows the split vector to grow linearly.
///
/// In other words, each new fragment will have equal capacity,
/// which is equal to the capacity of the first fragment.
///
/// # Examples
///
/// ```
/// use orx_split_vec::*;
///
/// // SplitVec<usize, Linear>
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
#[derive(Debug, Clone, PartialEq)]
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

    fn maximum_concurrent_capacity<T>(
        &self,
        fragments: &[Fragment<T>],
        fragments_capacity: usize,
    ) -> usize {
        assert!(fragments_capacity >= fragments.len());

        fragments_capacity * self.constant_fragment_capacity
    }

    fn required_fragments_len<T>(&self, _: &[Fragment<T>], maximum_capacity: usize) -> usize {
        let num_full_fragments = maximum_capacity / self.constant_fragment_capacity;
        let remainder = maximum_capacity % self.constant_fragment_capacity;
        let additional_fragment = if remainder > 0 { 1 } else { 0 };

        num_full_fragments + additional_fragment
    }
}

impl GrowthWithConstantTimeAccess for Linear {
    fn get_fragment_and_inner_indices_unchecked(&self, element_index: usize) -> (usize, usize) {
        let f = element_index >> self.constant_fragment_capacity_exponent;
        let i = element_index % self.constant_fragment_capacity;
        (f, i)
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
    /// use orx_split_vec::*;
    ///
    /// // SplitVec<usize, Linear>
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
        let fragments = Fragment::new(constant_fragment_capacity).into_fragments();
        let growth = Linear::new(constant_fragment_capacity_exponent);
        Self::from_raw_parts(0, fragments, growth)
    }

    /// Creates a new split vector with `Linear` growth and initial `fragments_capacity`.
    ///
    /// This method differs from [`SplitVec::with_linear_growth`] only by the pre-allocation of fragments collection.
    /// Note that this (only) important for concurrent programs:
    /// * SplitVec already keeps all elements pinned to their locations;
    /// * Creating a buffer for storing the meta information is important for keeping the meta information pinned as well.
    /// This is relevant and important for concurrent programs.
    ///
    /// # Panics
    ///
    /// Panics if `fragments_capacity == 0`.
    pub fn with_linear_growth_and_fragments_capacity(
        constant_fragment_capacity_exponent: usize,
        fragments_capacity: usize,
    ) -> Self {
        assert!(constant_fragment_capacity_exponent > 0);
        assert!(fragments_capacity > 0);

        let constant_fragment_capacity = FIXED_CAPACITIES[constant_fragment_capacity_exponent];
        let fragments = Fragment::new(constant_fragment_capacity)
            .into_fragments_with_capacity(fragments_capacity);
        let growth = Linear::new(constant_fragment_capacity_exponent);
        Self::from_raw_parts(0, fragments, growth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orx_pinned_vec::{PinnedVec, PinnedVecGrowthError};

    #[test]
    fn get_fragment_and_inner_indices() {
        let growth = Linear::new(2);

        let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
        let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

        assert_eq!((0, 0), growth.get_fragment_and_inner_indices_unchecked(0));
        assert_eq!((0, 1), growth.get_fragment_and_inner_indices_unchecked(1));
        assert_eq!((1, 0), growth.get_fragment_and_inner_indices_unchecked(4));
        assert_eq!((2, 1), growth.get_fragment_and_inner_indices_unchecked(9));
        assert_eq!((4, 0), growth.get_fragment_and_inner_indices_unchecked(16));

        assert_eq!(Some((0, 0)), get(0));
        assert_eq!(Some((0, 1)), get(1));
        assert_eq!(Some((1, 0)), get(4));
        assert_eq!(Some((2, 1)), get(9));
        assert_eq!(Some((4, 0)), get(16));

        assert_eq!(None, get_none(0));
        assert_eq!(None, get_none(1));
        assert_eq!(None, get_none(4));
        assert_eq!(None, get_none(9));
        assert_eq!(None, get_none(16));
    }

    #[test]
    fn get_fragment_and_inner_indices_exhaustive() {
        let growth = Linear::new(5);

        let get = |index| growth.get_fragment_and_inner_indices::<char>(usize::MAX, &[], index);
        let get_none = |index| growth.get_fragment_and_inner_indices::<char>(index, &[], index);

        let curr_capacity = 32;

        let mut f = 0;
        let mut prev_cumulative_capacity = 0;
        let mut cumulative_capacity = curr_capacity;

        for index in 0..1111111 {
            if index == cumulative_capacity {
                prev_cumulative_capacity = cumulative_capacity;
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
        fn max_cap<T>(vec: &SplitVec<T, Linear>) -> usize {
            vec.growth()
                .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
        }

        let mut vec: SplitVec<char, Linear> = SplitVec::with_linear_growth(5);
        assert_eq!(max_cap(&vec), 4 * 2usize.pow(5));

        let until = max_cap(&vec);
        for _ in 0..until {
            vec.push('x');
            assert_eq!(max_cap(&vec), 4 * 2usize.pow(5));
        }

        // fragments allocate beyond max_cap
        vec.push('x');
        assert_eq!(max_cap(&vec), 8 * 2usize.pow(5));
    }

    #[test]
    fn with_linear_growth() {
        let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth(10);

        assert_eq!(4, vec.fragments.capacity());

        for _ in 0..100_000 {
            vec.push('x');
        }

        assert!(vec.fragments.capacity() > 4);

        let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth(10);
        let result = unsafe { vec.grow_to(100_000) };
        assert!(result.is_ok());
        assert!(result.expect("is-ok") >= 100_000);
    }

    #[test]
    fn with_linear_growth_and_fragments_capacity_normal_growth() {
        let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 1);

        assert_eq!(1, vec.fragments.capacity());

        for _ in 0..100_000 {
            vec.push('x');
        }

        assert!(vec.fragments.capacity() > 4);
    }

    #[test]
    fn with_linear_growth_and_fragments_capacity_concurrent_grow_never() {
        let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 1);

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test]
    fn with_linear_growth_and_fragments_capacity_concurrent_grow_once() {
        let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 2);

        assert!(vec.can_concurrently_add_fragment());

        let next_capacity = vec.capacity() + vec.growth().new_fragment_capacity(vec.fragments());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(result, Ok(next_capacity));

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test]
    fn with_linear_growth_and_fragments_capacity_concurrent_grow_twice() {
        // when possible
        let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 3);

        assert!(vec.can_concurrently_add_fragment());

        let fragment_2_capacity = vec.growth().new_fragment_capacity(vec.fragments());
        let fragment_3_capacity = fragment_2_capacity;
        let new_capacity = vec.capacity() + fragment_2_capacity + fragment_3_capacity;

        let result = unsafe { vec.concurrently_grow_to(new_capacity - 1) };
        assert_eq!(result, Ok(new_capacity));

        assert!(!vec.can_concurrently_add_fragment());

        let result = unsafe { vec.concurrently_grow_to(vec.capacity() + 1) };
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );

        // when not possible
        let mut vec: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 2);

        assert!(vec.can_concurrently_add_fragment()); // although we can add one fragment

        let result = unsafe { vec.concurrently_grow_to(new_capacity - 1) }; // we cannot add two
        assert_eq!(
            result,
            Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
        );
    }

    #[test]
    #[should_panic]
    fn with_linear_growth_and_fragments_capacity_zero() {
        let _: SplitVec<char, _> = SplitVec::with_linear_growth_and_fragments_capacity(10, 0);
    }

    #[test]
    fn required_fragments_len() {
        let vec: SplitVec<char, Linear> = SplitVec::with_linear_growth(5);
        let num_fragments = |max_cap| {
            vec.growth()
                .required_fragments_len(vec.fragments(), max_cap)
        };

        assert_eq!(num_fragments(0), 0);
        assert_eq!(num_fragments(1), 1);
        assert_eq!(num_fragments(2), 1);
        assert_eq!(num_fragments(32), 1);
        assert_eq!(num_fragments(33), 2);
        assert_eq!(num_fragments(32 * 7), 7);
        assert_eq!(num_fragments(32 * 7 + 1), 8);
    }
}
