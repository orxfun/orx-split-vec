use crate::Fragment;
use alloc::vec::Vec;
use core::mem::ManuallyDrop;

pub(crate) struct RawFragment<T> {
    pub ptr: *const T,
    pub len: usize,
    pub capacity: usize,
    pub must_drop: bool,
}

impl<T> RawFragment<T> {
    fn new(ptr: *const T, len: usize, capacity: usize, must_drop: bool) -> Self {
        debug_assert!(len <= capacity);
        Self {
            ptr,
            len,
            capacity,
            must_drop,
        }
    }
}

impl<T> From<Fragment<T>> for RawFragment<T> {
    fn from(fragment: Fragment<T>) -> Self {
        let (ptr, len, capacity) = (fragment.as_ptr(), fragment.len(), fragment.capacity());
        let _ = ManuallyDrop::new(fragment);
        Self::new(ptr, len, capacity, true)
    }
}

impl<T> From<&[T]> for RawFragment<T> {
    fn from(fragment: &[T]) -> Self {
        let (ptr, len, capacity) = (fragment.as_ptr(), fragment.len(), fragment.len());
        Self::new(ptr, len, capacity, false)
    }
}

impl<T> Drop for RawFragment<T> {
    fn drop(&mut self) {
        if self.must_drop {
            let _vec_to_drop = unsafe { Vec::from_raw_parts(self.ptr as *mut T, 0, self.capacity) };
        }
    }
}
