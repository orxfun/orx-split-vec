use crate::Growth;

#[derive(Default, Clone)]
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

pub(crate) fn maximum_concurrent_capacity<G: Growth, T>(
    fragments: &[Fragment<T>],
    fragments_capacity: usize,
    growth: &G,
) -> usize {
    assert!(fragments_capacity >= fragments.len());

    match fragments_capacity == fragments.len() {
        true => fragments.iter().map(|x| x.capacity()).sum(),
        false => {
            let mut capacities: Vec<_> = fragments.iter().map(|x| x.capacity()).collect();
            for _ in fragments.len()..fragments_capacity {
                let new_capacity = growth.new_fragment_capacity_from(capacities.iter().copied());
                capacities.push(new_capacity);
            }
            capacities.iter().sum()
        }
    }
}

pub(crate) fn num_fragments_for_capacity<G: Growth, T>(
    fragments: &[Fragment<T>],
    growth: &G,
    required_capacity: usize,
) -> (usize, usize) {
    let current_capacity: usize = fragments.iter().map(|x| x.capacity()).sum();

    match current_capacity >= required_capacity {
        true => (fragments.len(), current_capacity),
        false => {
            let mut num_fragments = fragments.len();
            let mut capacities: Vec<_> = fragments.iter().map(|x| x.capacity()).collect();
            let mut capacity = current_capacity;
            while capacity < required_capacity {
                let new_capacity = growth.new_fragment_capacity_from(capacities.iter().copied());
                capacities.push(new_capacity);
                capacity += new_capacity;
                num_fragments += 1;
            }
            (num_fragments, capacity)
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
