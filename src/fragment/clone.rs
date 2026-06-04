use crate::Fragment;
use alloc::vec::Vec;

impl<T: Clone> Clone for Fragment<T> {
    fn clone(&self) -> Self {
        let mut data = Vec::with_capacity(self.capacity());
        data.extend_from_slice(self.as_slice());
        Self::new(self.capacity(), data)
    }
}
