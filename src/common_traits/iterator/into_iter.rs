use crate::{Fragment, Growth, SplitVec};
use alloc::vec::Vec;
use core::iter::FusedIterator;

impl<T, G: Growth> IntoIterator for SplitVec<T, G> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.fragments)
    }
}

/// An iterator that moves out of a vector.
///
/// This struct is created by the `into_iter` method on `SplitVec` (provided by the `IntoIterator` trait).
pub struct IntoIter<T> {
    outer: alloc::vec::IntoIter<Fragment<T>>,
    inner: alloc::vec::IntoIter<T>,
}

impl<T> IntoIter<T> {
    pub(crate) fn new(fragments: Vec<Fragment<T>>) -> Self {
        let mut outer = fragments.into_iter();
        let inner = outer
            .next()
            .map(|f| f.data.into_iter())
            .unwrap_or(Vec::new().into_iter());

        Self { outer, inner }
    }

    fn next_fragment(&mut self) -> Option<T> {
        match self.outer.next() {
            Some(f) => {
                self.inner = f.data.into_iter();
                self.next()
            }
            None => None,
        }
    }
}

impl<T: Clone> Clone for IntoIter<T> {
    fn clone(&self) -> Self {
        Self {
            outer: self.outer.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_element = self.inner.next();
        if next_element.is_some() {
            next_element
        } else {
            self.next_fragment()
        }
    }
}

impl<T> FusedIterator for IntoIter<T> {}
