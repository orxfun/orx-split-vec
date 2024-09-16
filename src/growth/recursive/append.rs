use crate::{IntoFragments, Recursive, SplitVec};

impl<T> SplitVec<T, Recursive> {
    /// Consumes and appends `other` vector into this vector in constant time without memory copies.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_split_vec::*;
    ///
    /// let mut recursive = SplitVec::with_recursive_growth();
    ///
    /// recursive.push('a');
    /// assert_eq!(recursive, &['a']);
    ///
    /// recursive.append(vec!['b', 'c']);
    /// assert_eq!(recursive, &['a', 'b', 'c']);
    ///
    /// recursive.append(vec![vec!['d'], vec!['e', 'f']]);
    /// assert_eq!(recursive, &['a', 'b', 'c', 'd', 'e', 'f']);
    ///
    /// let other_split_vec: SplitVec<_> = vec!['g', 'h'].into();
    /// recursive.append(other_split_vec);
    /// assert_eq!(recursive, &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']);
    /// ```
    pub fn append<I: IntoFragments<T>>(&mut self, other: I) {
        let fragments = other.into_fragments();
        for fragment in fragments {
            self.len += fragment.len();
            self.fragments.push(fragment);
        }
        // TODO: does this break internal structure of the vec; be careful on its impact on linked-list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orx_pinned_vec::PinnedVec;

    #[test]
    fn append_full_fragment_when_empty() {
        let mut vec = SplitVec::with_recursive_growth();
        assert_eq!(vec.capacity(), 4);

        vec.append(alloc::vec![0, 1, 2]);
        assert_eq!(vec.fragments().len(), 2);
        assert_eq!(vec.capacity(), 4 + 3);

        vec.push(3);
        assert_eq!(vec.fragments().len(), 3);
        assert_eq!(vec.capacity(), 4 + 3 + 6);

        assert_eq!(vec, &[0, 1, 2, 3]);
    }

    #[test]
    fn append_half_fragment_when_empty() {
        let mut vec = SplitVec::with_recursive_growth();
        assert_eq!(vec.capacity(), 4);

        let mut append = alloc::vec::Vec::with_capacity(4);
        append.extend_from_slice(&[0, 1, 2]);
        vec.append(append);
        assert_eq!(vec.fragments().len(), 2);
        assert_eq!(vec.capacity(), 4 + 4);

        vec.push(3);
        assert_eq!(vec.fragments().len(), 2);
        assert_eq!(vec.capacity(), 4 + 4);

        vec.push(4);
        assert_eq!(vec.fragments().len(), 3);
        assert_eq!(vec.capacity(), 4 + 4 + 8);

        assert_eq!(vec, &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn append_full_fragment_when_non_empty() {
        let mut vec = SplitVec::with_recursive_growth();
        vec.push(42);
        assert_eq!(vec.capacity(), 4);

        vec.append(alloc::vec![0, 1, 2]);
        assert_eq!(vec.fragments().len(), 2);
        assert_eq!(vec.capacity(), 4 + 3);

        vec.push(3);
        assert_eq!(vec.fragments().len(), 3);
        assert_eq!(vec.capacity(), 4 + 3 + 6);

        assert_eq!(vec, &[42, 0, 1, 2, 3]);
    }

    #[test]
    fn append_half_fragment_when_non_empty() {
        let mut vec = SplitVec::with_recursive_growth();
        vec.push(42);
        assert_eq!(vec.capacity(), 4);

        let mut append = alloc::vec::Vec::with_capacity(4);
        append.extend_from_slice(&[0, 1, 2]);
        vec.append(append);
        assert_eq!(vec.fragments().len(), 2);
        assert_eq!(vec.capacity(), 4 + 4);

        vec.push(3);
        assert_eq!(vec.fragments().len(), 2);
        assert_eq!(vec.capacity(), 4 + 4);

        vec.push(4);
        assert_eq!(vec.fragments().len(), 3);
        assert_eq!(vec.capacity(), 4 + 4 + 8);

        assert_eq!(vec, &[42, 0, 1, 2, 3, 4]);
    }
}
