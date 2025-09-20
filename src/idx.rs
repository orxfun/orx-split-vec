use core::cmp::Ordering;

/// Index of an element in a jagged array.
#[derive(Default, PartialEq, Debug, Clone)]
pub struct SplitIndex {
    /// Index of the array containing the element.
    pub f: usize,
    /// Index of the element within the array containing it.
    pub i: usize,
}

impl From<(usize, usize)> for SplitIndex {
    #[inline(always)]
    fn from((f, i): (usize, usize)) -> Self {
        Self::new(f, i)
    }
}

impl From<[usize; 2]> for SplitIndex {
    #[inline(always)]
    fn from([f, i]: [usize; 2]) -> Self {
        Self::new(f, i)
    }
}

impl SplitIndex {
    /// Creates a new jagged index:
    ///
    /// * `f`: index of the array containing the element.
    /// * `i`: index of the element within the array containing it.
    #[inline(always)]
    pub fn new(f: usize, i: usize) -> Self {
        Self { f, i }
    }
}

impl PartialOrd for SplitIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.f.partial_cmp(&other.f) {
            Some(Ordering::Equal) => self.i.partial_cmp(&other.i),
            ord => ord,
        }
    }
}
