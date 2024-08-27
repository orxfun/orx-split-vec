use crate::Fragment;
use std::mem::ManuallyDrop;

pub fn fragment_into_raw<T>(mut fragment: Fragment<T>) -> (*mut T, usize, usize) {
    let (len, capacity) = (fragment.len(), fragment.capacity());
    let ptr = fragment.as_mut_ptr();

    let _ = ManuallyDrop::new(fragment);

    (ptr, len, capacity)
}

pub unsafe fn fragment_from_raw<T>(ptr: *mut T, len: usize, capacity: usize) -> Fragment<T> {
    assert!(capacity >= len);
    let vec = Vec::from_raw_parts(ptr, len, capacity);
    vec.into()
}
