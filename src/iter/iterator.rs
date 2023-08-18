use crate::{fragment::fragment_struct::Fragment, SplitVec};

/// Iterator over the `SplitVec`.
pub struct SplitVecIterator<'a, T> {
    fragments: &'a Vec<Fragment<T>>,
    f: usize,
    i: usize,
}

impl<'a, T> Iterator for SplitVecIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.fragments[self.f].len() {
            self.f += 1;
            self.i = 1;
            self.fragments.get(self.f).and_then(|f| f.get(0))
        } else {
            self.i += 1;
            self.fragments[self.f].get(self.i - 1)
        }
    }
}

impl<'a, T> From<&'a SplitVec<T>> for SplitVecIterator<'a, T> {
    fn from(value: &'a SplitVec<T>) -> Self {
        Self {
            fragments: &value.fragments,
            f: 0,
            i: 0,
        }
    }
}
