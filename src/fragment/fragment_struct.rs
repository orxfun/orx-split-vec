use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

const ZST_VEC_CAPACITY: usize = 4;

/// A contiguous fragment of the split vector.
///
/// Suppose a split vector contains 10 integers from 0 to 9.
/// Depending on the growth strategy of the split vector,
/// this data might be stored in 3 contiguous fragments,
/// say [0, 1, 2, 3], [4, 5, 6, 7] and [8, 9].
#[derive(Default)]
pub struct Fragment<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> Fragment<T> {
    /// Returns the effective capacity of a vector, normalizing zero-sized types.
    pub fn capacity_of_vec(vec: &Vec<T>) -> usize {
        match core::mem::size_of::<T>() == 0 {
            true => ZST_VEC_CAPACITY,
            false => vec.capacity(),
        }
    }

    /// Creates a fragment from `data` with the target logical `capacity`.
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

    /// Consumes the fragment and returns the inner vector.
    pub fn into_inner(self) -> Vec<T> {
        self.data
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

    /// Returns the number of initialized elements in the fragment.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the fragment contains no elements.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the logical capacity of the fragment.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a shared slice view of initialized elements.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns a raw pointer to the fragment's initialized elements.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Binary-searches initialized elements with a comparator.
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        self.data.binary_search_by(f)
    }

    /// Returns an iterator over initialized elements.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.data.iter()
    }

    /// Returns the last initialized element, if any.
    #[inline(always)]
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns the first initialized element, if any.
    #[inline(always)]
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    /// Returns a shared reference to the element at `index`, if present.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Returns a shared reference to the element at `index` without bounds checks.
    ///
    /// # Safety
    ///
    /// Caller must ensure `index < self.len()`.
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.data.get_unchecked(index) }
    }

    // exposed vec mut methods

    /// Appends an element to the end of initialized region.
    #[inline(always)]
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Sets the initialized length of the fragment.
    ///
    /// # Safety
    ///
    /// Caller must uphold `Vec::set_len` invariants.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { self.data.set_len(new_len) };
    }

    /// # SAFETY
    ///
    /// Obtained reference to the vector can be used to change the length of the vector
    /// by adding or removing elements; however, it must not change the capacity and
    /// underlying allocation of the vector.
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns a mutable raw pointer to initialized elements.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Clones and appends all elements from `slice`.
    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.data.extend_from_slice(slice);
    }

    /// Returns a mutable reference to the element at `index` without bounds checks.
    ///
    /// # Safety
    ///
    /// Caller must ensure `index < self.len()` and aliasing rules are respected.
    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(index) }
    }

    /// Removes and returns the last element, if any.
    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Inserts `element` at `index`, shifting later elements to the right.
    #[inline(always)]
    pub fn insert(&mut self, index: usize, element: T) {
        self.data.insert(index, element);
    }

    /// Removes and returns the element at `index`, shifting later elements left.
    #[inline(always)]
    pub fn remove(&mut self, index: usize) -> T {
        self.data.remove(index)
    }

    /// Removes all initialized elements from the fragment.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Truncates the initialized length to at most `len`.
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }

    /// Swaps two initialized elements.
    #[inline(always)]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b);
    }

    /// Returns a mutable iterator over initialized elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// Sorts initialized elements with the given comparator.
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

impl<T> IntoIterator for Fragment<T> {
    type Item = T;

    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Index<usize> for Fragment<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<Range<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<Range<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeFrom<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeFrom<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeTo<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeTo<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeInclusive<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeInclusive<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeToInclusive<usize>> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeToInclusive<usize>> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<RangeFull> for Fragment<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<RangeFull> for Fragment<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.data[index]
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
