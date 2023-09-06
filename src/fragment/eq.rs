use crate::{Fragment, Growth, SplitVec};
use orx_fixed_vec::FixedVec;
use orx_pinned_vec::PinnedVec;

impl<T: PartialEq, U> PartialEq<U> for Fragment<T>
where
    U: AsRef<[T]>,
{
    fn eq(&self, other: &U) -> bool {
        self.data == other.as_ref()
    }
}

impl<T: PartialEq> PartialEq<Fragment<T>> for [T] {
    fn eq(&self, other: &Fragment<T>) -> bool {
        self == other.data
    }
}
impl<T: PartialEq> PartialEq<Fragment<T>> for Vec<T> {
    fn eq(&self, other: &Fragment<T>) -> bool {
        self == &other.data
    }
}
impl<T: PartialEq, const N: usize> PartialEq<Fragment<T>> for [T; N] {
    fn eq(&self, other: &Fragment<T>) -> bool {
        self.as_slice() == other.data
    }
}

impl<T: PartialEq, G: Growth> PartialEq<SplitVec<T, G>> for FixedVec<T> {
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
