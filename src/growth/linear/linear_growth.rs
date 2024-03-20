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
        Self {
            fragments: vec![Fragment::new(constant_fragment_capacity)],
            growth: Linear::new(constant_fragment_capacity_exponent),
            len: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
