use crate::{SplitVec, SplitVecGrowth};
use std::ops::Range;

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

impl<T, G> SplitVec<T, G>
where
    G: SplitVecGrowth<T>,
{
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
    /// use orx_split_vec::{SplitVec, SplitVecSlice};
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
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
        if let Some((sf, si)) = self.get_fragment_and_inner_indices(range.start) {
            if let Some((ef, ei)) = self.get_fragment_and_inner_indices(range.end - 1) {
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
    /// use orx_split_vec::{SplitVec, SplitVecSlice};
    ///
    /// let mut vec = SplitVec::with_linear_growth(4);
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
        if let Some((sf, si)) = self.get_fragment_and_inner_indices(range.start) {
            if let Some((ef, ei)) = self.get_fragment_and_inner_indices(range.end - 1) {
                if sf == ef {
                    vec![&self.fragments[sf][si..=ei]]
                } else {
                    let mut vec = Vec::with_capacity(ef - sf);
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
