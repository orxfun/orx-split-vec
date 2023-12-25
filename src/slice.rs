use orx_pinned_vec::PinnedVec;

use crate::{Fragment, Growth, SplitVec};
use std::ops::{Range, RangeBounds};

#[derive(PartialEq, Eq, Debug, Clone)]
/// Returns the result of trying to get a slice as a contagious memory from the split vector.
pub enum SplitVecSlice<'a, T> {
    /// The desired range completely belongs to one fragment and the slice can be provided.
    Ok(&'a [T]),
    /// The desired range is split to at least two fragments.
    /// The tuple contains indices of the fragments containing
    /// the first and last element of the desired range.
    Fragmented(usize, usize),
    /// An error case where the desired range is out of bounds of the vector.
    OutOfBounds,
}

impl<T, G: Growth> SplitVec<T, G> {
    fn range_start(range: &Range<usize>) -> usize {
        match range.start_bound() {
            std::ops::Bound::Excluded(x) => x + 1,
            std::ops::Bound::Included(x) => *x,
            std::ops::Bound::Unbounded => 0,
        }
    }
    fn range_end(&self, range: &Range<usize>) -> usize {
        match range.end_bound() {
            std::ops::Bound::Excluded(x) => *x,
            std::ops::Bound::Included(x) => x + 1,
            std::ops::Bound::Unbounded => self.len(),
        }
    }

    /// Returns the result of trying to return the required `range` as a contagious slice of data.
    /// It might return Ok of the slice if the range belongs to one fragment.
    ///
    /// Otherwise, one of the two failure cases will be returned:
    /// * OutOfBounds if the range does not fit in the range of the entire split vector, or
    /// * Fragmented if the range belongs to at least two fragments, additionally returns the fragment indices of the range.
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
    /// assert_eq!(4, vec.fragments()[0].capacity());
    /// assert_eq!(4, vec.fragments()[1].capacity());
    /// assert_eq!(4, vec.fragments()[2].capacity());
    ///
    /// assert_eq!(4, vec.fragments()[0].len()); // [0, 1, 2, 3]
    /// assert_eq!(4, vec.fragments()[1].len()); // [4, 5, 6, 7]
    /// assert_eq!(2, vec.fragments()[2].len()); // [8, 9]
    ///
    /// // Ok
    /// assert_eq!(SplitVecSlice::Ok(&[0, 1, 2, 3]), vec.try_get_slice(0..4));
    /// assert_eq!(SplitVecSlice::Ok(&[5, 6]), vec.try_get_slice(5..7));
    /// assert_eq!(SplitVecSlice::Ok(&[8, 9]), vec.try_get_slice(8..10));
    ///
    /// // Fragmented
    /// assert_eq!(SplitVecSlice::Fragmented(0, 1), vec.try_get_slice(3..6));
    /// assert_eq!(SplitVecSlice::Fragmented(0, 2), vec.try_get_slice(3..9));
    /// assert_eq!(SplitVecSlice::Fragmented(1, 2), vec.try_get_slice(7..9));
    ///
    /// // OutOfBounds
    /// assert_eq!(SplitVecSlice::OutOfBounds, vec.try_get_slice(5..12));
    /// assert_eq!(SplitVecSlice::OutOfBounds, vec.try_get_slice(10..11));
    /// ```
    pub fn try_get_slice(&self, range: Range<usize>) -> SplitVecSlice<T> {
        let a = Self::range_start(&range);
        let b = self.range_end(&range);

        if b == 0 {
            SplitVecSlice::Ok(&[])
        } else if let Some((sf, si)) = self.get_fragment_and_inner_indices(a) {
            if let Some((ef, ei)) = self.get_fragment_and_inner_indices(b - 1) {
                if sf == ef {
                    SplitVecSlice::Ok(&self.fragments[sf][si..=ei])
                } else {
                    SplitVecSlice::Fragmented(sf, ef)
                }
            } else {
                SplitVecSlice::OutOfBounds
            }
        } else {
            SplitVecSlice::OutOfBounds
        }
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
    /// assert_eq!(4, vec.fragments()[0].capacity());
    /// assert_eq!(4, vec.fragments()[1].capacity());
    /// assert_eq!(4, vec.fragments()[2].capacity());
    ///
    /// assert_eq!(4, vec.fragments()[0].len()); // [0, 1, 2, 3]
    /// assert_eq!(4, vec.fragments()[1].len()); // [4, 5, 6, 7]
    /// assert_eq!(2, vec.fragments()[2].len()); // [8, 9]
    ///
    /// // single fragment
    /// assert_eq!(vec![&[0, 1, 2, 3]], vec.slice(0..4));
    /// assert_eq!(vec![&[5, 6]], vec.slice(5..7));
    /// assert_eq!(vec![&[8, 9]], vec.slice(8..10));
    ///
    /// // Fragmented
    /// assert_eq!(vec![&vec![3], &vec![4, 5]], vec.slice(3..6));
    /// assert_eq!(vec![&vec![3], &vec![4, 5, 6, 7], &vec![8]], vec.slice(3..9));
    /// assert_eq!(vec![&vec![7], &vec![8]], vec.slice(7..9));
    ///
    /// // OutOfBounds
    /// assert!(vec.slice(5..12).is_empty());
    /// assert!(vec.slice(10..11).is_empty());
    /// ```
    pub fn slice(&self, range: Range<usize>) -> Vec<&[T]> {
        let a = Self::range_start(&range);
        let b = self.range_end(&range);

        if b == 0 {
            vec![]
        } else if let Some((sf, si)) = self.get_fragment_and_inner_indices(a) {
            if let Some((ef, ei)) = self.get_fragment_and_inner_indices(b - 1) {
                if sf == ef {
                    vec![&self.fragments[sf][si..=ei]]
                } else {
                    let mut vec = Vec::with_capacity(ef - sf + 1);
                    vec.push(&self.fragments[sf][si..]);
                    for f in sf + 1..ef {
                        vec.push(&self.fragments[f]);
                    }
                    vec.push(&self.fragments[ef][..=ei]);
                    vec
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn try_get_slice() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..42 {
                assert_eq!(SplitVecSlice::OutOfBounds, vec.try_get_slice(0..i + 1));
                assert_eq!(SplitVecSlice::OutOfBounds, vec.try_get_slice(i..i + 1));
                vec.push(i);
            }

            for f in 0..vec.fragments.len() {
                let begin: usize = vec.fragments.iter().take(f).map(|f| f.len()).sum();
                let end = begin + vec.fragments[f].len();
                let half = begin + vec.fragments[f].len() / 2;

                // ok
                let slice_full_fragment = vec.try_get_slice(begin..end);
                assert_eq!(slice_full_fragment, SplitVecSlice::Ok(&vec.fragments[f]));

                let slice_half_fragment = vec.try_get_slice(begin..half);
                assert_eq!(
                    slice_half_fragment,
                    SplitVecSlice::Ok(&vec.fragments[f][0..vec.fragments[f].len() / 2])
                );

                let slice_half_fragment = vec.try_get_slice(half..end);
                assert_eq!(
                    slice_half_fragment,
                    SplitVecSlice::Ok(
                        &vec.fragments[f][vec.fragments[f].len() / 2..vec.fragments[f].len()]
                    )
                );

                // fragmented
                if f > 0 {
                    let prev_begin = begin - 1;
                    let slice = vec.try_get_slice(prev_begin..end);
                    assert_eq!(slice, SplitVecSlice::Fragmented(f - 1, f));
                    if f < vec.fragments.len() - 1 {
                        let next_end = end + 1;

                        let slice = vec.try_get_slice(begin..next_end);
                        assert_eq!(slice, SplitVecSlice::Fragmented(f, f + 1));

                        let slice = vec.try_get_slice(prev_begin..next_end);
                        assert_eq!(slice, SplitVecSlice::Fragmented(f - 1, f + 1));
                    }
                }
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn slice() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..184 {
                assert!(vec.slice(i..i + 1).is_empty());
                assert!(vec.slice(0..i + 1).is_empty());
                vec.push(i);
            }

            let slice = vec.slice(0..vec.len());
            let mut combined = vec![];
            for s in slice {
                combined.extend_from_slice(s);
            }
            for i in 0..184 {
                assert_eq!(i, vec[i]);
                assert_eq!(i, combined[i]);
            }

            let begin = vec.len() / 4;
            let end = 3 * vec.len() / 4;
            let slice = vec.slice(begin..end);
            let mut combined = vec![];
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
}
