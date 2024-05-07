use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T> {
    iter_outer: std::slice::Iter<'a, Fragment<T>>,
    iter_inner: std::slice::Iter<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(fragments: &'a [Fragment<T>]) -> Self {
        let mut iter_outer = fragments.iter();
        let iter_inner = iter_outer.next().map(|x| x.iter()).unwrap_or([].iter());
        Self {
            iter_outer,
            iter_inner,
        }
    }

    fn next_fragment(&mut self) -> Option<&'a T> {
        match self.iter_outer.next() {
            Some(f) => {
                self.iter_inner = f.iter();
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            iter_outer: self.iter_outer.clone(),
            iter_inner: self.iter_inner.clone(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

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

impl<T> FusedIterator for Iter<'_, T> {}
