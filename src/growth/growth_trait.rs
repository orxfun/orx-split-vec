use crate::Fragment;

/// Growth strategy of a split vector.
pub trait SplitVecGrowth<T>: Clone {
    /// Given that the split vector contains the given `fragments`,
    /// returns the capacity of the next fragment.
    fn new_fragment_capacity(&self, fragments: &[Fragment<T>]) -> usize;
    /// Returns the location of the element with the given `element_index` on the split vector
    /// as a tuple of (fragment-index, index-within-fragment).
    ///
    /// Returns None if the element index is out of bounds.
    fn get_fragment_and_inner_indices(
        &self,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)>;
}

/// Growth strategy of a split vector which allows flexible index access.
///
/// This marker trait practically means:
/// * the index access to a split vector element will be linear in the number of fragments,
/// * on the other hand, methods such as `append` will be available and cheap.
pub trait SplitVecGrowthWithFlexibleIndexAccess<T>: SplitVecGrowth<T> {}
