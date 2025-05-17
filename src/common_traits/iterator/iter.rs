use super::reductions;
use crate::{Growth, SplitVec, fragment::fragment_struct::Fragment};
use core::iter::FusedIterator;

impl<'a, T, G: Growth> IntoIterator for &'a SplitVec<T, G> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(&self.fragments)
    }
}

/// Iterator over the `SplitVec`.
///
/// This struct is created by `SplitVec::iter()` method.
#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T> {
    outer: core::slice::Iter<'a, Fragment<T>>,
    inner: core::slice::Iter<'a, T>,
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self {
            outer: Default::default(),
            inner: Default::default(),
        }
    }
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

    #[inline(always)]
    fn remaining_len(&self) -> usize {
        self.inner.len() + self.outer.clone().map(|x| x.len()).sum::<usize>()
    }
}

impl<T> Clone for Iter<'_, T> {
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
        match next_element.is_some() {
            true => next_element,
            false => self.next_fragment(),
        }
    }

    // override default implementations

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

    #[inline(always)]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.remaining_len()
    }

    fn fold<B, F>(mut self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        reductions::fold(&mut self.outer, &mut self.inner, init, f)
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        match self.outer.last() {
            Some(x) => x.last(),
            _ => self.inner.last(),
        }
    }

    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let a = self.inner.max();
        let b = self.outer.filter_map(|x| x.iter().max()).max();
        inner_outer_reduce(a, b, core::cmp::max)
    }

    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let a = self.inner.min();
        let b = self.outer.filter_map(|x| x.iter().min()).min();
        inner_outer_reduce(a, b, core::cmp::min)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            match self.outer.next() {
                Some(fragment) => self.inner = fragment.iter(),
                None => return None,
            }
        }

        let mut inner_len = self.inner.len();
        let mut n = n;
        while n >= inner_len {
            n -= inner_len;
            match self.outer.next() {
                Some(fragment) => {
                    self.inner = fragment.iter();
                    inner_len = fragment.len();
                }
                None => return None,
            }
        }

        self.inner.nth(n)
    }

    fn reduce<F>(mut self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        reductions::reduce(&mut self.outer, &mut self.inner, f)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining_len();
        (len, Some(len))
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

impl<T> ExactSizeIterator for Iter<'_, T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.remaining_len()
    }
}

// helper functions

fn inner_outer_reduce<'a, T, F>(a: Option<&'a T>, b: Option<&'a T>, compare: F) -> Option<&'a T>
where
    F: Fn(&'a T, &'a T) -> &'a T,
{
    match (a, b) {
        (Some(a), Some(b)) => Some(compare(a, b)),
        (Some(a), None) => Some(a),
        _ => b,
    }
}
