#[derive(Default)]
/// A contagious fragment of the split vector.
///
/// Suppose a split vector contains 10 integers from 0 to 9.
/// Depending on the growth strategy of the split vector,
/// this data might be stored in 3 contagious fragments,
/// say [0, 1, 2, 3], [4, 5, 6, 7] and [8, 9].
pub struct Fragment<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Fragment<T> {
    /// Creates a new fragment with the given `capacity` and pushes already the `first_value`.
    pub fn new_with_first_value(capacity: usize, first_value: T) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.push(first_value);
        Self { data }
    }

    /// Creates a new fragment with the given `capacity`.
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new fragment with length and capacity equal to the given `capacity`, where each entry is filled with `f()`.
    pub fn new_filled<F: Fn() -> T>(capacity: usize, f: F) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(f());
        }
        Self { data }
    }

    /// Returns whether the fragment has room to push a new item or not.
    pub fn has_capacity_for_one(&self) -> bool {
        self.data.len() < self.data.capacity()
    }

    /// Returns the available capacity in the fragment.
    pub fn room(&self) -> usize {
        self.data.capacity() - self.data.len()
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
        let slice = std::slice::from_raw_parts_mut(self.data.as_mut_ptr(), self.capacity());
        slice.iter_mut().for_each(|m| *m = std::mem::zeroed());
    }
}

pub(crate) unsafe fn set_fragments_len<T>(fragments: &mut [Fragment<T>], len: usize) {
    let mut remaining = len;

    for fragment in fragments {
        let capacity = fragment.capacity();

        match remaining <= capacity {
            true => {
                fragment.set_len(remaining);
                remaining = 0;
            }
            false => {
                fragment.set_len(capacity);
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
        let mut fragment: Fragment<i32> = Fragment::new(4);
        unsafe { fragment.zero() };
        unsafe { fragment.set_len(4) };
        let zero: i32 = unsafe { std::mem::zeroed() };
        for i in 0..4 {
            assert_eq!(fragment.get(i), Some(&zero));
        }
    }
}
