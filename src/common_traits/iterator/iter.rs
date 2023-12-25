use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter()` method.
#[derive(Debug)]
pub struct Iter<'a, T> {
    pub(crate) fragments: &'a [Fragment<T>],
    pub(crate) f: usize,
    pub(crate) i: usize,
}
impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(fragments: &'a [Fragment<T>]) -> Self {
        let f = 0;
        let i = 0;
        Self { fragments, f, i }
    }
}
impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            fragments: self.fragments,
            f: self.f,
            i: self.i,
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline(always)]
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
impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.fragments
            .iter()
            .skip(self.f + 1)
            .map(|x| x.len())
            .sum::<usize>()
            + if self.f == self.fragments.len() || self.i == self.fragments[self.f].len() {
                0
            } else {
                self.fragments[self.f].len() - self.i
            }
    }
}
impl<T> FusedIterator for Iter<'_, T> {}
