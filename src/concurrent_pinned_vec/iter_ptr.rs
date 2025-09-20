use crate::{
    GrowthWithConstantTimeAccess, concurrent_pinned_vec::iter_ptr_slices::IterPtrOfConSlices,
};

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
        match self.slices.next() {
            Some((ptr, len)) => {
                debug_assert!(len > 0);
                self.len_of_remaining_slices -= len;
                // SAFETY: pointers are not null since slice is not empty
                self.current_ptr = ptr;
                self.current_last = unsafe { ptr.add(len - 1) };
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, T, G> Iterator for IterPtrOfCon<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_ptr.is_null() {
            false => {
                let is_last_of_slice = self.current_ptr == self.current_last;

                let ptr = self.current_ptr as *mut T;

                self.current_ptr = match is_last_of_slice {
                    // SAFETY: current_ptr is not the last element, hance current_ptr+1 is in bounds
                    false => unsafe { self.current_ptr.add(1) },
                    true => core::ptr::null_mut(),
                };

                // SAFETY: ptr is valid and its value can be taken.
                // Drop will skip this position which is now uninitialized.
                Some(ptr)
            }
            true => self.next_slice(),
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
