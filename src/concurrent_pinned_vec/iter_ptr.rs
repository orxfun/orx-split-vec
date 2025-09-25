use crate::{
    GrowthWithConstantTimeAccess, concurrent_pinned_vec::iter_ptr_slices::IterPtrOfConSlices,
};
use core::{cell::UnsafeCell, ops::Range};

pub struct IterPtrOfCon<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    slices: IterPtrOfConSlices<'a, T, G>,
    len_of_remaining_slices: usize,
    current_ptr: *const T,
    current_last: *const T,
}

impl<'a, T, G> IterPtrOfCon<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    pub fn new(
        capacity: usize,
        fragments: &'a [UnsafeCell<*mut T>],
        growth: G,
        range: Range<usize>,
    ) -> Self {
        let len_of_remaining_slices = range.len();
        let slices = IterPtrOfConSlices::new(capacity, fragments, growth, range);
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

    fn next_slice(&mut self) -> Option<*mut T> {
        self.slices.next().and_then(|(ptr, len)| {
            debug_assert!(len > 0);
            self.len_of_remaining_slices -= len;
            // SAFETY: pointers are not null since slice is not empty
            self.current_ptr = ptr;
            self.current_last = unsafe { ptr.add(len - 1) };
            self.next()
        })
    }
}

impl<'a, T, G> Iterator for IterPtrOfCon<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining();
        (len, Some(len))
    }
}

impl<'a, T, G> ExactSizeIterator for IterPtrOfCon<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    fn len(&self) -> usize {
        self.remaining()
    }
}
