use core::ops::{Bound, RangeBounds};

pub(crate) fn range_start<R: RangeBounds<usize>>(range: &R) -> usize {
    match range.start_bound() {
        Bound::Excluded(x) => x + 1,
        Bound::Included(x) => *x,
        Bound::Unbounded => 0,
    }
}
pub(crate) fn range_end<R: RangeBounds<usize>>(range: &R, vec_len: usize) -> usize {
    match range.end_bound() {
        Bound::Excluded(x) => *x,
        Bound::Included(x) => x + 1,
        Bound::Unbounded => vec_len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_all_growth_types, Growth, SplitVec};
    use orx_pinned_vec::PinnedVec;

    #[test]
    fn range_start_end() {
        fn test<G: Growth>(vec: SplitVec<usize, G>) {
            assert_eq!(10, range_start(&(10..20)));
            assert_eq!(10, range_start(&(10..=20)));
            assert_eq!(0, range_start(&(..20)));
            assert_eq!(10, range_start(&(10..)));
            assert_eq!(0, range_start(&(..)));

            assert_eq!(20, range_end(&(10..20), vec.len()));
            assert_eq!(21, range_end(&(10..=20), vec.len()));
            assert_eq!(20, range_end(&(..20), vec.len()));
            assert_eq!(vec.len(), range_end(&(10..), vec.len()));
            assert_eq!(vec.len(), range_end(&(..), vec.len()));
        }

        test_all_growth_types!(test);
    }
}
