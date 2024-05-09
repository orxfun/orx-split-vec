use super::reductions;
use crate::fragment::fragment_struct::Fragment;
use std::iter::FusedIterator;

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T> {
    outer: std::slice::Iter<'a, Fragment<T>>,
    inner: std::slice::Iter<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(fragments: &'a [Fragment<T>]) -> Self {
        let mut outer = fragments.iter();
        let inner = outer.next().map(|x| x.iter()).unwrap_or([].iter());
        Self { outer, inner }
    }

    fn next_fragment(&mut self) -> Option<&'a T> {
        match self.outer.next() {
            Some(f) => {
                self.inner = f.iter();
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            outer: self.outer.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_element = self.inner.next();
        if next_element.is_some() {
            next_element
        } else {
            self.next_fragment()
        }
    }

    // reductions
    fn all<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        reductions::all(&mut self.outer, &mut self.inner, f)
    }

    fn any<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        reductions::any(&mut self.outer, &mut self.inner, f)
    }

    fn fold<B, F>(mut self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        reductions::fold(&mut self.outer, &mut self.inner, init, f)
    }
}

impl<T> FusedIterator for Iter<'_, T> {}
