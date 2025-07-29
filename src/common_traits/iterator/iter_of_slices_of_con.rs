use super::iter_of_slices::SliceBorrowKind;
use crate::{
    GrowthWithConstantTimeAccess,
    range_helpers::{range_end, range_start},
};
use core::{
    cell::UnsafeCell, cmp::min, iter::FusedIterator, marker::PhantomData, ops::RangeBounds,
};

pub struct IterOfSlicesOfCon<'a, T, G, K>
where
    G: GrowthWithConstantTimeAccess,
    K: SliceBorrowKind<'a, T>,
{
    fragments: &'a [UnsafeCell<*mut T>],
    growth: G,
    sf: usize,
    si: usize,
    si_end: usize,
    ef: usize,
    ei: usize,
    f: usize,
    phantom: PhantomData<K>,
}

impl<'a, T, G, K> IterOfSlicesOfCon<'a, T, G, K>
where
    G: GrowthWithConstantTimeAccess,
    K: SliceBorrowKind<'a, T>,
{
    fn empty() -> Self {
        Self {
            fragments: &[],
            growth: G::pseudo_default(),
            sf: 0,
            si: 0,
            si_end: 0,
            ef: 0,
            ei: 0,
            f: 1,
            phantom: PhantomData,
        }
    }

    fn single_slice(
        fragments: &'a [UnsafeCell<*mut T>],
        growth: G,
        f: usize,
        begin: usize,
        end: usize,
    ) -> Self {
        Self {
            fragments,
            growth,
            sf: f,
            si: begin,
            si_end: end,
            ef: f,
            ei: 0,
            f,
            phantom: PhantomData,
        }
    }

    pub(crate) fn new(
        capacity: usize,
        fragments: &'a [UnsafeCell<*mut T>],
        growth: G,
        range: impl RangeBounds<usize>,
    ) -> Self {
        let fragment_and_inner_indices = |i| growth.get_fragment_and_inner_indices_unchecked(i);

        let a = range_start(&range);
        let b = min(capacity, range_end(&range, capacity));

        match b.saturating_sub(a) {
            0 => Self::empty(),
            _ => {
                let (sf, si) = fragment_and_inner_indices(a);
                let (ef, ei) = fragment_and_inner_indices(b - 1);

                match sf == ef {
                    true => Self::single_slice(fragments, growth, sf, si, ei + 1),
                    false => {
                        let si_end = growth.fragment_capacity_of(sf);
                        Self {
                            fragments,
                            growth,
                            sf,
                            si,
                            si_end,
                            ef,
                            ei,
                            f: sf,
                            phantom: PhantomData,
                        }
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn remaining_len(&self) -> usize {
        (1 + self.ef).saturating_sub(self.f)
    }

    #[inline(always)]
    fn get_ptr_fi(&self, f: usize, i: usize) -> *mut T {
        let p = unsafe { *self.fragments[f].get() };
        unsafe { p.add(i) }
    }

    #[inline(always)]
    fn capacity_of(&self, f: usize) -> usize {
        self.growth.fragment_capacity_of(f)
    }
}
impl<'a, T, G, K> Default for IterOfSlicesOfCon<'a, T, G, K>
where
    G: GrowthWithConstantTimeAccess,
    K: SliceBorrowKind<'a, T>,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T, G, K> Iterator for IterOfSlicesOfCon<'a, T, G, K>
where
    G: GrowthWithConstantTimeAccess,
    K: SliceBorrowKind<'a, T>,
{
    type Item = K::Slice;

    fn next(&mut self) -> Option<Self::Item> {
        match self.f {
            f if f == self.sf => {
                self.f += 1;
                let len = self.si_end - self.si;
                let p = self.get_ptr_fi(self.sf, self.si);
                Some(K::get_slice_from_mut_ptr(p, len))
            }
            f if f < self.ef => {
                self.f += 1;
                let len = self.capacity_of(f);
                let p = self.get_ptr_fi(f, 0);
                Some(K::get_slice_from_mut_ptr(p, len))
            }
            f if f == self.ef => {
                self.f += 1;
                let len = self.ei + 1;
                let p = self.get_ptr_fi(self.ef, 0);
                Some(K::get_slice_from_mut_ptr(p, len))
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining_len();
        (len, Some(len))
    }
}

impl<'a, T, G, K> FusedIterator for IterOfSlicesOfCon<'a, T, G, K>
where
    G: GrowthWithConstantTimeAccess,
    K: SliceBorrowKind<'a, T>,
{
}

impl<'a, T, G, K> ExactSizeIterator for IterOfSlicesOfCon<'a, T, G, K>
where
    G: GrowthWithConstantTimeAccess,
    K: SliceBorrowKind<'a, T>,
{
    fn len(&self) -> usize {
        self.remaining_len()
    }
}
