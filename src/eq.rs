use crate::{Fragment, SplitVec, SplitVecGrowth};

impl<T: PartialEq, G> PartialEq<SplitVec<T, G>> for [T]
where
    G: SplitVecGrowth<T>,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        are_fragments_eq_to_slice(&other.fragments, self)
    }
}

impl<T: PartialEq, G> PartialEq<SplitVec<T, G>> for Vec<T>
where
    G: SplitVecGrowth<T>,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        are_fragments_eq_to_slice(&other.fragments, self)
    }
}

impl<T: PartialEq, G, const N: usize> PartialEq<SplitVec<T, G>> for [T; N]
where
    G: SplitVecGrowth<T>,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        are_fragments_eq_to_slice(&other.fragments, self)
    }
}

impl<T: PartialEq, G> PartialEq<SplitVec<T, G>> for SplitVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        let iter1 = self.iter();
        let iter2 = other.iter();
        iter1 == iter2
    }
}

impl<T: PartialEq, G: SplitVecGrowth<T>> Eq for SplitVec<T, G> {}

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
