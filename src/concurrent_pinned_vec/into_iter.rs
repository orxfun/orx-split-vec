use crate::{
    GrowthWithConstantTimeAccess,
    concurrent_pinned_vec::into_iter_ptr_slices::IntoIterPtrOfConSlices,
};
use alloc::vec::Vec;
use core::cell::UnsafeCell;

pub struct ConcurrentSplitVecIntoIter<T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    slices: IntoIterPtrOfConSlices<T, G>,
    len_of_remaining_slices: usize,
    current_ptr: *const T,
    current_last: *const T,
}
