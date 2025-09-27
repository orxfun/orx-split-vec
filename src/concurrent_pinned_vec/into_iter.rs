use alloc::vec::Vec;
use core::cell::UnsafeCell;

pub struct ConcurrentSplitVecIntoIter<T> {
    data: Vec<UnsafeCell<*mut T>>,
}
