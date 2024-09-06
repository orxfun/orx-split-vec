use crate::Fragment;
use alloc::vec::Vec;

impl<T> From<Vec<T>> for Fragment<T> {
    fn from(value: Vec<T>) -> Self {
        Self { data: value }
    }
}
impl<T> From<Fragment<T>> for Vec<T> {
    fn from(value: Fragment<T>) -> Self {
        value.data
    }
}
