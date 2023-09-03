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
    pub(crate) fn index_of(&self, ptr_element: usize) -> Option<usize> {
        let ptr_beg = self.data.as_ptr() as usize;
        if ptr_element < ptr_beg {
            None
        } else {
            let ptr_end = (unsafe { self.data.as_ptr().add(self.data.len() - 1) }) as usize;
            if ptr_element > ptr_end {
                None
            } else {
                let diff = ptr_element - ptr_beg;
                let count = diff / std::mem::size_of::<T>();
                Some(count)
            }
        }
    }
}
