use crate::{fragment::fragment_struct::Fragment, FragmentGrowth};

#[derive(Debug)]
/// A split vector; i.e., a vector of fragments.
pub struct SplitVec<T> {
    pub(crate) fragments: Vec<Fragment<T>>,
    /// Fragment growth strategy of the split vector.
    pub growth: FragmentGrowth,
}

impl<T> SplitVec<T> {
    /// Creates an empty split vector with the given `growth` strategy.
    pub fn with_growth(growth: FragmentGrowth) -> Self {
        let capacity = growth.get_capacity(0);
        let fragment = Fragment::new(capacity);
        let fragments = vec![fragment];
        Self { fragments, growth }
    }
    /// Returns a mutable reference to the vector of fragments.
    ///
    /// # Safety
    ///
    /// Fragments of the split vector maintain the following structure:
    /// * the fragments vector is never empty, it has at least one fragment;
    /// * all fragments have a positive capacity;
    ///     * capacity of fragment f is equal to `self.growth.get_capacity(f)`.
    /// * if there exist F fragments in the vector:
    ///     * none of the fragments with indices `0..F-2` has capacity; i.e., len==capacity,
    ///     * the last fragment at position `F-1` might or might not have capacity.
    ///
    /// Breaking this structure invalidates the `SplitVec` struct,
    /// and its methods lead to UB.
    pub unsafe fn fragments_mut(&mut self) -> &mut Vec<Fragment<T>> {
        &mut self.fragments
    }
    /// Returns the fragments of the split vector.
    ///
    /// The fragments of the split vector satisfy the following structure:
    /// * the fragments vector is never empty, it has at least one fragment;
    /// * all fragments have a positive capacity;
    ///     * capacity of fragment f is equal to `self.growth.get_capacity(f)`.
    /// * if there exist F fragments in the vector:
    ///     * none of the fragments with indices `0..F-2` has capacity; i.e., len==capacity,
    ///     * the last fragment at position `F-1` might or might not have capacity.
    pub fn fragments(&self) -> &[Fragment<T>] {
        &self.fragments
    }
    /// Returns a reference to the last fragment of the split vector.
    pub fn last_fragment(&self) -> &Fragment<T> {
        &self.fragments[self.fragments.len() - 1]
    }
    /// Returns the fragment index and the index within fragment of the item with the given `index`;
    /// None if the index is out of bounds.
    pub fn fragment_and_inner_index(&self, index: usize) -> Option<(usize, usize)> {
        let mut prev_end = 0;
        let mut end = 0;
        for (f, fragment) in self.fragments.iter().enumerate() {
            end += fragment.len();
            if index < end {
                return Some((f, index - prev_end));
            }
            prev_end = end;
        }
        None
    }

    // helpers
    pub(crate) fn add_fragment(&mut self) {
        let capacity = self.growth.get_capacity(self.fragments.len());
        let new_fragment = Fragment::new(capacity);
        self.fragments.push(new_fragment);
    }
    pub(crate) fn add_fragment_with_first_value(&mut self, first_value: T) {
        let capacity = self.growth.get_capacity(self.fragments.len());
        let new_fragment = Fragment::new_with_first_value(capacity, first_value);
        self.fragments.push(new_fragment);
    }
}
