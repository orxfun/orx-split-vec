use crate::{Fragment, Growth, SplitVec};
use orx_pinned_vec::PinnedVec;

impl<T: PartialEq, G> PartialEq<SplitVec<T, G>> for [T]
where
    G: Growth,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        are_fragments_eq_to_slice(&other.fragments, self)
    }
}

impl<T: PartialEq, G> PartialEq<SplitVec<T, G>> for Vec<T>
where
    G: Growth,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        are_fragments_eq_to_slice(&other.fragments, self)
    }
}

impl<T: PartialEq, G, const N: usize> PartialEq<SplitVec<T, G>> for [T; N]
where
    G: Growth,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        are_fragments_eq_to_slice(&other.fragments, self)
    }
}

impl<T: PartialEq, G> PartialEq<SplitVec<T, G>> for SplitVec<T, G>
where
    G: Growth,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        let mut iter1 = self.iter();
        let mut iter2 = other.iter();
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(x), Some(y)) => {
                    if x != y {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

impl<T: PartialEq, G: Growth> Eq for SplitVec<T, G> {}

pub(crate) fn are_fragments_eq_to_slice<T: PartialEq>(
    fragments: &[Fragment<T>],
    slice: &[T],
) -> bool {
    let mut slice_beg = 0;
    for fragment in fragments {
        let slice_end = slice_beg + fragment.len();
        let slice_of_slice = &slice[slice_beg..slice_end];
        if fragment.data != slice_of_slice {
            return false;
        }
        slice_beg = slice_end;
    }
    true
}
