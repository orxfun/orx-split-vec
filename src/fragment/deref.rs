use crate::Fragment;
use alloc::vec::Vec;
use core::ops::Deref;

impl<T> Deref for Fragment<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
