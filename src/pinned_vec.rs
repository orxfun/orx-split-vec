use crate::{eq::are_fragments_eq_to_slice, Growth, SplitVec};
use orx_pinned_vec::utils::slice;
use orx_pinned_vec::PinnedVec;
use std::fmt::{Debug, Formatter, Result};

impl<T, G> PinnedVec<T> for SplitVec<T, G>
where
    G: Growth,
{
    type Iter<'a> = crate::common_traits::iterator::iter::Iter<'a, T> where T: 'a, Self: 'a;
    type IterMut<'a> = crate::common_traits::iterator::iter_mut::IterMut<'a, T> where T: 'a, Self: 'a;

    /// Returns the index of the `element` with the given reference.
    /// This method has *O(f)* time complexity where f is the number of fragments.
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
    /// use orx_split_vec::prelude::*;
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

    /// Returns the total number of elements the split vector can hold without
    /// reallocating.
    ///
    /// See `FragmentGrowth` for details of capacity growth policies.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
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

    /// Clears the vector, removing all values.
    ///
    /// This method:
    /// * drops all fragments except for the first one, and
    /// * clears the first fragment.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
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
    /// use orx_split_vec::prelude::*;
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
    /// use orx_split_vec::prelude::*;
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
    /// use orx_split_vec::prelude::*;
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
    /// Returns a reference to an element or subslice, without doing bounds checking.
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
    /// Returns a mutable reference to an element or subslice, without doing bounds checking.
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

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(16);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    ///
    /// vec.insert(1, 4);
    /// assert_eq!(vec, [1, 4, 2, 3]);
    ///
    /// vec.insert(4, 5);
    /// assert_eq!(vec, [1, 4, 2, 3, 5]);
    /// ```
    ///
    /// # Safety
    ///
    /// If the element type is not a `NotSelfRefVecItem`;
    /// in other words, if the elements hold references of each other,
    /// `insert` method might invalidate the references.
    /// This method is then called `unsafe_insert`.
    ///
    /// Consider the following struct which is not a `NotSelfRefVecItem`:
    /// ```
    /// struct Node<'a, T> {
    ///     value: T,
    ///     related_to: Option<&'a Node<'a, T>>,
    /// }
    /// ```
    ///
    /// Further, assume we build a vector of two nodes `[x, y]`,
    /// where each node is related to the other `x <--> y`.
    ///
    /// If we insert another node `w` at index 0, the vector takes the form `[w, x, y]`
    /// causing the following problems:
    ///
    /// * `x` is related to the node at position 1, which is itself: `x -> x`.
    /// * `y` is realted to the node at position 0, which is `w`: `y -> w`.
    ///
    /// Both relations are wrong after insertion.
    ///
    /// For this reason, insertions when `T` is a self-referencing-vector-item
    /// are **unsafe** and the caller is responsible for correcting the references
    /// if it needs to use this method.
    unsafe fn unsafe_insert(&mut self, index: usize, value: T) {
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
    /// use orx_split_vec::prelude::*;
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
    /// use orx_split_vec::prelude::*;
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

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(16);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    ///
    /// assert_eq!(vec.pop(), Some(3));
    /// assert_eq!(vec, [1, 2]);
    /// ```
    ///
    /// # Safety
    ///
    /// If the element type is not a `NotSelfRefVecItem`;
    /// in other words, if the elements hold references of each other,
    /// `pop` method might invalidate the references.
    /// This method is then called `unsafe_pop`.
    ///
    /// Consider the following struct which is not a `NotSelfRefVecItem`:
    /// ```
    /// struct Node<'a, T> {
    ///     value: T,
    ///     related_to: Option<&'a Node<'a, T>>,
    /// }
    /// ```
    ///
    /// Further, assume we build a vector of two nodes `[x, y]`,
    /// where each node is related to the other `x <--> y`.
    ///
    /// If we pop `y` from the vector, leaving the vector as `[x]`:
    ///
    /// * `y` still correctly points to the node at position 0, which is `x`.
    /// * However, `y` points to the node at position 1 which is does not belong to
    /// the vector now causing an undefined behavior.
    ///
    /// For this reason, popping when `T` is a self-referencing-vector-item
    /// is **unsafe** and the caller is responsible for correcting the references
    /// if it needs to use this method.
    unsafe fn unsafe_pop(&mut self) -> Option<T> {
        if self.fragments.is_empty() {
            None
        } else {
            let f = self.fragments.len() - 1;
            if self.fragments[f].len() == 0 {
                if f == 0 {
                    None
                } else {
                    self.len -= 1;
                    self.fragments.pop();
                    self.fragments[f - 1].pop()
                }
            } else {
                self.len -= 1;
                self.fragments[f].pop()
            }
        }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
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

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(*n*).
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(16);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// vec.push(4);
    /// vec.push(5);
    ///
    /// assert_eq!(vec.remove(1), 2);
    /// assert_eq!(vec, [1, 3, 4, 5]);
    ///
    /// assert_eq!(vec.remove(2), 4);
    /// assert_eq!(vec, [1, 3, 5]);
    /// ```
    ///
    /// # Safety
    ///
    /// If the element type is not a `NotSelfRefVecItem`;
    /// in other words, if the elements hold references of each other,
    /// `remove` method might invalidate the references.
    /// This method is then called `unsafe_remove`.
    ///
    /// Consider the following struct which is not a `NotSelfRefVecItem`:
    /// ```
    /// struct Node<'a, T> {
    ///     value: T,
    ///     related_to: Option<&'a Node<'a, T>>,
    /// }
    /// ```
    ///
    /// Further, assume we build a vector of two nodes `[x, y]`,
    /// where each node is related to the other `x <--> y`.
    ///
    /// If we remove the element at position 0; i.e., `x` from the vector,
    /// leaving the vector as `[y]`:
    ///
    /// * `y` points to the node at position 0, which is itself: `y -> y`.
    /// * Furthermore, `x` points to the node at position 1 which is does not belong to
    /// the vector now causing an undefined behavior.
    ///
    /// For this reason, removals when `T` is a self-referencing-vector-item
    /// are **unsafe** and the caller is responsible for correcting the references
    /// if it needs to use this method.
    unsafe fn unsafe_remove(&mut self, index: usize) -> T {
        let drop_empty_last_fragment = self.fragments.last().map(|f| f.is_empty()).unwrap_or(false);
        if drop_empty_last_fragment {
            self.fragments.pop();
        }

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

        self.len -= 1;
        value
    }
    /// Swaps the two elements of the vector with at the given positions 'a' and 'b'.
    ///
    /// # Panics
    ///
    /// Panics if either of the indices `a` or 'b' is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut vec = SplitVec::with_linear_growth(2);
    /// vec.push(1);    // fragment 0
    /// vec.push(2);    // fragment 0
    /// vec.push(10);   // fragment 1
    /// vec.push(20);   // fragment 1
    /// vec.push(100);  // fragment 2
    /// assert_eq!(vec, [1, 2, 10, 20, 100]);
    ///
    /// // this is a regular vec.swap
    /// vec.swap(0, 1);
    /// assert_eq!(vec, [2, 1, 10, 20, 100]);
    ///
    /// // this is inter-fragments swap; i.e., mem:swap
    /// vec.swap(1, 4);
    /// assert_eq!(vec, [2, 100, 10, 20, 1]);
    /// ```
    ///
    /// # Safety
    ///
    /// If the element type is not a `NotSelfRefVecItem`;
    /// in other words, if the elements hold references of each other,
    /// `swap` method might invalidate the references.
    /// This method is then called `unsafe_remove`.
    ///
    /// Consider the following struct which is not a `NotSelfRefVecItem`:
    /// ```
    /// struct Node<'a, T> {
    ///     value: T,
    ///     related_to: Option<&'a Node<'a, T>>,
    /// }
    /// ```
    ///
    /// Further, assume we build a vector of two nodes `[x, y, z]`,
    /// where each node is related to the next one: `x --> y --> z`.
    ///
    /// If we swap elements at positions 1 and 2, the vector becomes `[x, z, y]`.
    /// But now,
    ///
    /// * `x` still points to position 1 which is now occupied by `z`;
    /// * `y` still points to position 2 which is now occupied by itself.
    ///
    /// This does not cause an undefined behavior in the classical sense;
    /// however, the meaning of the vector and relations are broken.
    ///
    /// For this reason, swaps when `T` is a self-referencing-vector-item
    /// are **unsafe** and the caller is responsible for correcting the references
    /// if it needs to use this method.
    unsafe fn unsafe_swap(&mut self, a: usize, b: usize) {
        let (af, ai) = self
            .get_fragment_and_inner_indices(a)
            .expect("out-of-bounds");
        let (bf, bi) = self
            .get_fragment_and_inner_indices(b)
            .expect("out-of-bounds");
        if af == bf {
            self.fragments[af].swap(ai, bi);
        } else {
            let ptr_a = self.fragments[af].as_mut_ptr().add(ai);
            let ref_a = unsafe { &mut *ptr_a };
            let ref_b = &mut self.fragments[bf][bi];
            std::mem::swap(ref_a, ref_b);
        }
    }
    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// fn get_vec() -> SplitVec<usize, Linear> {
    ///     let mut vec = SplitVec::with_linear_growth(4);
    ///     for i in 0..11 {
    ///         vec.push(i);
    ///     }
    ///     // [ [0,1,2,3], [4,5,6,7], [8,9,10] ]
    ///     vec
    /// }
    ///
    /// let mut vec = get_vec();
    /// vec.truncate(100);
    /// assert_eq!(vec, (0..11).collect::<Vec<_>>());
    ///
    /// for i in 0..11 {
    ///     let mut vec = get_vec();
    ///     vec.truncate(i);
    ///     assert_eq!(vec, (0..i).collect::<Vec<_>>());
    /// }
    ///
    /// ```
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// `truncate` is unsafe since it is possible that remaining elements are referencing
    /// to elements which are dropped by the truncate method.
    ///
    /// On the other hand, any vector implementing `PinnedVec<T>` where `T: NotSelfRefVecItem`
    /// implements `PinnedVecSimple<T>` which implements the safe version of this method.
    unsafe fn unsafe_truncate(&mut self, len: usize) {
        if let Some((f, i)) = self.get_fragment_and_inner_indices(len) {
            self.fragments.truncate(f + 1);
            self.fragments[f].truncate(i);
            self.len = len;
        }
    }

    // required for common trait implementations
    fn partial_eq<S>(&self, other: S) -> bool
    where
        S: AsRef<[T]>,
        T: PartialEq,
    {
        are_fragments_eq_to_slice(&self.fragments, other.as_ref())
    }
    fn debug(&self, f: &mut Formatter<'_>) -> Result
    where
        T: Debug,
    {
        writeln!(f, "SplitVec [")?;
        for frag in &self.fragments {
            writeln!(f, "    {:?}", frag)?;
        }
        writeln!(f, "]")
    }

    unsafe fn unsafe_clone(&self) -> Self
    where
        T: Clone,
    {
        let fragments: Vec<_> = self
            .fragments
            .iter()
            .map(|fragment| {
                let mut vec = Vec::with_capacity(fragment.capacity());
                vec.extend_from_slice(fragment);
                vec.into()
            })
            .collect();
        Self {
            fragments,
            len: self.len,
            growth: self.growth.clone(),
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        Self::Iter::new(&self.fragments)
    }
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Self::IterMut::new(&mut self.fragments)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test::macros::Num;
    use crate::test_all_growth_types;

    #[test]
    fn index_of() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            let mut another_vec = vec![];
            for i in 0..157 {
                vec.push(i);
                another_vec.push(i);
            }
            for i in 0..vec.len() {
                assert_eq!(Some(i), vec.index_of(&vec[i]));
                assert_eq!(None, vec.index_of(&another_vec[i]));

                let scalar = another_vec[i];
                assert_eq!(None, vec.index_of(&scalar));
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
    fn unsafe_insert() {
        fn test<G: Growth>(mut vec: SplitVec<Num, G>) {
            for i in 0..42 {
                vec.push(Num::new(i));
            }
            for i in 0..42 {
                unsafe { vec.unsafe_insert(i, Num::new(100 + i)) };
            }

            for i in 0..42 {
                assert_eq!(Some(&Num::new(i)), vec.get(42 + i));
                assert_eq!(Some(&Num::new(100 + i)), vec.get(i));
            }
        }
        test_all_growth_types!(test);
    }
    #[test]
    fn unsafe_shrink() {
        fn test<G: Growth>(mut vec: SplitVec<Num, G>) {
            for i in 0..42 {
                vec.push(Num::new(i));
                assert_eq!(Num::new(i), unsafe { vec.unsafe_remove(0) });
                assert!(vec.is_empty());
            }

            for i in 0..42 {
                vec.push(Num::new(i));
            }
            for i in 0..42 {
                assert_eq!(Num::new(i), unsafe { vec.unsafe_remove(0) });
            }
            assert!(vec.is_empty());

            for i in 0..42 {
                vec.push(Num::new(i));
            }
            for i in (0..42).rev() {
                assert_eq!(Some(Num::new(i)), unsafe { vec.unsafe_pop() });
            }
            assert_eq!(None, unsafe { vec.unsafe_pop() });
            assert!(vec.is_empty());

            for i in 0..42 {
                vec.push(Num::new(i));
            }
            for _ in 0..42 {
                unsafe { vec.unsafe_remove(vec.len() / 2) };
            }
            assert!(vec.is_empty());
        }
        test_all_growth_types!(test);
    }
    #[test]
    fn unsafe_swap() {
        fn test<G: Growth>(mut vec: SplitVec<Num, G>) {
            for i in 0..42 {
                vec.push(Num::new(i));
            }

            for i in 0..21 {
                unsafe { vec.unsafe_swap(i, 21 + i) };
            }

            for i in 0..21 {
                assert_eq!(Num::new(21 + i), vec[i]);
            }
            for i in 21..42 {
                assert_eq!(Num::new(i - 21), vec[i]);
            }
        }
        test_all_growth_types!(test);
    }
    #[test]
    fn unsafe_truncate() {
        fn test<G: Growth>(mut vec: SplitVec<Num, G>) {
            let std_vec: Vec<_> = (0..42).map(Num::new).collect();
            for i in 0..42 {
                vec.push(Num::new(i));
            }

            unsafe { vec.unsafe_truncate(100) };
            assert_eq!(vec, std_vec);

            for i in (0..42).rev() {
                unsafe { vec.unsafe_truncate(i) };
                assert_eq!(vec, &std_vec[0..i]);
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn unsafe_clone() {
        fn test<G: Growth>(mut vec: SplitVec<Num, G>) {
            assert!(vec.is_empty());

            for i in 0..53 {
                vec.push(Num::new(i));
            }

            let clone = unsafe { vec.unsafe_clone() };
            assert_eq!(vec, clone);
        }
        test_all_growth_types!(test);
    }
}
