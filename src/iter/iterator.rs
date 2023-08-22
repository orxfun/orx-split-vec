use crate::{fragment::fragment_struct::Fragment, SplitVec, SplitVecGrowth};

/// Iterator over the `SplitVec`.
#[derive(Debug)]
pub struct SplitVecIterator<'a, T> {
    pub(crate) fragments: &'a Vec<Fragment<T>>,
    pub(crate) f: usize,
    pub(crate) i: usize,
}
impl<'a, T> Clone for SplitVecIterator<'a, T> {
    fn clone(&self) -> Self {
        Self {
            fragments: self.fragments,
            f: self.f,
            i: self.i,
        }
    }
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

impl<'a, T, G> From<&'a SplitVec<T, G>> for SplitVecIterator<'a, T>
where
    G: SplitVecGrowth<T>,
{
    fn from(value: &'a SplitVec<T, G>) -> Self {
        Self {
            fragments: &value.fragments,
            f: 0,
            i: 0,
        }
    }
}
