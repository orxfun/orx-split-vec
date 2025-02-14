use crate::common_traits::iterator::IterPtr;
use crate::common_traits::iterator::IterPtrBackward;
use crate::common_traits::iterator::IterSlices;
use crate::fragment::fragment_struct::set_fragments_len;
use crate::range_helpers::{range_end, range_start};
use crate::{algorithms, Fragment, Growth, SplitVec};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::RangeBounds;
use orx_iterable::Collection;
use orx_pinned_vec::utils::slice;
use orx_pinned_vec::{CapacityState, PinnedVec};
use orx_pseudo_default::PseudoDefault;

impl<T, G: Growth> PseudoDefault for SplitVec<T, G> {
    fn pseudo_default() -> Self {
        let growth = G::pseudo_default();
        let capacity = growth.first_fragment_capacity();
        let fragments = alloc::vec![Fragment::new(capacity)];
        Self::from_raw_parts(0, fragments, growth)
    }
}

impl<T, G: Growth> PinnedVec<T> for SplitVec<T, G> {
    type IterRev<'a>
        = crate::common_traits::iterator::IterRev<'a, T>
    where
        T: 'a,
        Self: 'a;

    type IterMutRev<'a>
        = crate::common_traits::iterator::IterMutRev<'a, T>
    where
        T: 'a,
        Self: 'a;

    type SliceIter<'a>
        = IterSlices<'a, T>
    where
        T: 'a,
        Self: 'a;

    type SliceMutIter<'a>
        = Vec<&'a mut [T]>
    where
        T: 'a,
        Self: 'a;

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

    /// Returns the index of the element of the vector that the `element_ptr` points to.
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
    fn index_of_ptr(&self, element_ptr: *const T) -> Option<usize> {
        // TODO! # examples in docs
        let mut count = 0;
        for fragment in &self.fragments {
            if let Some(index) = slice::index_of_ptr(&fragment.data, element_ptr) {
                return Some(count + index);
            } else {
                count += fragment.len()
            }
        }
        None
    }

    fn push_get_ptr(&mut self, value: T) -> *const T {
        self.len += 1;
        match self.has_capacity_for_one() {
            true => {
                let f = self.fragments.len() - 1;
                let fragment = &mut self.fragments[f];
                let idx = fragment.len();
                fragment.push(value);
                unsafe { fragment.as_ptr().add(idx) }
            }
            false => {
                //
                self.add_fragment_with_first_value(value);
                let f = self.fragments.len() - 1;
                self.fragments[f].as_ptr()
            }
        }
    }

    unsafe fn iter_ptr<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i,
    {
        IterPtr::from(self.fragments.as_slice())
    }

    unsafe fn iter_ptr_rev<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i,
    {
        IterPtrBackward::from(self.fragments.as_slice())
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
            .any(|fragment| slice::contains_reference(fragment.as_slice(), element))
    }

    /// Returns whether or not the element with the given pointer belongs to the vector.
    /// This method has *O(f)* time complexity where `f << vec.len()` is the number of fragments.
    ///
    /// Note that `T: Eq` is not required; memory address is used.
    ///
    fn contains_ptr(&self, element_ptr: *const T) -> bool {
        self.fragments
            .iter()
            .any(|fragment| slice::contains_ptr(fragment.as_slice(), element_ptr))
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
        match self.has_capacity_for_one() {
            true => {
                let last_f = self.fragments.len() - 1;
                self.fragments[last_f].push(value);
            }
            false => self.add_fragment_with_first_value(value),
        }
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
            .expect("first index is out-of-bounds");
        let (bf, bi) = self
            .get_fragment_and_inner_indices(b)
            .expect("second index out-of-bounds");
        if af == bf {
            self.fragments[af].swap(ai, bi);
        } else {
            let ptr_a = unsafe { self.fragments[af].as_mut_ptr().add(ai) };
            let ref_a = unsafe { &mut *ptr_a };
            let ref_b = &mut self.fragments[bf][bi];
            core::mem::swap(ref_a, ref_b);
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

    fn iter_rev(&self) -> Self::IterRev<'_> {
        Self::IterRev::new(&self.fragments)
    }

    fn iter_mut_rev(&mut self) -> Self::IterMutRev<'_> {
        Self::IterMutRev::new(&mut self.fragments)
    }

    /// Returns the view on the required `range` as a vector of slices:
    ///
    /// * returns an empty vector if the range is out of bounds;
    /// * returns a vector with one slice if the range completely belongs to one fragment (in this case `try_get_slice` would return Ok),
    /// * returns an ordered vector of slices when chained forms the required range.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(2);
    ///
    /// vec.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// assert_eq!(vec.fragments()[0], &[0, 1, 2, 3]);
    /// assert_eq!(vec.fragments()[1], &[4, 5, 6, 7]);
    /// assert_eq!(vec.fragments()[2], &[8, 9]);
    ///
    /// // single fragment
    /// assert_eq!(vec![&[0, 1, 2, 3]], vec.slices(0..4).collect::<Vec<_>>());
    /// assert_eq!(vec![&[5, 6]], vec.slices(5..7).collect::<Vec<_>>());
    /// assert_eq!(vec![&[8, 9]], vec.slices(8..10).collect::<Vec<_>>());
    ///
    /// // Fragmented
    /// assert_eq!(vec![&vec![3], &vec![4, 5]], vec.slices(3..6).collect::<Vec<_>>());
    /// assert_eq!(vec![&vec![3], &vec![4, 5, 6, 7], &vec![8]], vec.slices(3..9).collect::<Vec<_>>());
    /// assert_eq!(vec![&vec![7], &vec![8]], vec.slices(7..9).collect::<Vec<_>>());
    ///
    /// // OutOfBounds
    /// assert_eq!(vec.slices(5..12).len(), 0);
    /// assert_eq!(vec.slices(10..11).len(), 0);
    /// ```
    fn slices<R: RangeBounds<usize>>(&self, range: R) -> Self::SliceIter<'_> {
        Self::SliceIter::new(self, range)
    }

    /// Returns a mutable view on the required `range` as a vector of slices:
    ///
    /// * returns an empty vector if the range is out of bounds;
    /// * returns a vector with one slice if the range completely belongs to one fragment (in this case `try_get_slice` would return Ok),
    /// * returns an ordered vector of slices when chained forms the required range.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(2);
    /// vec.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// assert_eq!(vec.fragments()[0], &[0, 1, 2, 3]);
    /// assert_eq!(vec.fragments()[1], &[4, 5, 6, 7]);
    /// assert_eq!(vec.fragments()[2], &[8, 9]);
    ///
    /// // single fragment
    /// let mut slices = vec.slices_mut(0..4);
    /// assert_eq!(slices.len(), 1);
    /// assert_eq!(slices[0], &[0, 1, 2, 3]);
    /// slices[0][1] *= 10;
    /// assert_eq!(vec.fragments()[0], &[0, 10, 2, 3]);
    ///
    /// // single fragment - partially
    /// let mut slices = vec.slices_mut(5..7);
    /// assert_eq!(slices.len(), 1);
    /// assert_eq!(slices[0], &[5, 6]);
    /// slices[0][1] *= 10;
    /// assert_eq!(vec.fragments()[1], &[4, 5, 60, 7]);
    ///
    /// // multiple fragments
    /// let slices = vec.slices_mut(2..6);
    /// assert_eq!(slices.len(), 2);
    /// assert_eq!(slices[0], &[2, 3]);
    /// assert_eq!(slices[1], &[4, 5]);
    /// for s in slices {
    ///     for x in s {
    ///         *x *= 10;
    ///     }
    /// }
    ///
    /// assert_eq!(vec.fragments()[0], &[0, 10, 20, 30]);
    /// assert_eq!(vec.fragments()[1], &[40, 50, 60, 7]);
    /// assert_eq!(vec.fragments()[2], &[8, 9]);
    ///
    /// // out of bounds
    /// assert!(vec.slices_mut(5..12).is_empty());
    /// assert!(vec.slices_mut(10..11).is_empty());
    /// ```
    fn slices_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Self::SliceMutIter<'_> {
        use alloc::vec;
        use core::slice::from_raw_parts_mut;

        let a = range_start(&range);
        let b = range_end(&range, self.len());

        match b.saturating_sub(a) {
            0 => Vec::new(),
            _ => match self.get_fragment_and_inner_indices(a) {
                None => Vec::new(),
                Some((sf, si)) => match self.get_fragment_and_inner_indices(b - 1) {
                    None => Vec::new(),
                    Some((ef, ei)) => match sf.cmp(&ef) {
                        Ordering::Equal => vec![&mut self.fragments[sf][si..=ei]],
                        _ => {
                            let mut vec = Vec::with_capacity(ef - sf + 1);

                            let ptr_s = unsafe { self.fragments[sf].as_mut_ptr().add(si) };
                            let slice_len = self.fragments[sf].capacity() - si;
                            vec.push(unsafe { from_raw_parts_mut(ptr_s, slice_len) });
                            for f in sf + 1..ef {
                                let ptr_s = self.fragments[f].as_mut_ptr();
                                let slice_len = self.fragments[f].capacity();
                                vec.push(unsafe { from_raw_parts_mut(ptr_s, slice_len) });
                            }
                            vec.push(&mut self.fragments[ef][..=ei]);
                            vec
                        }
                    },
                },
            },
        }
    }

    /// Returns a pointer to the `index`-th element of the vector.
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
    fn get_ptr(&self, index: usize) -> Option<*const T> {
        self.growth_get_ptr(index)
    }

    /// Returns a mutable pointer to the `index`-th element of the vector.
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
    fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T> {
        self.growth_get_ptr_mut(index)
    }

    unsafe fn set_len(&mut self, new_len: usize) {
        set_fragments_len(&mut self.fragments, new_len);
        self.len = new_len;
    }

    fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        algorithms::binary_search::binary_search_by(&self.fragments, f)
    }

    fn sort(&mut self)
    where
        T: Ord,
    {
        algorithms::in_place_sort::in_place_sort_by(&mut self.fragments, T::cmp)
    }

    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        algorithms::in_place_sort::in_place_sort_by(&mut self.fragments, compare)
    }

    fn sort_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let compare = |a: &T, b: &T| f(a).cmp(&f(b));
        algorithms::in_place_sort::in_place_sort_by(&mut self.fragments, compare)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::macros::Num;
    use crate::test_all_growth_types;
    use crate::*;
    use alloc::string::String;
    use alloc::vec::Vec;
    use orx_pinned_vec::*;
    use orx_pseudo_default::PseudoDefault;

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
            let mut another_vec = Vec::new();
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
    fn slices() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..184 {
                assert_eq!(vec.slices(i..i + 1).len(), 0);
                assert_eq!(vec.slices(0..i + 1).len(), 0);
                vec.push(i);
            }

            let slice = vec.slices(0..vec.len());
            let mut combined = Vec::new();
            for s in slice {
                combined.extend_from_slice(s);
            }
            for i in 0..184 {
                assert_eq!(i, vec[i]);
                assert_eq!(i, combined[i]);
            }

            let begin = vec.len() / 4;
            let end = 3 * vec.len() / 4;
            let slice = vec.slices(begin..end);
            let mut combined = Vec::new();
            for s in slice {
                combined.extend_from_slice(s);
            }
            for i in begin..end {
                assert_eq!(i, vec[i]);
                assert_eq!(i, combined[i - begin]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn slices_mut() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..184 {
                assert!(vec.slices_mut(i..i + 1).is_empty());
                assert!(vec.slices_mut(0..i + 1).is_empty());
                vec.push(i);
            }

            let slice = vec.slices_mut(0..vec.len());
            let mut combined = Vec::new();
            for s in slice {
                combined.extend_from_slice(s);
            }
            for i in 0..184 {
                assert_eq!(i, vec[i]);
                assert_eq!(i, combined[i]);
            }

            let begin = vec.len() / 4;
            let end = 3 * vec.len() / 4;
            let slice = vec.slices_mut(begin..end);
            let mut combined = Vec::new();
            for s in slice {
                combined.extend_from_slice(s);
            }
            for i in begin..end {
                assert_eq!(i, vec[i]);
                assert_eq!(i, combined[i - begin]);
            }

            vec.clear();

            for _ in 0..184 {
                vec.push(0);
            }

            fn update(slice: Vec<&mut [usize]>, begin: usize) {
                let mut val = begin;
                for s in slice {
                    for x in s {
                        *x = val;
                        val += 1;
                    }
                }
            }
            let mut fill = |begin: usize, end: usize| {
                let range = begin..end;
                update(vec.slices_mut(range), begin);
            };

            fill(0, 14);
            fill(14, 56);
            fill(56, 77);
            fill(77, 149);
            fill(149, 182);
            fill(182, 184);
            for i in 0..184 {
                assert_eq!(vec.get(i), Some(&i));
            }
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
            assert!(vec.get_ptr_mut(i).is_none());
        }
    }

    #[test]
    fn pseudo_default() {
        let vec = SplitVec::<String, Doubling>::pseudo_default();
        assert_eq!(vec.len(), 0);

        let vec = SplitVec::<String, Recursive>::pseudo_default();
        assert_eq!(vec.len(), 0);

        let vec = SplitVec::<String, Linear>::pseudo_default();
        assert_eq!(vec.len(), 0);
    }
}
