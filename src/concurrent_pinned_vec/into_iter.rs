use crate::{
    GrowthWithConstantTimeAccess,
    concurrent_pinned_vec::into_iter_ptr_slices::IntoIterPtrOfConSlices,
};
use alloc::vec::Vec;
use core::{cell::UnsafeCell, ops::Range};

pub struct ConcurrentSplitVecIntoIter<T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    slices: IntoIterPtrOfConSlices<T, G>,
    len_of_remaining_slices: usize,
    current_ptr: *const T,
    current_last: *const T,
}

impl<'a, T, G> Default for ConcurrentSplitVecIntoIter<T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    fn default() -> Self {
        Self {
            slices: Default::default(),
            len_of_remaining_slices: Default::default(),
            current_ptr: Default::default(),
            current_last: Default::default(),
        }
    }
}

impl<'a, T, G> ConcurrentSplitVecIntoIter<T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    pub fn new(
        capacity: usize,
        fragments: Vec<UnsafeCell<*mut T>>,
        growth: G,
        range: Range<usize>,
    ) -> Self {
        let len_of_remaining_slices = range.len();
        let slices = IntoIterPtrOfConSlices::new(capacity, fragments, growth, range);
        Self {
            slices,
            len_of_remaining_slices,
            current_ptr: core::ptr::null(),
            current_last: core::ptr::null(),
        }
    }

    fn remaining(&self) -> usize {
        let remaining_current = match self.current_ptr.is_null() {
            true => 0,
            // SAFETY: whenever current_ptr is not null, we know that current_last is also not
            // null which is >= current_ptr.
            false => unsafe { self.current_last.offset_from(self.current_ptr) as usize + 1 },
        };

        self.len_of_remaining_slices + remaining_current
    }

    fn next_ptr(&mut self) -> Option<*mut T> {
        match self.current_ptr {
            ptr if ptr.is_null() => self.next_slice(),
            ptr if ptr == self.current_last => {
                self.current_ptr = core::ptr::null_mut();
                Some(ptr as *mut T)
            }
            ptr => {
                // SAFETY: current_ptr is not the last element, hance current_ptr+1 is in bounds
                self.current_ptr = unsafe { self.current_ptr.add(1) };

                // SAFETY: ptr is valid and its value can be taken.
                // Drop will skip this position which is now uninitialized.
                Some(ptr as *mut T)
            }
        }
    }

    fn next_slice(&mut self) -> Option<*mut T> {
        self.slices.next().and_then(|(ptr, len)| {
            debug_assert!(len > 0);
            self.len_of_remaining_slices -= len;
            // SAFETY: pointers are not null since slice is not empty
            self.current_ptr = ptr;
            self.current_last = unsafe { ptr.add(len - 1) };
            self.next_ptr()
        })
    }
}

impl<T, G> Iterator for ConcurrentSplitVecIntoIter<T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_ptr().map(|ptr| unsafe { ptr.read() })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining();
        (len, Some(len))
    }
}

impl<T, G> ExactSizeIterator for ConcurrentSplitVecIntoIter<T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    fn len(&self) -> usize {
        self.remaining()
    }
}

impl<T, G> Drop for ConcurrentSplitVecIntoIter<T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    fn drop(&mut self) {
        if core::mem::needs_drop::<T>() {
            while let Some(ptr) = self.next_ptr() {
                // SAFETY: ptr is in bounds and have not been dropped yet
                unsafe { ptr.drop_in_place() };
            }
        }
    }
}
