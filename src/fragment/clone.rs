use crate::Fragment;
use alloc::vec::Vec;

impl<T: Clone> Clone for Fragment<T> {
    fn clone(&self) -> Self {
        let mut data = Vec::with_capacity(self.data.capacity());
        data.extend_from_slice(&self.data);
        Self {
            data,
            capacity: self.capacity,
        }
    }
}
