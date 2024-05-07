use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Mutable iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter_mut()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterMut<'a, T> {
    iter_outer: std::slice::IterMut<'a, Fragment<T>>,
    iter_inner: std::slice::IterMut<'a, T>,
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(fragments: &'a mut [Fragment<T>]) -> Self {
        let mut iter_outer = fragments.iter_mut();
        let iter_inner = iter_outer
            .next()
            .map(|x| x.iter_mut())
            .unwrap_or([].iter_mut());
        Self {
            iter_outer,
            iter_inner,
        }
    }

    fn next_fragment(&mut self) -> Option<&'a mut T> {
        match self.iter_outer.next() {
            Some(f) => {
                self.iter_inner = f.iter_mut();
                self.next()
            }
            None => None,
        }
    }
}

impl<T> FusedIterator for IterMut<'_, T> {}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_element = self.iter_inner.next();
        if next_element.is_some() {
            next_element
        } else {
            self.next_fragment()
        }
    }
}
