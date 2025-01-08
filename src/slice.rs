use crate::{
    range_helpers::{range_end, range_start},
    Growth, SplitVec,
};
use core::{cmp::Ordering, ops::RangeBounds};
use orx_pinned_vec::PinnedVec;

#[derive(PartialEq, Eq, Debug, Clone)]
/// Returns the result of trying to get a slice as a contiguous memory from the split vector.
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
    /// Returns the result of trying to return the required `range` as a contiguous slice of data.
    /// It might return Ok of the slice if the range belongs to one fragment.
    ///
    /// Otherwise, one of the two failure cases will be returned:
    /// * OutOfBounds if the range does not fit in the range of the entire split vector, or
    /// * Fragmented if the range belongs to at least two fragments, additionally returns the fragment indices of the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::*;
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
    pub fn try_get_slice<R: RangeBounds<usize>>(&self, range: R) -> SplitVecSlice<T> {
        let a = range_start(&range);
        let b = range_end(&range, self.len());

        match b.saturating_sub(a) {
            0 => SplitVecSlice::Ok(&[]),
            _ => match self.get_fragment_and_inner_indices(a) {
                None => SplitVecSlice::OutOfBounds,
                Some((sf, si)) => match self.get_fragment_and_inner_indices(b - 1) {
                    None => SplitVecSlice::OutOfBounds,
                    Some((ef, ei)) => match sf.cmp(&ef) {
                        Ordering::Equal => SplitVecSlice::Ok(&self.fragments[sf][si..=ei]),
                        _ => SplitVecSlice::Fragmented(sf, ef),
                    },
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_all_growth_types;
    use crate::*;

    #[test]
    fn try_get_slice() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..42 {
                assert_eq!(SplitVecSlice::OutOfBounds, vec.try_get_slice(0..(i + 1)));
                assert_eq!(SplitVecSlice::OutOfBounds, vec.try_get_slice(i..(i + 1)));
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
}
