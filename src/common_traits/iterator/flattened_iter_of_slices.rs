use super::{iter_of_slices::SliceBorrowKind, IterOfSlices};
use crate::{
    range_helpers::{range_end, range_start},
    Growth,
};
use core::cmp::min;
use core::{borrow::Borrow, iter::FusedIterator, ops::RangeBounds};
use orx_pinned_vec::PinnedVec;

pub struct FlattenedIterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    remaining: usize,
    outer: IterOfSlices<'a, T, K>,
    inner: K::SliceIter,
}

impl<'a, T: 'a, K> FlattenedIterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    fn empty() -> Self {
        Self {
            outer: IterOfSlices::default(),
            remaining: 0,
            inner: K::SliceIter::default(),
        }
    }

    pub(crate) fn new(
        split_vec: K::SplitVecBorrow<impl Growth>,
        range: impl RangeBounds<usize>,
    ) -> Self {
        let len = split_vec.borrow().len();
        let a = min(len, range_start(&range));
        let b = min(len, range_end(&range, len));
        let remaining = b.saturating_sub(a);

        let mut outer = IterOfSlices::new(split_vec, range);
        let inner = outer.next().map(K::get_slice_iter).unwrap_or_default();

        Self {
            outer,
            remaining,
            inner,
        }
    }

    fn next_slice(&mut self) -> Option<K::SliceElem> {
        match self.outer.next().map(K::get_slice_iter) {
            Some(inner) => {
                self.inner = inner;
                self.next()
            }
            None => None,
        }
    }
}

impl<'a, T: 'a, K> Default for FlattenedIterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T: 'a, K> Iterator for FlattenedIterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    type Item = K::SliceElem;

    fn next(&mut self) -> Option<Self::Item> {
        let next_element = self.inner.next();
        match next_element.is_some() {
            true => {
                self.remaining -= 1;
                next_element
            }
            false => self.next_slice(),
        }
    }
}

impl<'a, T: 'a, K> FusedIterator for FlattenedIterOfSlices<'a, T, K> where K: SliceBorrowKind<'a, T> {}

impl<'a, T: 'a, K> ExactSizeIterator for FlattenedIterOfSlices<'a, T, K>
where
    K: SliceBorrowKind<'a, T>,
{
    fn len(&self) -> usize {
        self.remaining
    }
}
