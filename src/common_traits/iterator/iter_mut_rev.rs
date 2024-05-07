use crate::fragment::fragment_struct::Fragment;
use std::iter::{FusedIterator, Rev};

/// Mutable iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter_mut_rev()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterMutRev<'a, T> {
    iter_outer: Rev<std::slice::IterMut<'a, Fragment<T>>>,
    iter_inner: Rev<std::slice::IterMut<'a, T>>,
}

impl<'a, T> IterMutRev<'a, T> {
    pub(crate) fn new(fragments: &'a mut [Fragment<T>]) -> Self {
        let mut iter_outer = fragments.iter_mut().rev();
        let iter_inner = iter_outer
            .next()
            .map(|x| x.iter_mut())
            .unwrap_or([].iter_mut())
            .rev();
        Self {
            iter_outer,
            iter_inner,
        }
    }

    fn next_fragment(&mut self) -> Option<&'a mut T> {
        match self.iter_outer.next() {
            Some(f) => {
                self.iter_inner = f.iter_mut().rev();
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, T> Iterator for IterMutRev<'a, T> {
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

impl<T> FusedIterator for IterMutRev<'_, T> {}
