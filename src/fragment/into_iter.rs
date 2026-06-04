use crate::Fragment;

impl<T> IntoIterator for Fragment<T> {
    type Item = T;

    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
