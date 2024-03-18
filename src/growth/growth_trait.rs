use crate::Fragment;

/// Growth strategy of a split vector.
pub trait Growth: Clone {
    /// Given that the split vector contains the given `fragments`,
    /// returns the capacity of the next fragment.
    fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize;

    /// ***O(fragments.len())*** Returns the location of the element with the given `element_index` on the split vector as a tuple of (fragment-index, index-within-fragment).
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

    /// ***O(fragments.len())*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the  vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    unsafe fn get_ptr_mut<T>(&self, fragments: &mut [Fragment<T>], index: usize) -> Option<*mut T> {
        let mut prev_cumulative_capacity = 0;
        let mut cumulative_capacity = 0;
        for fragment in fragments {
            cumulative_capacity += fragment.capacity();
            if index < cumulative_capacity {
                let index_in_fragment = index - prev_cumulative_capacity;
                return Some(fragment.as_mut_ptr().add(index_in_fragment));
            }
            prev_cumulative_capacity = cumulative_capacity;
        }
        None
    }
}

/// Growth strategy of a split vector which allows for constant time access to the elements.
pub trait GrowthWithConstantTimeAccess: Growth {
    /// ***O(1)*** Returns the location of the element with the given `element_index` on the split vector as a tuple of (fragment-index, index-within-fragment).
    ///
    /// Notice that unlike the [`Growth::get_fragment_and_inner_indices`]:
    /// * this method does not receive the current state of the split vector,
    /// * therefore, it does not perform bounds check,
    /// * and hence, returns the expected fragment and within-fragment indices for any index computed by the constant access time function.
    fn get_fragment_and_inner_indices_unchecked(&self, element_index: usize) -> (usize, usize);

    /// ***O(1)*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the split vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    unsafe fn get_ptr_mut<T>(&self, fragments: &mut [Fragment<T>], index: usize) -> Option<*mut T> {
        let (f, i) = self.get_fragment_and_inner_indices_unchecked(index);
        fragments
            .get_mut(f)
            .map(|fragment| fragment.as_mut_ptr().add(i))
    }
}
