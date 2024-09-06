use crate::*;
use alloc::vec::Vec;

impl<T, G, U> PartialEq<U> for SplitVec<T, G>
where
    U: AsRef<[T]>,
    T: PartialEq,
    G: Growth,
{
    fn eq(&self, other: &U) -> bool {
        are_fragments_eq_to_slice(&self.fragments, other.as_ref())
    }
}

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

#[cfg(test)]
mod tests {
    use crate::test_all_growth_types;
    use crate::*;
    use alloc::vec::Vec;

    #[test]
    fn eq() {
        fn test<G: Growth>(mut vec: SplitVec<usize, G>) {
            for i in 0..142 {
                vec.push(i);
            }

            let eq_vec: Vec<_> = (0..vec.capacity()).collect();
            let eq_vec_as_ref: &[usize] = eq_vec.as_ref();
            assert_eq!(vec, eq_vec_as_ref);
            assert_eq!(&vec, &eq_vec);
            assert_eq!(vec, eq_vec);
        }

        test_all_growth_types!(test);
    }
}
