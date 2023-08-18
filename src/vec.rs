use crate::{Fragment, SplitVec};

impl<T> SplitVec<T> {
    /// Returns the number of elements in the vector, also referred to
    /// as its 'length'.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::default();
    /// assert_eq!(0, vec.len());
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(3, vec.len());
    /// ```
    pub fn len(&self) -> usize {
        self.fragments.iter().map(|f| f.len()).sum()
    }
    /// Returns `true` if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::default();
    /// assert!(vec.is_empty());
    /// vec.push(1);
    /// assert!(!vec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.fragments.iter().all(|f| f.is_empty())
    }

    /// Returns the total number of elements the split vector can hold without
    /// reallocating.
    ///
    /// See `FragmentGrowth` for details of capacity growth policies.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// // default growth starting with 4, and doubling at each new fragment.
    /// let mut vec = SplitVec::<usize>::default();
    /// assert_eq!(4, vec.capacity());
    ///
    /// for i in 0..4 {
    ///     vec.push(i);
    /// }
    /// assert_eq!(4, vec.capacity());
    ///
    /// vec.push(4);
    /// assert_eq!(4 + 6, vec.capacity());
    ///
    /// // second fragment will have capacity 4*1.5 by default growth
    /// // see `FragmentGrowth` for different growth strategies.
    ///
    /// ```
    pub fn capacity(&self) -> usize {
        self.fragments.iter().map(|f| f.capacity()).sum()
    }

    /// Returns a reference to the element with the given `index`;
    /// None if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::<usize>::default();
    /// vec.extend_from_slice(&[10, 40, 30]);
    /// assert_eq!(Some(&40), vec.get(1));
    /// assert_eq!(None, vec.get(3));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.fragment_and_inner_index(index)
            .map(|(f, i)| &self.fragments[f][i])
    }
    /// Returns a mutable reference to the element with the given `index`;
    /// None if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::<usize>::default();
    /// vec.extend_from_slice(&[0, 1, 2]);
    ///
    /// if let Some(elem) = vec.get_mut(1) {
    ///     *elem = 42;
    /// }
    ///
    /// assert_eq!(vec, &[0, 42, 2]);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.fragment_and_inner_index(index)
            .map(|(f, i)| &mut self.fragments[f][i])
    }

    /// Directly appends the `fragment` to the end of the split vector.
    ///
    /// This operation does not require any copies or allocation;
    /// the fragment is moved into the split vector and added as a new fragment,
    /// without copying the underlying data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::<usize>::default();
    ///
    /// // append to empty split vector
    /// assert!(vec.is_empty());
    /// let mut other = Vec::with_capacity(4);
    /// other.extend_from_slice(&[0, 1, 2]);
    ///
    /// vec.append(other);
    /// assert_eq!(vec, &[0, 1, 2]);
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(4, vec.fragments()[0].capacity()); // SplitVec will make use of the appended vector's additional capacity
    ///
    /// vec.push(3);
    /// assert_eq!(vec, &[0, 1, 2, 3]);
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(vec.fragments()[0].as_slice(), &[0, 1, 2, 3]);
    ///
    /// // next push will use SplitVec's growth
    /// vec.extend_from_slice(&[4, 5, 6]);
    /// assert_eq!(vec, &[0, 1, 2, 3, 4, 5, 6]);
    /// assert_eq!(2, vec.fragments().len());
    /// assert_eq!(vec.fragments()[0].as_slice(), &[0, 1, 2, 3]);
    /// assert_eq!(vec.fragments()[1].as_slice(), &[4, 5, 6]);
    ///
    /// // we can append another fragment directly
    /// vec.append(vec![7, 8]);
    /// assert_eq!(vec, &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
    /// assert_eq!(3, vec.fragments().len());
    /// assert_eq!(vec.fragments()[0].as_slice(), &[0, 1, 2, 3]);
    /// assert_eq!(vec.fragments()[1].as_slice(), &[4, 5, 6]);
    /// assert_eq!(vec.fragments()[2].as_slice(), &[7, 8]);
    /// ```
    pub fn append<F>(&mut self, fragment: F)
    where
        F: Into<Fragment<T>>,
    {
        if self.is_empty() {
            self.fragments[0] = fragment.into();
        } else {
            self.fragments.push(fragment.into());
        }
    }
}
