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
}
