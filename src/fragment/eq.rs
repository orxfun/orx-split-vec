use crate::Fragment;
use alloc::vec::Vec;

impl<T: PartialEq, U> PartialEq<U> for Fragment<T>
where
    U: AsRef<[T]>,
{
    fn eq(&self, other: &U) -> bool {
        self.as_slice() == other.as_ref()
    }
}

impl<T: PartialEq> PartialEq<Fragment<T>> for [T] {
    fn eq(&self, other: &Fragment<T>) -> bool {
        self == other.as_slice()
    }
}

impl<T: PartialEq> PartialEq<Fragment<T>> for Vec<T> {
    fn eq(&self, other: &Fragment<T>) -> bool {
        self == other.as_slice()
    }
}

impl<T: PartialEq, const N: usize> PartialEq<Fragment<T>> for [T; N] {
    fn eq(&self, other: &Fragment<T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}
