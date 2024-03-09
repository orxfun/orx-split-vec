use crate::Fragment;

/// Growth strategy of a split vector.
pub trait Growth: Clone {
    /// Given that the split vector contains the given `fragments`,
    /// returns the capacity of the next fragment.
    fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize;

    /// Returns the location of the element with the given `element_index` on the split vector as a tuple of (fragment-index, index-within-fragment).
    ///
    /// Returns None if the element index is out of bounds.
    fn get_fragment_and_inner_indices<T>(
        &self,
        _vec_len: usize,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        let mut prev_end = 0;
        let mut end = 0;
        for (f, fragment) in fragments.iter().enumerate() {
            end += fragment.len();
            if element_index < end {
                return Some((f, element_index - prev_end));
            }
            prev_end = end;
        }
        None
    }
}

/// Growth strategy of a split vector which allows for constant time access to the elements.
pub trait GrowthWithConstantTimeAccess: Growth {
    /// Returns the location of the element with the given `element_index` on the split vector as a tuple of (fragment-index, index-within-fragment).
    ///
    /// Notice that unlike the [`Growth::get_fragment_and_inner_indices`]:
    /// * this method does not receive the current state of the split vector,
    /// * therefore, it does not perform bounds check,
    /// * and hence, returns the expected fragment and within-fragment indices for any index computed by the constant access time function.
    fn get_fragment_and_inner_indices_unchecked(&self, element_index: usize) -> (usize, usize);
}
