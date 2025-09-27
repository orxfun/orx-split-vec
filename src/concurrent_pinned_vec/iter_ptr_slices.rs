use crate::{
    GrowthWithConstantTimeAccess,
    range_helpers::{range_end, range_start},
};
use core::cmp::min;
use core::{cell::UnsafeCell, iter::FusedIterator, ops::Range};

pub struct IterPtrOfConSlices<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    fragments: &'a [UnsafeCell<*mut T>],
    growth: G,
    sf: usize,
    si: usize,
    si_end: usize,
    ef: usize,
    ei: usize,
    f: usize,
}

impl<'a, T, G> Default for IterPtrOfConSlices<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T, G> IterPtrOfConSlices<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
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
        }
    }

    pub fn new(
        capacity: usize,
        fragments: &'a [UnsafeCell<*mut T>],
        growth: G,
        range: Range<usize>,
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

impl<'a, T, G> Iterator for IterPtrOfConSlices<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    type Item = (*mut T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.f {
            f if f == self.sf => {
                self.f += 1;
                let len = self.si_end - self.si;
                let p = self.get_ptr_fi(self.sf, self.si);
                Some((p, len))
            }
            f if f < self.ef => {
                self.f += 1;
                let len = self.capacity_of(f);
                let p = self.get_ptr_fi(f, 0);
                Some((p, len))
            }
            f if f == self.ef => {
                self.f += 1;
                let len = self.ei + 1;
                let p = self.get_ptr_fi(self.ef, 0);
                Some((p, len))
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining_len();
        (len, Some(len))
    }
}

impl<'a, T, G> FusedIterator for IterPtrOfConSlices<'a, T, G> where G: GrowthWithConstantTimeAccess {}

impl<'a, T, G> ExactSizeIterator for IterPtrOfConSlices<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
    fn len(&self) -> usize {
        self.remaining_len()
    }
}
