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
    }
}
