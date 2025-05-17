use crate::Fragment;
use alloc::{string::String, vec::Vec};
use orx_pseudo_default::PseudoDefault;

/// Growth strategy of a split vector.
pub trait Growth: Clone + PseudoDefault + Send + Sync {
    /// Given that the split vector has no fragments yet,
    /// returns the capacity of the first fragment.
    fn first_fragment_capacity(&self) -> usize {
        self.new_fragment_capacity_from([].into_iter())
    }

    /// Given that the split vector contains the given `fragments`,
    /// returns the capacity of the next fragment.
    #[inline(always)]
    fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
        self.new_fragment_capacity_from(fragments.iter().map(|x| x.capacity()))
    }

    /// Given that the split vector contains fragments with the given `fragment_capacities`,
    /// returns the capacity of the next fragment.
    fn new_fragment_capacity_from(
        &self,
        fragment_capacities: impl ExactSizeIterator<Item = usize>,
    ) -> usize;

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
    fn get_ptr<T>(&self, fragments: &[Fragment<T>], index: usize) -> Option<*const T> {
        self.get_ptr_and_indices(fragments, index).map(|x| x.0)
    }

    /// ***O(fragments.len())*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the  vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    fn get_ptr_mut<T>(&self, fragments: &mut [Fragment<T>], index: usize) -> Option<*mut T> {
        self.get_ptr_mut_and_indices(fragments, index).map(|x| x.0)
    }

    /// ***O(fragments.len())*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`
    /// together with the index of the fragment that the element belongs to
    /// and index of the element withing the respective fragment.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the  vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    fn get_ptr_and_indices<T>(
        &self,
        fragments: &[Fragment<T>],
        index: usize,
    ) -> Option<(*const T, usize, usize)> {
        let mut prev_cumulative_capacity = 0;
        let mut cumulative_capacity = 0;
        for (f, fragment) in fragments.iter().enumerate() {
            cumulative_capacity += fragment.capacity();
            if index < cumulative_capacity {
                let index_in_fragment = index - prev_cumulative_capacity;
                return Some((
                    unsafe { fragment.as_ptr().add(index_in_fragment) },
                    f,
                    index_in_fragment,
                ));
            }
            prev_cumulative_capacity = cumulative_capacity;
        }
        None
    }

    /// ***O(fragments.len())*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`
    /// together with the index of the fragment that the element belongs to
    /// and index of the element withing the respective fragment.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the  vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    fn get_ptr_mut_and_indices<T>(
        &self,
        fragments: &mut [Fragment<T>],
        index: usize,
    ) -> Option<(*mut T, usize, usize)> {
        let mut prev_cumulative_capacity = 0;
        let mut cumulative_capacity = 0;
        for (f, fragment) in fragments.iter_mut().enumerate() {
            cumulative_capacity += fragment.capacity();
            if index < cumulative_capacity {
                let index_in_fragment = index - prev_cumulative_capacity;
                return Some((
                    unsafe { fragment.as_mut_ptr().add(index_in_fragment) },
                    f,
                    index_in_fragment,
                ));
            }
            prev_cumulative_capacity = cumulative_capacity;
        }
        None
    }

    /// Returns the maximum number of elements that can safely be stored in a concurrent program.
    ///
    /// Note that pinned vectors already keep the elements pinned to their memory locations.
    /// Therefore, concurrently safe growth here corresponds to growth without requiring `fragments` collection to allocate.
    /// Recall that `fragments` contains meta information about the splits of the `SplitVec`, such as the capacity of each split.
    ///
    /// This default implementation is not the most efficient as it allocates a small vector to compute the capacity.
    /// However, it is almost always possible to provide a non-allocating implementation provided that the concurrency is relevant.
    /// `Doubling`, `Recursive` and `Linear` growth strategies introduced in this crate all override this method.
    ///
    /// # Panics
    ///
    /// Panics if `fragments.len() < fragments_capacity`, which must not hold.
    fn maximum_concurrent_capacity<T>(
        &self,
        fragments: &[Fragment<T>],
        fragments_capacity: usize,
    ) -> usize {
        assert!(fragments_capacity >= fragments.len());

        if fragments_capacity == fragments.len() {
            fragments.iter().map(|x| x.capacity()).sum()
        } else {
            let mut cloned: Vec<Fragment<T>> = Vec::with_capacity(fragments_capacity);
            for fragment in fragments {
                cloned.push(Vec::with_capacity(fragment.capacity()).into());
            }
            for _ in fragments.len()..fragments_capacity {
                let new_capacity = self.new_fragment_capacity(&cloned);
                let fragment = Vec::with_capacity(new_capacity).into();
                cloned.push(fragment);
            }
            cloned.iter().map(|x| x.capacity()).sum()
        }
    }

    /// Returns the number of fragments with this growth strategy in order to be able to reach a capacity of `maximum_capacity` of elements.
    /// Returns the error if it the growth strategy does not allow the required number of fragments.
    ///
    /// This method is relevant and useful for concurrent programs, which helps in avoiding the fragments to allocate.
    fn required_fragments_len<T>(
        &self,
        fragments: &[Fragment<T>],
        maximum_capacity: usize,
    ) -> Result<usize, String> {
        fn overflown_err() -> String {
            alloc::format!(
                "Maximum cumulative capacity that can be reached is {}.",
                usize::MAX
            )
        }

        let mut cloned: Vec<Fragment<T>> = Vec::new();
        for fragment in fragments {
            cloned.push(Vec::with_capacity(fragment.capacity()).into());
        }

        let mut num_fragments = cloned.len();
        let mut current_capacity: usize = cloned.iter().map(|x| x.capacity()).sum();

        while current_capacity < maximum_capacity {
            let new_capacity = self.new_fragment_capacity(&cloned);
            let (new_current_capacity, overflown) = current_capacity.overflowing_add(new_capacity);
            if overflown {
                return Err(overflown_err());
            }

            let fragment = Vec::with_capacity(new_capacity).into();
            cloned.push(fragment);

            current_capacity = new_current_capacity;
            num_fragments += 1;
        }

        Ok(num_fragments)
    }

    /// Returns the maximum possible bound on concurrent capacity.
    fn maximum_concurrent_capacity_bound<T>(&self, _: &[Fragment<T>], _: usize) -> usize {
        usize::MAX
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

    /// ***O(1)*** Returns a pointer to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the split vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    fn get_ptr<T>(&self, fragments: &[Fragment<T>], index: usize) -> Option<*const T> {
        let (f, i) = self.get_fragment_and_inner_indices_unchecked(index);
        fragments
            .get(f)
            .map(|fragment| unsafe { fragment.as_ptr().add(i) })
    }

    /// ***O(1)*** Returns a mutable reference to the `index`-th element of the split vector of the `fragments`.
    ///
    /// Returns `None` if `index`-th position does not belong to the split vector; i.e., if `index` is out of cumulative capacity of fragments.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the split vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    fn get_ptr_mut<T>(&self, fragments: &mut [Fragment<T>], index: usize) -> Option<*mut T> {
        let (f, i) = self.get_fragment_and_inner_indices_unchecked(index);
        fragments
            .get_mut(f)
            .map(|fragment| unsafe { fragment.as_mut_ptr().add(i) })
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
    fn get_ptr_mut_and_indices<T>(
        &self,
        fragments: &mut [Fragment<T>],
        index: usize,
    ) -> Option<(*mut T, usize, usize)> {
        let (f, i) = self.get_fragment_and_inner_indices_unchecked(index);
        fragments
            .get_mut(f)
            .map(|fragment| (unsafe { fragment.as_mut_ptr().add(i) }, f, i))
    }

    /// ***O(1)*** Returns the capacity of the fragment with the given `fragment_index`.
    fn fragment_capacity_of(&self, fragment_index: usize) -> usize;
}
