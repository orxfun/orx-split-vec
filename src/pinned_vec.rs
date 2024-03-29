use crate::{Growth, SplitVec};
use orx_pinned_vec::utils::slice;
use orx_pinned_vec::{CapacityState, PinnedVec, PinnedVecGrowthError};

impl<T, G> PinnedVec<T> for SplitVec<T, G>
where
    G: Growth,
{
    type Iter<'a> = crate::common_traits::iterator::iter::Iter<'a, T> where T: 'a, Self: 'a;
    type IterMut<'a> = crate::common_traits::iterator::iter_mut::IterMut<'a, T> where T: 'a, Self: 'a;
    type IterRev<'a> = crate::common_traits::iterator::iter_rev::IterRev<'a, T> where T: 'a, Self: 'a;
    type IterMutRev<'a> = crate::common_traits::iterator::iter_mut_rev::IterMutRev<'a, T> where T: 'a, Self: 'a;

    /// Returns the index of the `element` with the given reference.
    /// This method has *O(f)* time complexity where `f << vec.len()` is the number of fragments.
    ///
    /// Note that `T: Eq` is not required; reference equality is used.
    ///
    /// # Safety
    ///
    /// Since `SplitVec` implements `PinnedVec`, the underlying memory
    /// of the vector stays pinned; i.e., is not carried to different memory
    /// locations.
    /// Therefore, it is possible and safe to compare an element's reference
    /// to find its position in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(2);
    /// for i in 0..4 {
    ///     vec.push(10 * i);
    /// }
    ///
    /// assert_eq!(Some(0), vec.index_of(&vec[0]));
    /// assert_eq!(Some(1), vec.index_of(&vec[1]));
    /// assert_eq!(Some(2), vec.index_of(&vec[2]));
    /// assert_eq!(Some(3), vec.index_of(&vec[3]));
    ///
    /// // the following does not compile since vec[4] is out of bounds
    /// // assert_eq!(Some(3), vec.index_of(&vec[4]));
    ///
    /// // num certainly does not belong to `vec`
    /// let num = 42;
    /// assert_eq!(None, vec.index_of(&num));
    ///
    /// // even if its value belongs
    /// let num = 20;
    /// assert_eq!(None, vec.index_of(&num));
    ///
    /// // as expected, querying elements of another vector will also fail
    /// let eq_vec = vec![0, 10, 20, 30];
    /// for i in 0..4 {
    ///     assert_eq!(None, vec.index_of(&eq_vec[i]));
    /// }
    /// ```
    fn index_of(&self, element: &T) -> Option<usize> {
        let mut count = 0;
        for fragment in &self.fragments {
            if let Some(index) = slice::index_of(&fragment.data, element) {
                return Some(count + index);
            } else {
                count += fragment.len()
            }
        }
        None
    }

    /// Returns whether or not the `element` with the given reference belongs to the vector.
    /// This method has *O(f)* time complexity where `f << vec.len()` is the number of fragments.
    ///
    /// Note that `T: Eq` is not required; memory address is used.
    ///
    /// # Safety
    ///
    /// Since `FixedVec` implements `PinnedVec`, the underlying memory
    /// of the vector stays pinned; i.e., is not carried to different memory
    /// locations.
    /// Therefore, it is possible and safe to compare an element's reference
    /// to find its position in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::new();
    /// for i in 0..4 {
    ///     vec.push(10 * i);
    /// }
    ///
    /// assert!(vec.contains_reference(&vec[0]));
    /// assert!(vec.contains_reference(&vec[1]));
    /// assert!(vec.contains_reference(&vec[2]));
    /// assert!(vec.contains_reference(&vec[3]));
    ///
    /// // num certainly does not belong to `vec`
    /// let num = 42;
    /// assert!(!vec.contains_reference(&num));
    ///
    /// // even if its value belongs
    /// let num = 20;
    /// assert!(!vec.contains_reference(&num));
    ///
    /// // as expected, querying elements of another vector will also fail
    /// let eq_vec = vec![0, 10, 20, 30];
    /// for i in 0..4 {
    ///     assert!(!vec.contains_reference(&eq_vec[i]));
    /// }
    /// ```
    fn contains_reference(&self, element: &T) -> bool {
        self.fragments
            .iter()
            .any(|fragment| slice::contains_reference(&fragment.data, element))
    }

    /// Returns the total number of elements the split vector can hold without
    /// reallocating.
    ///
    /// See `FragmentGrowth` for details of capacity growth policies.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// // default growth starting with 4, and doubling at each new fragment.
    /// let mut vec = SplitVec::with_doubling_growth();
    /// assert_eq!(4, vec.capacity());
    ///
    /// for i in 0..4 {
    ///     vec.push(i);
    /// }
    /// assert_eq!(4, vec.capacity());
    ///
    /// vec.push(4);
    /// assert_eq!(4 + 8, vec.capacity());
    ///
    /// ```
    fn capacity(&self) -> usize {
        self.fragments.iter().map(|f| f.capacity()).sum()
    }

    fn capacity_state(&self) -> CapacityState {
        CapacityState::DynamicCapacity {
            current_capacity: self.capacity(),
            maximum_concurrent_capacity: self.maximum_concurrent_capacity(),
        }
    }

    /// Clears the vector, removing all values.
    ///
    /// This method:
    /// * drops all fragments except for the first one, and
    /// * clears the first fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(5);
    /// for _ in 0..10 {
    ///     vec.push(4.2);
    /// }
    ///
    /// vec.clear();
    ///
    /// assert!(vec.is_empty());
    /// ```
    fn clear(&mut self) {
        if !self.fragments.is_empty() {
            self.fragments.truncate(1);
            self.fragments[0].clear();
        }
        self.len = 0;
    }

    /// Clones and appends all elements in a slice to the vec.
    ///
    /// Iterates over the slice `other`, clones each element, and then appends
    /// it to this vector. The `other` slice is traversed in-order.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend_from_slice(&[4, 5, 6, 7]);
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.len += other.len();
        let mut slice = other;
        while !slice.is_empty() {
            if !self.has_capacity_for_one() {
                self.add_fragment();
            }
            let f = self.fragments.len() - 1;

            let last = &mut self.fragments[f];
            let available = last.room();

            if available < slice.len() {
                last.extend_from_slice(&slice[0..available]);
                slice = &slice[available..];
                self.add_fragment();
            } else {
                last.extend_from_slice(slice);
                break;
            }
        }
    }

    /// Returns a reference to the element with the given `index`;
    /// None if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(5);
    /// vec.extend_from_slice(&[10, 40, 30]);
    /// assert_eq!(Some(&40), vec.get(1));
    /// assert_eq!(None, vec.get(3));
    /// ```
    fn get(&self, index: usize) -> Option<&T> {
        self.get_fragment_and_inner_indices(index)
            .map(|(f, i)| unsafe { self.fragments.get_unchecked(f).get_unchecked(i) })
    }

    /// Returns a mutable reference to the element with the given `index`;
    /// None if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(5);
    /// vec.extend_from_slice(&[0, 1, 2]);
    ///
    /// if let Some(elem) = vec.get_mut(1) {
    ///     *elem = 42;
    /// }
    ///
    /// assert_eq!(vec, &[0, 42, 2]);
    /// ```
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_fragment_and_inner_indices(index)
            .map(|(f, i)| unsafe { self.fragments.get_unchecked_mut(f).get_unchecked_mut(i) })
    }

    /// Returns a reference to an element or sub-slice, without doing bounds checking.
    ///
    /// For a safe alternative see `get`.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.get(index).expect("out-of-bounds")
    }

    /// Returns a mutable reference to an element or sub-slice, without doing bounds checking.
    ///
    /// For a safe alternative see `get_mut`.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("out-of-bounds")
    }

    /// Returns a reference to the first element of the vector; returns None if the vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::new();
    /// assert!(vec.first().is_none());
    ///
    /// vec.push(42);
    /// assert_eq!(Some(&42), vec.first());
    ///
    /// vec.push(864121);
    /// assert_eq!(Some(&42), vec.first());
    ///
    /// vec.insert(0, 7);
    /// assert_eq!(Some(&7), vec.first());
    /// ```
    #[inline(always)]
    fn first(&self) -> Option<&T> {
        self.fragments.first().and_then(|x| x.first())
    }

    /// Returns a reference to the last element of the vector; returns None if the vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::new();
    /// assert!(vec.last().is_none());
    ///
    /// vec.push(42);
    /// assert_eq!(Some(&42), vec.last());
    ///
    /// vec.push(7);
    /// assert_eq!(Some(&7), vec.last());
    ///
    /// vec.insert(0, 684321);
    /// assert_eq!(Some(&7), vec.last());
    /// ```
    #[inline(always)]
    fn last(&self) -> Option<&T> {
        self.fragments.last().and_then(|x| x.last())
    }

    #[inline(always)]
    unsafe fn first_unchecked(&self) -> &T {
        self.fragments.get_unchecked(0).get_unchecked(0)
    }

    #[inline(always)]
    unsafe fn last_unchecked(&self) -> &T {
        let fragment = self.fragments.get_unchecked(self.fragments.len() - 1);
        fragment.get_unchecked(fragment.len() - 1)
    }

    fn insert(&mut self, index: usize, value: T) {
        if index == self.len {
            self.push(value);
        } else {
            // make room for one
            if !self.has_capacity_for_one() {
                self.add_fragment();
            }

            let (f, i) = self
                .get_fragment_and_inner_indices(index)
                .expect("out-of-bounds");

            if self.fragments[f].has_capacity_for_one() {
                self.fragments[f].insert(i, value);
            } else {
                let mut popped = self.fragments[f].pop().expect("no-way!");
                self.fragments[f].insert(i, value);
                let mut f = f;
                loop {
                    f += 1;

                    if self.fragments[f].has_capacity_for_one() {
                        self.fragments[f].insert(0, popped);
                        break;
                    } else {
                        let new_popped = self.fragments[f].pop().expect("no-way");
                        self.fragments[f].insert(0, popped);
                        popped = new_popped;
                    }
                }
            }
            self.len += 1;
        }
    }

    /// Returns `true` if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(2);
    /// assert!(vec.is_empty());
    /// vec.push(1);
    /// assert!(!vec.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the vector, also referred to
    /// as its 'length'.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec =  SplitVec::with_linear_growth(8);
    /// assert_eq!(0, vec.len());
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(3, vec.len());
    /// ```
    fn len(&self) -> usize {
        self.len
    }

    fn pop(&mut self) -> Option<T> {
        if self.fragments.is_empty() {
            None
        } else {
            let f = self.fragments.len() - 1;
            if self.fragments[f].is_empty() {
                if f == 0 {
                    None
                } else {
                    self.len -= 1;
                    self.fragments.pop();
                    self.fragments[f - 1].pop()
                }
            } else {
                self.len -= 1;
                let popped = self.fragments[f].pop();
                if self.fragments[f].is_empty() {
                    self.fragments.pop();
                }
                popped
            }
        }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(16);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    fn push(&mut self, value: T) {
        self.len += 1;
        if self.has_capacity_for_one() {
            let last_f = self.fragments.len() - 1;
            self.fragments[last_f].push(value);
            return;
        }

        self.add_fragment_with_first_value(value);
    }

    fn remove(&mut self, index: usize) -> T {
        self.drop_last_empty_fragment();

        let (f, i) = self
            .get_fragment_and_inner_indices(index)
            .expect("out-of-bounds");

        let value = self.fragments[f].remove(i);

        for f2 in f + 1..self.fragments.len() {
            let x = self.fragments[f2].remove(0);
            self.fragments[f2 - 1].push(x);
            if self.fragments[f2].is_empty() {
                self.fragments.remove(f2);
                break;
            }
        }

        self.drop_last_empty_fragment();

        self.len -= 1;
        value
    }

    fn swap(&mut self, a: usize, b: usize) {
        let (af, ai) = self
            .get_fragment_and_inner_indices(a)
            .expect("out-of-bounds");
        let (bf, bi) = self
            .get_fragment_and_inner_indices(b)
            .expect("out-of-bounds");
        if af == bf {
            self.fragments[af].swap(ai, bi);
        } else {
            let ptr_a = unsafe { self.fragments[af].as_mut_ptr().add(ai) };
            let ref_a = unsafe { &mut *ptr_a };
            let ref_b = &mut self.fragments[bf][bi];
            std::mem::swap(ref_a, ref_b);
        }
    }

    fn truncate(&mut self, len: usize) {
        if let Some((f, i)) = self.get_fragment_and_inner_indices(len) {
            self.fragments.truncate(f + 1);
            self.fragments[f].truncate(i);
            self.len = len;

            self.drop_last_empty_fragment();
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        Self::Iter::new(&self.fragments)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Self::IterMut::new(&mut self.fragments)
    }

    fn iter_rev(&self) -> Self::IterRev<'_> {
        Self::IterRev::new(&self.fragments)
    }

    fn iter_mut_rev(&mut self) -> Self::IterMutRev<'_> {
        Self::IterMutRev::new(&mut self.fragments)
    }

    /// Returns a mutable reference to the `index`-th element of the vector.
    ///
    /// Returns `None` if `index`-th position does not belong to the vector; i.e., if `index` is out of `capacity`.
    ///
    /// Time complexity of the method is:
    /// * ***O(1)*** when `G: GrowthWithConstantTimeAccess`,
    /// * ***O(f)*** for the general case `G: Growth` where `f` is the number of fragments in the split vector.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    ///
    #[inline(always)]
    unsafe fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T> {
        self.growth_get_ptr_mut(index)
    }

    unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;

        let mut remaining = new_len;

        for fragment in &mut self.fragments {
            let capacity = fragment.capacity();
            if remaining <= capacity {
                if fragment.len() != remaining {
                    unsafe { fragment.set_len(remaining) };
                }
            } else {
                if fragment.len() != capacity {
                    unsafe { fragment.set_len(capacity) };
                }
                remaining -= capacity;
            }
        }
    }

    fn try_grow(&mut self) -> Result<usize, PinnedVecGrowthError> {
        if self.len() < self.capacity() {
            Err(PinnedVecGrowthError::CanOnlyGrowWhenVecIsAtCapacity)
        } else {
            self.add_fragment();
            Ok(self.capacity())
        }
    }

    unsafe fn grow_to(&mut self, new_capacity: usize) -> Result<usize, PinnedVecGrowthError> {
        if new_capacity <= self.capacity() {
            Ok(self.capacity())
        } else {
            let mut current_capacity = self.capacity();
            while new_capacity > current_capacity {
                let new_fragment_capacity = self.add_fragment_get_fragment_capacity();
                current_capacity += new_fragment_capacity;
            }
            debug_assert_eq!(current_capacity, self.capacity());
            Ok(current_capacity)
        }
    }

    unsafe fn concurrently_grow_to(
        &mut self,
        new_capacity: usize,
    ) -> Result<usize, PinnedVecGrowthError> {
        if new_capacity <= self.capacity() {
            Ok(self.capacity())
        } else {
            let mut current_capacity = self.capacity();
            while new_capacity > current_capacity {
                if !self.can_concurrently_add_fragment() {
                    return Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned);
                }
                let new_fragment_capacity = self.add_fragment_get_fragment_capacity();
                current_capacity += new_fragment_capacity;
            }
            debug_assert_eq!(current_capacity, self.capacity());
            Ok(current_capacity)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::macros::Num;
    use crate::test_all_growth_types;
    use crate::*;
    use orx_pinned_vec::*;

    #[test]
    fn pinned_vec_tests() {
        fn test<G: Growth>(vec: SplitVec<usize, G>) {
            for cap in [0, 10, 124, 5421] {
                test_pinned_vec(vec.clone(), cap);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn index_of_and_contains() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let mut another_vec = vec![];
            for i in 0..157 {
                vec.push(i);
                another_vec.push(i);
            }
            for i in 0..vec.len() {
                assert_eq!(Some(i), vec.index_of(&vec[i]));
                assert!(vec.contains_reference(&vec[i]));

                assert_eq!(None, vec.index_of(&another_vec[i]));
                assert!(!vec.contains_reference(&another_vec[i]));

                let scalar = another_vec[i];
                assert_eq!(None, vec.index_of(&scalar));
                assert!(!vec.contains_reference(&scalar));
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn capacity_state() {
        fn test<G: Growth>(vec: SplitVec<usize, G>) {
            match vec.capacity_state() {
                CapacityState::DynamicCapacity {
                    current_capacity,
                    maximum_concurrent_capacity,
                } => {
                    assert!(maximum_concurrent_capacity >= current_capacity);
                    assert_eq!(current_capacity, vec.capacity());
                    assert_eq!(
                        maximum_concurrent_capacity,
                        vec.growth()
                            .maximum_concurrent_capacity(vec.fragments(), vec.fragments.capacity())
                    );
                }
                #[allow(clippy::panic)]
                _ => panic!("must have dynamic capacity"),
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn len_and_is_empty() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..42 {
                assert_eq!(i, vec.len());
                vec.push(i);
            }
            assert_eq!(42, vec.len());

            vec.clear();
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            assert_eq!(42, vec.len());

            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.pop();
            }
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.remove(vec.len() / 2);
            }
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.remove(0);
            }
            assert_eq!(0, vec.len());

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                assert_eq!(42 - i, vec.len());
                vec.remove(vec.len() - 1);
            }
            assert_eq!(0, vec.len());

            vec.clear();
            for i in 0..42 {
                assert_eq!(i, vec.len());
                vec.insert(i, i);
                assert_eq!(i + 1, vec.len());
            }
            assert_eq!(42, vec.len());

            vec.clear();
            for i in 0..42 {
                assert_eq!(i, vec.len());
                vec.insert(0, i);
            }
            assert_eq!(42, vec.len());
        }

        test_all_growth_types!(test);
    }

    #[test]
    fn clear() {
        fn clear_is_empty<G: Growth>(mut vec: SplitVec<usize, G>) {
            vec.clear();
            assert!(vec.is_empty());
            assert_eq!(0, vec.len());

            vec.push(1);
            assert!(!vec.is_empty());
            for i in 0..42 {
                vec.push(i);
            }
            assert!(!vec.is_empty());

            vec.clear();
            assert!(vec.is_empty());
            assert_eq!(0, vec.len());
        }
        test_all_growth_types!(clear_is_empty);
    }

    #[test]
    fn get() {
        fn test_get<G: Growth>(mut vec: SplitVec<usize, G>) {
            assert!(vec.is_empty());

            for i in 0..53 {
                vec.push(i);

                assert_eq!(vec.get(i), Some(&i));
                assert_eq!(vec.get(i + 1), None);

                *vec.get_mut(i).expect("is-some") += 100;
            }

            for i in 0..53 {
                assert_eq!(vec.get(i), Some(&(100 + i)));
            }
        }
        test_all_growth_types!(test_get);
    }

    #[test]
    fn first_last() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            assert!(vec.first().is_none());
            assert!(vec.last().is_none());

            vec.push(42);

            assert_eq!(vec.first(), Some(&42));
            assert_eq!(vec.last(), Some(&42));

            unsafe {
                assert_eq!(vec.first_unchecked(), &42);
                assert_eq!(vec.last_unchecked(), &42);
            }

            vec.push(7);

            assert_eq!(vec.first(), Some(&42));
            assert_eq!(vec.last(), Some(&7));

            unsafe {
                assert_eq!(vec.first_unchecked(), &42);
                assert_eq!(vec.last_unchecked(), &7);
            }

            for _ in 0..800 {
                vec.insert(1, 56421);
            }

            assert_eq!(vec.first(), Some(&42));
            assert_eq!(vec.last(), Some(&7));

            unsafe {
                assert_eq!(vec.first_unchecked(), &42);
                assert_eq!(vec.last_unchecked(), &7);
            }

            vec.clear();

            assert!(vec.first().is_none());
            assert!(vec.last().is_none());
        }

        test_all_growth_types!(test);
    }

    #[test]
    fn extend_from_slice() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            vec.extend_from_slice(&(42..63).collect::<Vec<_>>());
            vec.extend_from_slice(&(63..100).collect::<Vec<_>>());

            assert_eq!(100, vec.len());
            for i in 0..100 {
                assert_eq!(i, vec[i]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn grow() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..42 {
                vec.push(i);
            }
            for i in 0..42 {
                vec.insert(i, 100 + i);
            }

            for i in 0..42 {
                assert_eq!(i, vec[42 + i]);
                assert_eq!(100 + i, vec[i]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn shrink() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..42 {
                vec.push(i);
                assert_eq!(i, vec.remove(0));
                assert!(vec.is_empty());
            }

            for i in 0..42 {
                vec.push(i);
            }
            for i in 0..42 {
                assert_eq!(i, vec.remove(0));
            }
            assert!(vec.is_empty());

            for i in 0..42 {
                vec.push(i);
            }
            for _ in 0..42 {
                vec.remove(vec.len() / 2);
            }
            assert!(vec.is_empty());
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn swap() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..42 {
                vec.push(i);
            }

            for i in 0..21 {
                vec.swap(i, 21 + i);
            }

            for i in 0..21 {
                assert_eq!(21 + i, vec[i]);
            }
            for i in 21..42 {
                assert_eq!(i - 21, vec[i]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn truncate() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let std_vec: Vec<_> = (0..42).collect();
            for i in 0..42 {
                vec.push(i);
            }

            vec.truncate(100);
            assert_eq!(vec, std_vec);

            for i in (0..42).rev() {
                vec.truncate(i);
                assert_eq!(vec, &std_vec[0..i]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn insert() {
        fn test<G: Growth>(mut vec: SplitVec<Num, G>) {
            for i in 0..42 {
                vec.push(Num::new(i));
            }
            for i in 0..42 {
                vec.insert(i, Num::new(100 + i));
            }

            for i in 0..42 {
                assert_eq!(Some(&Num::new(i)), vec.get(42 + i));
                assert_eq!(Some(&Num::new(100 + i)), vec.get(i));
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn clone() {
        fn test<G: Growth>(mut vec: SplitVec<Num, G>) {
            assert!(vec.is_empty());

            for i in 0..53 {
                vec.push(Num::new(i));
            }

            let clone = vec.clone();
            assert_eq!(vec, clone);
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn set_len_get_ptr_mut() {
        let mut vec = SplitVec::with_doubling_growth();
        vec.push(0);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        assert_eq!(vec.capacity(), 12);

        for i in vec.len()..vec.capacity() {
            unsafe { *(vec.get_ptr_mut(i).expect("is-some")) = i };
            unsafe { vec.set_len(i + 1) };

            assert_eq!(vec.get(i), Some(&i));
        }

        for i in vec.capacity()..(vec.capacity() + 100) {
            assert!(unsafe { vec.get_ptr_mut(i) }.is_none());
        }
    }

    #[test]
    fn try_grow() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            fn grow_one_fragment<G: Growth>(vec: &mut SplitVec<usize, G>) {
                let old_len = vec.len();
                let old_capacity = vec.capacity();
                assert!(old_len < old_capacity);

                for i in old_len..old_capacity {
                    assert_eq!(
                        Err(PinnedVecGrowthError::CanOnlyGrowWhenVecIsAtCapacity),
                        vec.try_grow()
                    );
                    vec.push(i);
                }
                assert_eq!(vec.capacity(), old_capacity);

                let result = vec.try_grow();
                assert!(result.is_ok());
                let new_capacity = result.expect("is-ok");
                assert!(new_capacity > old_capacity);
            }

            for _ in 0..5 {
                grow_one_fragment(&mut vec);
            }

            assert_eq!(5 + 1, vec.fragments().len());
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn grow_to_under_capacity() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for _ in 0..10 {
                assert_eq!(Ok(vec.capacity()), unsafe { vec.grow_to(0) });
                assert_eq!(Ok(vec.capacity()), unsafe {
                    vec.grow_to(vec.capacity() - 1)
                });
                assert_eq!(Ok(vec.capacity()), unsafe { vec.grow_to(vec.capacity()) });
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn grow_to() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for _ in 0..10 {
                let expected_capacity =
                    vec.capacity() + vec.growth().new_fragment_capacity(vec.fragments());
                let expected_num_fragments = vec.fragments().len() + 1;

                assert_eq!(Ok(expected_capacity), unsafe {
                    vec.grow_to(vec.capacity() + 1)
                });

                assert_eq!(vec.fragments().len(), expected_num_fragments);
                assert_eq!(vec.capacity(), expected_capacity);
            }

            vec.clear();

            for _ in 0..10 {
                let prev_num_fragments = vec.fragments().len();

                let new_capacity = unsafe { vec.grow_to(vec.capacity() + 1000) }.expect("is-okay");

                assert!(vec.fragments().len() >= prev_num_fragments);
                assert_eq!(vec.capacity(), new_capacity);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn concurrently_grow_to() {
        fn test_succeed<G: Growth>(mut vec: SplitVec<usize, G>) {
            let max_con_cap = vec.capacity_state().maximum_concurrent_capacity();

            assert_eq!(Ok(max_con_cap), unsafe {
                vec.concurrently_grow_to(max_con_cap)
            });
        }

        fn test_fail<G: Growth>(mut vec: SplitVec<usize, G>) {
            let max_con_cap = vec.capacity_state().maximum_concurrent_capacity();

            assert_eq!(
                Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned),
                unsafe { vec.concurrently_grow_to(max_con_cap + 1) }
            );
        }

        test_all_growth_types!(test_succeed);
        test_all_growth_types!(test_fail);
    }
}
