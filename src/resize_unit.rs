use crate::SplitVec;

impl<T> SplitVec<T> {
    /// Appends an element to the back of a collection.
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
    /// ```
    pub fn push(&mut self, value: T) {
        let last_f = self.fragments.len() - 1;
        let fragment = &mut self.fragments[last_f];
        if fragment.has_capacity_for_one() {
            fragment.push(value);
            return;
        }

        self.add_fragment_with_first_value(value);
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
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
    ///
    /// assert_eq!(vec.pop(), Some(3));
    /// assert_eq!(vec, [1, 2]);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.fragments
            .iter_mut()
            .last()
            .and_then(|fragment| fragment.pop())
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
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::default();
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
    pub fn insert(&mut self, index: usize, value: T) {
        if index == self.len() {
            self.push(value);
        } else {
            // make room for one
            if !self.last_fragment().has_capacity_for_one() {
                self.add_fragment();
            }

            let (f, i) = self.fragment_and_inner_index(index).expect("out-of-bounds");

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
        }
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
    /// use orx_split_vec::SplitVec;
    ///
    /// let mut vec = SplitVec::default();
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
    pub fn remove(&mut self, index: usize) -> T {
        let (f, i) = self.fragment_and_inner_index(index).expect("out-of-bounds");

        let value = self.fragments[f].remove(i);

        for f2 in f + 1..self.fragments.len() {
            let x = self.fragments[f2].remove(0);
            self.fragments[f2 - 1].push(x);
            if self.fragments[f2].is_empty() {
                self.fragments.remove(f2);
                break;
            }
        }

        value
    }
}
