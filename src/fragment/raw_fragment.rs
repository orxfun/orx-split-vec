use crate::Fragment;
use core::mem::ManuallyDrop;

pub(crate) struct RawFragment<T> {
    pub ptr: *const T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> RawFragment<T> {
    pub fn new(ptr: *const T, len: usize, capacity: usize) -> Self {
        Self { ptr, len, capacity }
    }
}

impl<T> From<Fragment<T>> for RawFragment<T> {
    fn from(fragment: Fragment<T>) -> Self {
        let (ptr, len, capacity) = (fragment.as_ptr(), fragment.len(), fragment.capacity());
        let _ = ManuallyDrop::new(fragment);
        Self::new(ptr, len, capacity)
    }
}
