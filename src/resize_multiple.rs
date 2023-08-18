use crate::SplitVec;

impl<T: Clone> SplitVec<T> {
    /// Clones and appends all elements in a slice to the vec.
    ///
    /// Iterates over the slice `other`, clones each element, and then appends
    /// it to this vector. The `other` slice is traversed in-order.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::default();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend_from_slice(&[4, 5, 6, 7]);
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    pub fn extend_from_slice(&mut self, other: &[T]) {
        let mut slice = other;
        while !slice.is_empty() {
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
}

impl<'a, T: Clone + 'a> Extend<&'a T> for SplitVec<T> {
    /// Clones and appends all elements in the iterator to the vec.
    ///
    /// Iterates over the `iter`, clones each element, and then appends
    /// it to this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::default();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend(&[4, 5, 6, 7]);
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    ///
    /// let mut sec_vec = SplitVec::default();
    /// sec_vec.extend(vec.into_iter());
    /// assert_eq!(sec_vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x.clone());
        }
    }
}

impl<T> Extend<T> for SplitVec<T> {
    /// Extends a collection with the contents of an iterator.
    ///
    /// Iterates over the `iter`, moves and appends each element
    /// to this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::default();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend(vec![4, 5, 6, 7].into_iter());
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x);
        }
    }
}
