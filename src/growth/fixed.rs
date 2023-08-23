use crate::{Fragment, SplitVec, SplitVecGrowth};

/// A 'no-growth' strategy which does not allow the vector
/// to grow at all.
///
/// The vector must be created with sufficient capacity.
/// Exceeding this fixed capacity will lead to a panic.
///
/// The benefit of this strategy, on the other hand,
/// is its faster access by index operations;
/// which must be inlined and have a comparable performance
/// with regular slice access by index.
///
/// Further, the pinned-memory-location of already
/// pushed elements feature is maintained.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FixedCapacity;

impl<T> SplitVecGrowth<T> for FixedCapacity {
    /// Given that the split vector contains the given `fragments`,
    /// returns the capacity of the next fragment.
    ///
    /// However, the split vector with fixed capacity policy cannot grow;
    /// and hence this call panics.
    ///
    /// # Panics
    ///
    /// Panics regardless of the `fragments`. Since the split vector with
    /// fixed capacity policy cannot grow, the split vector will always
    /// contain a single fragment.
    #[allow(clippy::panic)]
    #[inline(always)]
    fn new_fragment_capacity(&self, fragments: &[Fragment<T>]) -> usize {
        debug_assert_eq!(1, fragments.len());
        panic!("a split vector with fixed capacity policy cannot grow.");
    }

    #[inline(always)]
    /// Returns the location of the element with the given `element_index` on the split vector
    /// as a tuple of (fragment-index, index-within-fragment).
    ///
    /// Returns None if the element index is out of bounds.
    ///
    /// Since a `SplitVec` with fixed capacity policy will always have a single fragment,
    /// this method will return:
    /// * `Some((0, element_index))` if `element_index` is in bounds,
    /// * `None` otherwise.
    fn get_fragment_and_inner_indices(
        &self,
        fragments: &[Fragment<T>],
        element_index: usize,
    ) -> Option<(usize, usize)> {
        debug_assert_eq!(1, fragments.len());
        if element_index < fragments[0].len() {
            Some((0, element_index))
        } else {
            None
        }
    }
}

impl<T> SplitVec<T, FixedCapacity> {
    /// Creates a split vector with the given `fixed_capacity`.
    ///
    /// This capacity is the hard limit and the vector cannot grow beyond it.
    /// Attempts to exceed this limit will lead to the code to panic.
    ///
    /// The benefit of this strategy, on the other hand,
    /// is its faster access by index operations;
    /// which must be inlined and have a comparable performance
    /// with regular slice access by index.
    ///
    /// Further, the pinned-memory-location of already
    /// pushed elements feature is maintained.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// // SplitVec<usize, FixedCapacity>
    /// let mut vec = SplitVec::with_fixed_capacity(4);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(4), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // push 4 elements to fill the vector completely
    /// for i in 0..4 {
    ///     vec.push(i);
    /// }
    /// assert_eq!(1, vec.fragments().len());
    ///
    /// // the next push exceeding the fixed_capacity will panic.
    /// // vec.push(4);
    /// ```
    pub fn with_fixed_capacity(fixed_capacity: usize) -> Self {
        Self {
            fragments: vec![Fragment::new(fixed_capacity)],
            growth: FixedCapacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FixedCapacity, Fragment, SplitVec, SplitVecGrowth};

    #[test]
    #[should_panic]
    fn new_cap() {
        fn new_fra() -> Fragment<usize> {
            Vec::<usize>::with_capacity(10).into()
        }

        let growth = FixedCapacity;
        let _ = growth.new_fragment_capacity(&[new_fra()]);
    }

    #[test]
    #[should_panic]
    fn indices_panics_when_fragments_is_empty() {
        assert_eq!(
            None,
            <FixedCapacity as SplitVecGrowth<usize>>::get_fragment_and_inner_indices(
                &FixedCapacity,
                &[],
                0
            )
        );
    }

    #[test]
    fn indices() {
        fn new_full() -> Fragment<usize> {
            (0..10).collect::<Vec<_>>().into()
        }

        let growth = FixedCapacity;

        for i in 0..10 {
            assert_eq!(
                Some((0, i)),
                growth.get_fragment_and_inner_indices(&[new_full()], i)
            );
        }
        assert_eq!(
            None,
            growth.get_fragment_and_inner_indices(&[new_full()], 10)
        );
    }

    #[test]
    fn fixed_capacity_vec() {
        let mut vec = SplitVec::with_fixed_capacity(77);

        for i in 0..vec.capacity() {
            assert_eq!(i, vec.len());
            vec.push(i);
            assert_eq!(1, vec.fragments().len());
            assert_eq!(77, vec.capacity());
            assert_eq!(77, vec.fragments()[0].capacity());
        }
    }

    #[test]
    #[should_panic]
    fn exceeding_fixed_capacity_panics() {
        let mut vec = SplitVec::with_fixed_capacity(77);
        for i in 0..vec.capacity() {
            vec.push(i);
        }
        vec.push(42); // panics!
    }
}
