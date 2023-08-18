use crate::Fragment;

impl<T> AsRef<[T]> for Fragment<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}
