use crate::{
    range_helpers::{range_end, range_start},
    Fragment, Growth, SplitVec,
};
use core::{iter::FusedIterator, ops::RangeBounds, slice::from_raw_parts_mut};
use orx_pinned_vec::PinnedVec;

pub struct IterMutSlices<'a, T> {
    fragments: &'a mut [Fragment<T>],
    sf: usize,
    si: usize,
    si_end: usize,
    ef: usize,
    ei: usize,
    f: usize,
}

impl<'a, T> Default for IterMutSlices<'a, T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T> IterMutSlices<'a, T> {
    fn empty() -> Self {
        Self {
            fragments: &mut [],
            sf: 0,
            si: 0,
            si_end: 0,
            ef: 0,
            ei: 0,
            f: 1,
        }
    }

    fn single_slice(fragments: &'a mut [Fragment<T>], f: usize, begin: usize, end: usize) -> Self {
        Self {
            fragments,
            sf: f,
            si: begin,
            si_end: end,
            ef: f,
            ei: 0,
            f,
        }
    }

    pub(crate) fn new(
        split_vec: &'a mut SplitVec<T, impl Growth>,
        range: impl RangeBounds<usize>,
    ) -> Self {
        let a = range_start(&range);
        let b = range_end(&range, split_vec.len());

        match b.saturating_sub(a) {
            0 => Self::empty(),
            _ => match split_vec.get_fragment_and_inner_indices(a) {
                None => Self::empty(),
                Some((sf, si)) => match split_vec.get_fragment_and_inner_indices(b - 1) {
                    None => Self::empty(),
                    Some((ef, ei)) => {
                        let fragments = &mut split_vec.fragments;
                        let si_end = fragments[sf].len();
                        match sf == ef {
                            true => Self::single_slice(fragments, sf, si, ei + 1),
                            false => Self {
                                fragments,
                                sf,
                                si,
                                si_end,
                                ef,
                                ei,
                                f: sf,
                            },
                        }
                    }
                },
            },
        }
    }

    #[inline(always)]
    fn remaining_len(&self) -> usize {
        (1 + self.ef).saturating_sub(self.f)
    }

    #[inline(always)]
    unsafe fn get_raw_mut_unchecked_fi(&mut self, f: usize, i: usize) -> *mut T {
        let p = self.fragments[f].as_mut_ptr();
        p.add(i)
    }
}

impl<'a, T> Iterator for IterMutSlices<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        match self.f {
            f if f == self.sf => {
                self.f += 1;
                let p = unsafe { self.get_raw_mut_unchecked_fi(self.sf, self.si) };
                let slice = unsafe { from_raw_parts_mut(p, self.si_end - self.si) };
                Some(slice)
            }
            f if f < self.ef => {
                self.f += 1;
                let slice_len = self.fragments[f].len();
                let p = unsafe { self.get_raw_mut_unchecked_fi(f, 0) };
                let slice = unsafe { from_raw_parts_mut(p, slice_len) };
                Some(slice)
            }
            f if f == self.ef => {
                self.f += 1;
                let slice_len = self.ei + 1;
                let p = unsafe { self.get_raw_mut_unchecked_fi(self.ef, 0) };
                let slice = unsafe { from_raw_parts_mut(p, slice_len) };
                Some(slice)
            }
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining_len();
        (len, Some(len))
    }
}

impl<'a, T> FusedIterator for IterMutSlices<'a, T> {}

impl<'a, T> ExactSizeIterator for IterMutSlices<'a, T> {
    fn len(&self) -> usize {
        self.remaining_len()
    }
}
