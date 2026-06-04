use crate::Fragment;
use alloc::vec::Vec;

impl<T> From<Fragment<T>> for Vec<T> {
    fn from(value: Fragment<T>) -> Self {
        value.data
    }
}
