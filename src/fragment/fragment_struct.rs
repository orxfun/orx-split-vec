use core::{cmp::Ordering, ops::Range};

use alloc::vec::Vec;

const ZST_VEC_CAPACITY: usize = 4;

/// A contiguous fragment of the split vector.
///
/// Suppose a split vector contains 10 integers from 0 to 9.
/// Depending on the growth strategy of the split vector,
/// this data might be stored in 3 contiguous fragments,
/// say [0, 1, 2, 3], [4, 5, 6, 7] and [8, 9].
#[derive(Default)]
pub struct Fragment<T> {
    pub(crate) data: Vec<T>,
    pub(crate) capacity: usize,
}

impl<T> Fragment<T> {
    pub fn capacity_of_vec(vec: &Vec<T>) -> usize {
        match core::mem::size_of::<T>() == 0 {
            true => ZST_VEC_CAPACITY,
            false => vec.capacity(),
        }
    }

    pub fn new(capacity: usize, mut data: Vec<T>) -> Self {
        match core::mem::size_of::<T>() == 0 {
            true => Self { data, capacity },
            false => {
                if data.capacity() < capacity {
                    data.reserve(capacity - data.capacity());
                }
                Self { data, capacity }
            }
        }
    }

    /// Creates a new fragment with the given `capacity` and pushes already the `first_value`.
    pub fn new_with_first_value(capacity: usize, first_value: T) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.push(first_value);
        Self { data, capacity }
    }

    /// Creates a new fragment with the given `capacity`.
    pub fn new_empty(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Creates a new fragment with length and capacity equal to the given `capacity`, where each entry is filled with `f()`.
    pub fn new_filled<F: Fn() -> T>(capacity: usize, f: F) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(f());
        }
        Self { data, capacity }
    }

    /// Returns whether the fragment has room to push a new item or not.
    pub fn has_capacity_for_one(&self) -> bool {
        self.data.len() < self.capacity
    }

    /// Returns the available capacity in the fragment.
    pub fn room(&self) -> usize {
        self.capacity - self.data.len()
    }

    // helpers
    pub(crate) fn fragments_with_default_capacity() -> Vec<Fragment<T>> {
        Vec::new()
    }

    pub(crate) fn into_fragments(self) -> Vec<Fragment<T>> {
        let mut fragments = Self::fragments_with_default_capacity();
        fragments.push(self);
        fragments
    }

    pub(crate) fn fragments_with_capacity(fragments_capacity: usize) -> Vec<Fragment<T>> {
        Vec::with_capacity(fragments_capacity)
    }

    pub(crate) fn into_fragments_with_capacity(
        self,
        fragments_capacity: usize,
    ) -> Vec<Fragment<T>> {
        let mut fragments = Self::fragments_with_capacity(fragments_capacity);
        fragments.push(self);
        fragments
    }

    /// Zeroes out all memory; i.e., positions in `0..fragment.capacity()`, of the fragment.
    #[inline(always)]
    pub(crate) unsafe fn zero(&mut self) {
        let slice =
            unsafe { core::slice::from_raw_parts_mut(self.data.as_mut_ptr(), self.capacity()) };
        slice
            .iter_mut()
            .for_each(|m| *m = unsafe { core::mem::zeroed() });
    }

    // exposed vec methods

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        self.data.binary_search_by(f)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.data.iter()
    }

    #[inline(always)]
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    #[inline(always)]
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.data.get_unchecked(index) }
    }

    // exposed vec mut methods

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { self.data.set_len(new_len) };
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.data.extend_from_slice(slice);
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(index) }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    #[inline(always)]
    pub fn insert(&mut self, index: usize, element: T) {
        self.data.insert(index, element);
    }

    #[inline(always)]
    pub fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }

    #[inline(always)]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b);
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.data.sort_by(compare);
    }
}

pub(crate) unsafe fn set_fragments_len<T>(fragments: &mut [Fragment<T>], len: usize) {
    let mut remaining = len;

    for fragment in fragments {
        let capacity = fragment.capacity();

        match remaining <= capacity {
            true => {
                unsafe { fragment.set_len(remaining) };
                remaining = 0;
            }
            false => {
                unsafe { fragment.set_len(capacity) };
                remaining -= capacity;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroed() {
        let mut fragment: Fragment<i32> = Fragment::new_empty(4);
        unsafe { fragment.zero() };
        unsafe { fragment.set_len(4) };
        let zero: i32 = unsafe { core::mem::zeroed() };
        for i in 0..4 {
            assert_eq!(fragment.get(i), Some(&zero));
        }
    }
}
