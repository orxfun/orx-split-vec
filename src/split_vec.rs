use crate::{fragment::fragment_struct::Fragment, Doubling, Growth};

/// A split vector; i.e., a vector of fragments, with the following features:
///
/// * Flexible in growth strategies; custom strategies can be defined.
/// * Growth does not cause any memory copies.
/// * Capacity of an already created fragment is never changed.
/// * The above feature allows the data to stay pinned in place. Memory location of an item added to the split vector will never change unless it is removed from the vector or the vector is dropped.
pub struct SplitVec<T, G = Doubling>
where
    G: Growth,
{
    pub(crate) len: usize,
    pub(crate) fragments: Vec<Fragment<T>>,
    /// Growth strategy of the split vector.
    ///
    /// Note that allocated data of split vector is pinned and allocated in fragments.
    /// Therefore, growth does not require copying data.
    ///
    /// The growth strategy determines the capacity of each fragment
    /// that will be added to the split vector when needed.
    ///
    /// Furthermore, it has an impact on index-access to the elements.
    /// See below for the complexities:
    ///
    /// * `Linear` (`SplitVec::with_linear_growth`) -> O(1)
    /// * `Doubling` (`SplitVec::with_doubling_growth`) -> O(1)
    /// * `Recursive` (`SplitVec::with_recursive_growth`) -> O(f) where f is the number of fragments; and O(1) append time complexity
    pub growth: G,
}

impl<T, G> SplitVec<T, G>
where
    G: Growth,
{
    pub(crate) fn from_raw_parts(len: usize, fragments: Vec<Fragment<T>>, growth: G) -> Self {
        debug_assert_eq!(len, fragments.iter().map(|x| x.len()).sum());
        Self {
            len,
            fragments,
            growth,
        }
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
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(2);
    ///
    /// for i in 0..6 {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(2, vec.fragments().len());
    /// assert_eq!(&[0, 1, 2, 3], vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5], vec.fragments()[1].as_slice());
    ///
    /// ```
    pub fn fragments(&self) -> &[Fragment<T>] {
        &self.fragments
    }

    /// Returns the fragment index and the index within fragment of the item with the given `index`;
    /// None if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(2);
    ///
    /// for i in 0..6 {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(&[0, 1, 2, 3], vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5], vec.fragments()[1].as_slice());
    ///
    /// // first fragment
    /// assert_eq!(Some((0, 0)), vec.get_fragment_and_inner_indices(0));
    /// assert_eq!(Some((0, 1)), vec.get_fragment_and_inner_indices(1));
    /// assert_eq!(Some((0, 2)), vec.get_fragment_and_inner_indices(2));
    /// assert_eq!(Some((0, 3)), vec.get_fragment_and_inner_indices(3));
    ///
    /// // second fragment
    /// assert_eq!(Some((1, 0)), vec.get_fragment_and_inner_indices(4));
    /// assert_eq!(Some((1, 1)), vec.get_fragment_and_inner_indices(5));
    ///
    /// // out of bounds
    /// assert_eq!(None, vec.get_fragment_and_inner_indices(6));
    /// ```
    #[inline(always)]
    pub fn get_fragment_and_inner_indices(&self, index: usize) -> Option<(usize, usize)> {
        self.growth
            .get_fragment_and_inner_indices(self.len, &self.fragments, index)
    }

    // helpers
    pub(crate) fn has_capacity_for_one(&self) -> bool {
        self.fragments
            .last()
            .map(|f| f.has_capacity_for_one())
            .unwrap_or(false)
    }

    pub(crate) fn add_fragment(&mut self) {
        let capacity = self.growth.new_fragment_capacity(&self.fragments);
        let new_fragment = Fragment::new(capacity);
        self.fragments.push(new_fragment);
    }

    pub(crate) fn add_fragment_with_first_value(&mut self, first_value: T) {
        let capacity = self.growth.new_fragment_capacity(&self.fragments);
        let new_fragment = Fragment::new_with_first_value(capacity, first_value);
        self.fragments.push(new_fragment);
    }

    pub(crate) fn drop_last_empty_fragment(&mut self) {
        let drop_empty_last_fragment = self.fragments.last().map(|f| f.is_empty()).unwrap_or(false);
        if drop_empty_last_fragment {
            _ = self.fragments.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::growth::growth_trait::GrowthWithConstantTimeAccess;
    use crate::test_all_growth_types;
    use crate::*;

    #[test]
    fn fragments() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..42 {
                vec.push(i);
            }

            let mut combined = vec![];
            let mut combined_mut = vec![];
            for fra in vec.fragments() {
                combined.extend_from_slice(fra);
            }
            for fra in unsafe { vec.fragments_mut() } {
                combined_mut.extend_from_slice(fra);
            }

            for i in 0..42 {
                assert_eq!(i, vec[i]);
                assert_eq!(i, combined[i]);
                assert_eq!(i, combined_mut[i]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn get_fragment_and_inner_indices() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..432 {
                vec.push(i);
                assert_eq!(None, vec.get_fragment_and_inner_indices(i + 1));
            }

            for i in 0..432 {
                let (f, ii) = vec.get_fragment_and_inner_indices(i).expect("is-some");
                assert_eq!(vec[i], vec.fragments[f][ii]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn get_ptr_mut() {
        fn test<G: GrowthWithConstantTimeAccess>(mut vec: SplitVec<usize, G>) {
            for i in 0..65 {
                vec.push(i);
            }
            for i in 0..64 {
                let p = unsafe { vec.get_ptr_mut(i) }.expect("is-some");
                assert_eq!(i, unsafe { *p });
            }
            for i in 64..vec.capacity() {
                let p = unsafe { vec.get_ptr_mut(i) };
                assert!(p.is_some());
            }

            for i in vec.capacity()..(vec.capacity() * 2) {
                let p = unsafe { vec.get_ptr_mut(i) };
                assert!(p.is_none());
            }
        }

        test(SplitVec::with_doubling_growth());
        test(SplitVec::with_linear_growth(6));
    }
}
