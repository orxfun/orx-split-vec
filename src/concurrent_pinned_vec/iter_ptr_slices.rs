use crate::GrowthWithConstantTimeAccess;
use core::{cell::UnsafeCell, iter::FusedIterator};

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

impl<'a, T, G> IterPtrOfConSlices<'a, T, G>
where
    G: GrowthWithConstantTimeAccess,
{
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
