use crate::Fragment;

impl<T: Clone> Clone for Fragment<T> {
    fn clone(&self) -> Self {
        let mut data = Vec::with_capacity(self.data.capacity());
        data.extend(self.data.iter().cloned());
        data.into()
    }
}
